use board::{Board, MoveKind, NodeCount};
use core::cell::Cell;
use eval::{Evaluator, PieceSquareTables, Score};
use proptest::prelude::*;
use search::{Depth, Interrupt, Search, TableEntry, TranspositionTable};

const MATE_IN_ONE: &str = "6k1/5ppp/8/8/8/8/5PPP/R5K1 w - - 0 1";
const MATE_IN_TWO: &str = "7k/8/8/8/8/8/R7/1R4K1 w - - 0 1";
const STALEMATE: &str = "7k/5Q2/6K1/8/8/8/8/8 b - - 0 1";
const MATES: [(&str, u8, u8); 2] = [(MATE_IN_ONE, 2, 1), (MATE_IN_TWO, 3, 3)];
const MATING_MOVE: &str = "a1a8";
const OPENING_LENGTH: usize = 8;
const TABLE_ENTRIES: usize = 1 << 16;
const TINY_TABLE_ENTRIES: usize = 2;
const TWO_PLIES: Depth = Depth::new(2);
const THREE_PLIES: Depth = Depth::new(3);
const CASES: u32 = 12;

struct Uninterrupted;

impl Interrupt for Uninterrupted {
    fn should_stop(&self, _: NodeCount) -> bool {
        false
    }
}

struct Interrupted;

impl Interrupt for Interrupted {
    fn should_stop(&self, _: NodeCount) -> bool {
        true
    }
}

struct Opening;

impl Opening {
    fn boards() -> impl Strategy<Value = Board> {
        prop::collection::vec(any::<usize>(), 0..=OPENING_LENGTH).prop_map(Self::indexed)
    }

    fn indexed(choices: Vec<usize>) -> Board {
        choices.into_iter().fold(Board::START, |board, index| {
            let moves = board.legal_moves();
            moves
                .get(index % moves.len().max(1))
                .and_then(|chess_move| board.make_move(*chess_move))
                .unwrap_or(board)
        })
    }
}

struct Fixture;

impl Fixture {
    fn searched(
        board: Board,
        depth: Depth,
        table: &mut TranspositionTable<'_>,
    ) -> Search<PieceSquareTables> {
        (0..depth.plies()).fold(Search::from(board), |search, _| {
            search.deepen(table, &Uninterrupted)
        })
    }

    fn from_fen(fen: &str, depth: u8) -> Search<PieceSquareTables> {
        let mut store = vec![TableEntry::EMPTY; TABLE_ENTRIES];
        let mut table = TranspositionTable::new(&mut store);
        Self::searched(fen.parse().unwrap(), Depth::new(depth), &mut table)
    }

    fn without_table(board: Board, depth: Depth) -> Search<PieceSquareTables> {
        Self::searched(board, depth, &mut TranspositionTable::new(&mut []))
    }
}

struct Oracle {
    visited: Cell<u32>,
}

impl Oracle {
    const AFFORDABLE_NODES: u32 = 1 << 16;

    fn new() -> Oracle {
        Oracle {
            visited: Cell::new(0),
        }
    }

    fn leaf(&self, board: &Board, plies: u8) -> Option<Score> {
        self.visited.set(self.visited.get() + 1);
        let moves = board.legal_moves();
        let floor = if board.in_check() {
            Score::INFINITY.negated()
        } else {
            PieceSquareTables::evaluate(board)
        };
        if self.visited.get() > Self::AFFORDABLE_NODES {
            None
        } else if moves.is_empty() && board.in_check() {
            Some(-Score::mate_in(plies))
        } else if moves.is_empty() {
            Some(Score::DRAW)
        } else {
            moves
                .into_iter()
                .filter(|chess_move| board.in_check() || chess_move.captures(board.placement()))
                .filter_map(|chess_move| board.make_move(chess_move))
                .try_fold(floor, |best, after| {
                    Some(best.max(-self.leaf(&after, plies + 1)?))
                })
        }
    }

    fn full_width(&self, board: &Board, depth: Depth, plies: u8) -> Option<Score> {
        let moves = board.legal_moves();
        match depth.decremented() {
            None => self.leaf(board, plies),
            Some(_) if moves.is_empty() && board.in_check() => Some(-Score::mate_in(plies)),
            Some(_) if moves.is_empty() => Some(Score::DRAW),
            Some(remaining) => moves
                .into_iter()
                .filter_map(|chess_move| board.make_move(chess_move))
                .map(|after| {
                    self.full_width(&after, remaining, plies + 1)
                        .map(Score::negated)
                })
                .collect::<Option<Vec<Score>>>()?
                .into_iter()
                .max(),
        }
    }
}

#[test]
fn mates_are_found_at_the_depth_that_reaches_them_and_stalemate_has_no_move_and_a_drawn_score() {
    for (fen, depth, plies) in MATES {
        assert_eq!(
            Fixture::from_fen(fen, depth).score(),
            Score::mate_in(plies),
            "{fen}"
        );
    }
    let best = Fixture::from_fen(MATE_IN_ONE, 2)
        .best_move()
        .map(|chess_move| chess_move.to_string());
    let stalemated = Fixture::from_fen(STALEMATE, 1);
    assert_eq!(
        (best.as_deref(), stalemated.best_move(), stalemated.score()),
        (Some(MATING_MOVE), None, Score::DRAW)
    );
}

#[test]
fn alpha_beta_scores_as_the_full_width_tree_over_a_stand_pat_capture_search_and_its_move_earns_it()
{
    proptest!(ProptestConfig::with_cases(CASES), |(board in Opening::boards(), depth in (1..=2u8).prop_map(Depth::new))| {
        let search = Fixture::without_table(board, depth);
        let expected = Oracle::new().full_width(&board, depth, 0);
        prop_assume!(expected.is_some());
        prop_assert_eq!(Some(search.score()), expected, "{} at {}", board, depth);
        let replied = search.best_move().and_then(|chess_move| board.make_move(chess_move));
        let reply = |after: Board| {
            Oracle::new().full_width(&after, depth.decremented().unwrap(), 1).map(Score::negated)
        };
        prop_assert_eq!(replied.map_or(Some(search.score()), reply), expected, "{} at {}", board, depth);
    });
}

#[test]
fn deepening_through_one_table_scores_as_without_one_fresh_or_warmed_and_the_warmed_table_saves_nodes()
 {
    proptest!(ProptestConfig::with_cases(CASES), |(board in Opening::boards())| {
        prop_assume!(!board.legal_moves().is_empty());
        let without = Fixture::without_table(board, THREE_PLIES);
        let mut store = vec![TableEntry::EMPTY; TABLE_ENTRIES];
        let mut table = TranspositionTable::new(&mut store);
        let fresh = Fixture::searched(board, THREE_PLIES, &mut table);
        let warmed = Fixture::searched(board, THREE_PLIES, &mut table);
        prop_assert_eq!((fresh.score(), warmed.score()), (without.score(), without.score()), "{}", board);
        prop_assert!(warmed.nodes() < fresh.nodes(), "{}: {} vs {}", board, warmed.nodes(), fresh.nodes());
    });
}

#[test]
fn a_table_of_two_slots_changes_no_score() {
    proptest!(ProptestConfig::with_cases(CASES), |(board in Opening::boards())| {
        let mut store = [TableEntry::EMPTY; TINY_TABLE_ENTRIES];
        let mut table = TranspositionTable::new(&mut store);
        let tiny = Fixture::searched(board, TWO_PLIES, &mut table);
        prop_assert_eq!(tiny.score(), Fixture::without_table(board, TWO_PLIES).score(), "{}", board);
    });
}

#[test]
fn a_mate_stored_from_the_root_reads_the_same_when_its_position_is_searched_from_the_root() {
    let board: Board = MATE_IN_TWO.parse().unwrap();
    let mut store = vec![TableEntry::EMPTY; TABLE_ENTRIES];
    let mut table = TranspositionTable::new(&mut store);
    let root = Fixture::searched(board, THREE_PLIES, &mut table);
    let child = root
        .best_move()
        .and_then(|chess_move| board.make_move(chess_move))
        .unwrap();
    assert_eq!(
        Fixture::searched(child, TWO_PLIES, &mut table).score(),
        Fixture::without_table(child, TWO_PLIES).score()
    );
}

#[test]
fn an_interrupted_deepening_keeps_the_search_as_it_was() {
    let mut table = TranspositionTable::new(&mut []);
    let settled = Fixture::searched(Board::START, TWO_PLIES, &mut table);
    let score = settled.score();
    let best = settled.best_move();
    let kept = settled.deepen(&mut table, &Interrupted);
    assert_eq!(
        (kept.depth(), kept.score(), kept.best_move()),
        (TWO_PLIES, score, best)
    );
}
