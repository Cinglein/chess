use board::{Board, ChessMove, MoveKind, NodeCount, SplitMix64};
use core::cell::{Cell, RefCell};
use core::marker::PhantomData;
use eval::{Evaluator, PieceKindValue, PieceSquareTables, Score};
use proptest::prelude::*;
use search::{Depth, Interrupt, Search, SearchReport, TableEntry, TranspositionTable};

const MATE_IN_ONE: &str = "6k1/5ppp/8/8/8/8/5PPP/R5K1 w - - 0 1";
const MATE_IN_TWO: &str = "7k/8/8/8/8/8/R7/1R4K1 w - - 0 1";
const STALEMATE: &str = "7k/5Q2/6K1/8/8/8/8/8 b - - 0 1";
const MATES: [(&str, u8, u8); 2] = [(MATE_IN_ONE, 2, 1), (MATE_IN_TWO, 3, 3)];
const MATING_MOVE: &str = "a1a8";
const LOCKED: &str = "4k3/8/8/p6p/P6P/8/8/4K3 w - - 0 1";
const CONTACT: &str = "4k3/8/8/3p4/4P3/8/8/4K3 w - - 0 1";
const POCKETED_KINGS: &str = "6bk/5ppp/5P2/8/8/2p5/PPP5/KB6 w - - 0 1";
const TACTICAL: &str = "r3k3/1P4P1/8/3q4/2P1N3/3R4/8/4K2n w - - 0 1";
const OPENING_LENGTH: usize = 8;
const TABLE_ENTRIES: usize = 1 << 16;
const TINY_TABLE_ENTRIES: usize = 2;
const ONE_PLY: Depth = Depth::new(1);
const TWO_PLIES: Depth = Depth::new(2);
const THREE_PLIES: Depth = Depth::new(3);
const CASES: u32 = 12;
const SALTED_CASES: u32 = 64;
const KILLER_SLOTS: usize = 2;

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

struct Fixture<E>(PhantomData<E>);

type Positional = Fixture<PieceSquareTables>;

impl<E: Evaluator> Fixture<E> {
    fn searched(board: Board, depth: Depth, table: &mut TranspositionTable<'_>) -> Search<E> {
        (0..depth.plies()).fold(Search::from(board), |search, _| {
            search.deepen(table, &Uninterrupted)
        })
    }

    fn from_fen(fen: &str, depth: u8) -> Search<E> {
        let mut store = vec![TableEntry::EMPTY; TABLE_ENTRIES];
        let mut table = TranspositionTable::new(&mut store);
        Self::searched(fen.parse().unwrap(), Depth::new(depth), &mut table)
    }

    fn without_table(board: Board, depth: Depth) -> Search<E> {
        Self::searched(board, depth, &mut TranspositionTable::new(&mut []))
    }
}

thread_local! {
    static SALT: Cell<u64> = const { Cell::new(0) };
    static EVALUATED: RefCell<Vec<Board>> = const { RefCell::new(Vec::new()) };
}

struct Salted;

impl Evaluator for Salted {
    fn evaluate(board: &Board) -> Score {
        EVALUATED.with_borrow_mut(|evaluated| evaluated.push(*board));
        Self::score(board)
    }
}

impl Salted {
    const SPREAD: u64 = 8;
    const OFFSETS: u64 = 17;

    fn score(board: &Board) -> Score {
        let mixed = SplitMix64::new(board.hash().bits() ^ SALT.get())
            .next()
            .output();
        let offset = SALT.get() % Self::OFFSETS;
        Score::new(i32::try_from(mixed % Self::SPREAD + offset).unwrap() - 12)
    }

    fn evaluations_during<T>(run: impl FnOnce() -> T) -> Evaluations<T> {
        EVALUATED.with_borrow_mut(Vec::clear);
        let outcome = run();
        Evaluations {
            outcome,
            boards: EVALUATED.take(),
        }
    }
}

struct Evaluations<T> {
    outcome: T,
    boards: Vec<Board>,
}

struct Traced;

impl Evaluator for Traced {
    fn evaluate(board: &Board) -> Score {
        EVALUATED.with_borrow_mut(|evaluated| evaluated.push(*board));
        PieceSquareTables::evaluate(board)
    }
}

#[derive(Clone, Copy)]
struct Branch {
    chess_move: ChessMove,
    child: Board,
}

struct Tree;

impl Tree {
    fn branches(board: &Board) -> impl Iterator<Item = Branch> + use<> {
        let position = *board;
        position
            .legal_moves()
            .into_iter()
            .filter_map(move |chess_move| {
                position
                    .make_move(chess_move)
                    .map(|child| Branch { chess_move, child })
            })
    }

    fn is_child(parent: &Board, board: &Board) -> bool {
        Self::branches(parent).any(|branch| branch.child == *board)
    }

    fn rich_boards() -> impl Strategy<Value = Board> {
        prop_oneof![
            any::<Board>(),
            prop::sample::select(&[CONTACT, TACTICAL]).prop_map(|fen| fen.parse().unwrap()),
        ]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Rank {
    Quiet,
    Capture { victim: Score, attacker: Score },
    Promotion,
}

impl Rank {
    fn of(chess_move: ChessMove, board: &Board) -> Rank {
        let placement = board.placement();
        let capture = chess_move
            .victim(placement)
            .zip(placement.piece_at(chess_move.origin()))
            .map_or(Rank::Quiet, |(victim, attacker)| Rank::Capture {
                victim: PieceKindValue::material(victim.kind()),
                attacker: -PieceKindValue::material(attacker.kind()),
            });
        chess_move
            .promotion_piece()
            .map_or(capture, |_| Rank::Promotion)
    }
}

#[derive(Clone, Copy)]
struct Visit {
    root_move: ChessMove,
    reply: ChessMove,
    score: Score,
}

impl Visit {
    fn two_plies_below(root: &Board, evaluated: &[Board]) -> Vec<Visit> {
        let leaves: Vec<(Board, Visit)> = Tree::branches(root)
            .flat_map(|branch| {
                Tree::branches(&branch.child).map(move |reply| {
                    let visit = Visit {
                        root_move: branch.chess_move,
                        reply: reply.chess_move,
                        score: Salted::score(&reply.child),
                    };
                    (reply.child, visit)
                })
            })
            .collect();
        evaluated
            .iter()
            .filter_map(|board| {
                leaves
                    .iter()
                    .find(|(leaf, _)| leaf == board)
                    .map(|(_, visit)| *visit)
            })
            .collect()
    }
}

struct Oracle<E> {
    visited: Cell<u32>,
    evaluator: PhantomData<E>,
}

impl<E: Evaluator> Oracle<E> {
    const AFFORDABLE_NODES: u32 = 1 << 16;

    fn new() -> Oracle<E> {
        Oracle {
            visited: Cell::new(0),
            evaluator: PhantomData,
        }
    }

    fn leaf(&self, board: &Board, plies: u8) -> Option<Score> {
        self.visited.set(self.visited.get() + 1);
        let moves = board.legal_moves();
        let floor = if board.in_check() {
            Score::INFINITY.negated()
        } else {
            E::evaluate(board)
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
            Positional::from_fen(fen, depth).score(),
            Score::mate_in(plies),
            "{fen}"
        );
    }
    let best = Positional::from_fen(MATE_IN_ONE, 2)
        .best_move()
        .map(|chess_move| chess_move.to_string());
    let stalemated = Positional::from_fen(STALEMATE, 1);
    assert_eq!(
        (best.as_deref(), stalemated.best_move(), stalemated.score()),
        (Some(MATING_MOVE), None, Score::DRAW)
    );
}

#[test]
fn alpha_beta_scores_as_the_full_width_tree_over_a_stand_pat_capture_search_and_its_move_earns_it()
{
    proptest!(ProptestConfig::with_cases(CASES), |(board in Opening::boards(), depth in (1..=2u8).prop_map(Depth::new))| {
        let search = Positional::without_table(board, depth);
        let expected = Oracle::<PieceSquareTables>::new().full_width(&board, depth, 0);
        prop_assume!(expected.is_some());
        prop_assert_eq!(Some(search.score()), expected, "{} at {}", board, depth);
        let replied = search.best_move().and_then(|chess_move| board.make_move(chess_move));
        let reply = |after: Board| {
            Oracle::<PieceSquareTables>::new().full_width(&after, depth.decremented().unwrap(), 1).map(Score::negated)
        };
        prop_assert_eq!(replied.map_or(Some(search.score()), reply), expected, "{} at {}", board, depth);
    });
}

#[test]
fn deepening_through_one_table_scores_as_without_one_fresh_or_warmed_and_the_warmed_table_saves_nodes()
 {
    proptest!(ProptestConfig::with_cases(CASES), |(board in Opening::boards())| {
        prop_assume!(!board.legal_moves().is_empty());
        let without = Positional::without_table(board, THREE_PLIES);
        let mut store = vec![TableEntry::EMPTY; TABLE_ENTRIES];
        let mut table = TranspositionTable::new(&mut store);
        let fresh = Positional::searched(board, THREE_PLIES, &mut table);
        let warmed = Positional::searched(board, THREE_PLIES, &mut table);
        prop_assert_eq!((fresh.score(), warmed.score()), (without.score(), without.score()), "{}", board);
        prop_assert!(warmed.nodes() < fresh.nodes(), "{}: {} vs {}", board, warmed.nodes(), fresh.nodes());
    });
}

#[test]
fn a_table_of_two_slots_changes_no_score() {
    proptest!(ProptestConfig::with_cases(CASES), |(board in Opening::boards())| {
        let mut store = [TableEntry::EMPTY; TINY_TABLE_ENTRIES];
        let mut table = TranspositionTable::new(&mut store);
        let tiny = Positional::searched(board, TWO_PLIES, &mut table);
        prop_assert_eq!(tiny.score(), Positional::without_table(board, TWO_PLIES).score(), "{}", board);
    });
}

#[test]
fn a_mate_stored_from_the_root_reads_the_same_when_its_position_is_searched_from_the_root() {
    let board: Board = MATE_IN_TWO.parse().unwrap();
    let mut store = vec![TableEntry::EMPTY; TABLE_ENTRIES];
    let mut table = TranspositionTable::new(&mut store);
    let root = Positional::searched(board, THREE_PLIES, &mut table);
    let child = root
        .best_move()
        .and_then(|chess_move| board.make_move(chess_move))
        .unwrap();
    assert_eq!(
        Positional::searched(child, TWO_PLIES, &mut table).score(),
        Positional::without_table(child, TWO_PLIES).score()
    );
}

#[test]
fn an_interrupted_deepening_keeps_the_search_as_it_was() {
    let mut table = TranspositionTable::new(&mut []);
    let settled = Positional::searched(Board::START, TWO_PLIES, &mut table);
    let score = settled.score();
    let best = settled.best_move();
    let kept = settled.deepen(&mut table, &Interrupted);
    assert_eq!(
        (kept.depth(), kept.score(), kept.best_move()),
        (TWO_PLIES, score, best)
    );
}

#[test]
fn a_mate_read_back_from_the_table_keeps_its_distance_from_the_root() {
    for (fen, depth, plies) in MATES {
        let board: Board = fen.parse().unwrap();
        let mut store = vec![TableEntry::EMPTY; TABLE_ENTRIES];
        let mut table = TranspositionTable::new(&mut store);
        let fresh = Positional::searched(board, Depth::new(depth), &mut table);
        let warmed = Positional::searched(board, Depth::new(depth), &mut table);
        assert_eq!(
            (fresh.score(), warmed.score()),
            (Score::mate_in(plies), Score::mate_in(plies)),
            "{fen}"
        );
    }
}

#[test]
fn any_evaluator_scores_as_the_full_width_tree_with_no_table_a_fresh_one_or_a_warmed_one() {
    proptest!(ProptestConfig::with_cases(SALTED_CASES), |(
        salt: u64,
        fen in prop::sample::select(&[LOCKED, POCKETED_KINGS]),
        depth in (1..=3u8).prop_map(Depth::new),
    )| {
        SALT.set(salt);
        let board: Board = fen.parse().unwrap();
        let expected = Oracle::<Salted>::new().full_width(&board, depth, 0);
        let mut store = vec![TableEntry::EMPTY; TABLE_ENTRIES];
        let mut table = TranspositionTable::new(&mut store);
        let fresh = Fixture::<Salted>::searched(board, depth, &mut table);
        let warmed = Fixture::<Salted>::searched(board, depth, &mut table);
        let bare = Fixture::<Salted>::without_table(board, depth);
        prop_assert_eq!(Some(bare.score()), expected, "{} at {}", fen, depth);
        prop_assert_eq!(Some(fresh.score()), expected, "{} at {}", fen, depth);
        prop_assert_eq!(Some(warmed.score()), expected, "{} at {}", fen, depth);
    });
}

#[test]
fn a_warmed_table_answers_for_each_child_as_the_full_width_tree_does() {
    proptest!(ProptestConfig::with_cases(SALTED_CASES), |(
        salt: u64,
        fen in prop::sample::select(&[LOCKED, POCKETED_KINGS]),
    )| {
        SALT.set(salt);
        let board: Board = fen.parse().unwrap();
        let mut store = vec![TableEntry::EMPTY; TABLE_ENTRIES];
        let mut table = TranspositionTable::new(&mut store);
        let _ = Fixture::<Salted>::searched(board, THREE_PLIES, &mut table);
        for branch in Tree::branches(&board) {
            let expected = Oracle::<Salted>::new().full_width(&branch.child, TWO_PLIES, 0);
            let answered = Fixture::<Salted>::searched(branch.child, TWO_PLIES, &mut table);
            prop_assert_eq!(Some(answered.score()), expected);
        }
    });
}

#[test]
fn a_contested_slot_goes_to_the_newest_position_however_deep_the_old_one_was() {
    let mut store = [TableEntry::EMPTY; 1];
    let _ = Positional::searched(
        LOCKED.parse().unwrap(),
        THREE_PLIES,
        &mut TranspositionTable::new(&mut store),
    );
    let deep = store;
    let _ = Positional::searched(
        CONTACT.parse().unwrap(),
        ONE_PLY,
        &mut TranspositionTable::new(&mut store),
    );
    assert_ne!(store, deep);
}

#[test]
fn the_root_keeps_the_first_of_equally_good_moves() {
    let board: Board = LOCKED.parse().unwrap();
    proptest!(ProptestConfig::with_cases(SALTED_CASES), |(salt: u64)| {
        SALT.set(salt);
        let Evaluations { outcome: search, boards } =
            Salted::evaluations_during(|| Fixture::<Salted>::without_table(board, ONE_PLY));
        let best = boards
            .iter()
            .filter_map(|leaf| Tree::branches(&board).find(|branch| branch.child == *leaf))
            .min_by_key(|branch| Salted::score(&branch.child));
        prop_assert_eq!(search.best_move(), best.map(|branch| branch.chess_move));
    });
}

#[test]
fn the_root_tries_promotions_then_captures_by_gain_then_quiet_moves() {
    proptest!(ProptestConfig::with_cases(SALTED_CASES), |(board in Tree::rich_boards())| {
        let evaluated =
            Salted::evaluations_during(|| Fixture::<Traced>::without_table(board, ONE_PLY));
        let ranks: Vec<Rank> = evaluated
            .boards
            .iter()
            .filter_map(|leaf| Tree::branches(&board).find(|branch| branch.child == *leaf))
            .map(|branch| Rank::of(branch.chess_move, &board))
            .collect();
        prop_assert!(ranks.is_sorted_by(|earlier, later| earlier >= later), "{}", board);
    });
}

#[test]
fn a_deepening_starts_from_the_move_the_last_one_chose() {
    let board: Board = LOCKED.parse().unwrap();
    proptest!(ProptestConfig::with_cases(SALTED_CASES), |(salt: u64)| {
        SALT.set(salt);
        let mut table = TranspositionTable::new(&mut []);
        let shallow = Fixture::<Salted>::without_table(board, ONE_PLY);
        let principal = board.make_move(shallow.best_move().unwrap()).unwrap();
        let evaluated =
            Salted::evaluations_during(|| shallow.deepen(&mut table, &Uninterrupted)).boards;
        prop_assert!(Tree::is_child(&principal, &evaluated[0]));
    });
}

#[test]
fn a_sibling_stops_at_its_refutation_and_tries_the_last_two_refutations_before_other_quiet_moves() {
    let board: Board = LOCKED.parse().unwrap();
    let replies = Tree::branches(&Tree::branches(&board).next().unwrap().child).count();
    proptest!(ProptestConfig::with_cases(SALTED_CASES), |(salt: u64)| {
        SALT.set(salt);
        let mut table = TranspositionTable::new(&mut []);
        let shallow = Fixture::<Salted>::without_table(board, ONE_PLY);
        let evaluated =
            Salted::evaluations_during(|| shallow.deepen(&mut table, &Uninterrupted)).boards;
        let visits = Visit::two_plies_below(&board, &evaluated);
        let mut best = Score::INFINITY.negated();
        let mut killers: Vec<ChessMove> = Vec::new();
        for sibling in visits.chunk_by(|earlier, later| earlier.root_move == later.root_move) {
            let refuted = sibling.iter().position(|visit| visit.score <= best);
            let visited = refuted.map_or(replies, |index| index + 1);
            prop_assert_eq!(sibling.len(), visited);
            let remembered = sibling.iter().map(|visit| killers.contains(&visit.reply));
            prop_assert!(remembered.is_sorted_by(|earlier, later| earlier >= later));
            match refuted.and_then(|index| sibling.get(index)) {
                Some(refutation) if killers.first() != Some(&refutation.reply) => {
                    killers.insert(0, refutation.reply);
                    killers.truncate(KILLER_SLOTS);
                }
                Some(_) => {}
                None => {
                    best = sibling.iter().map(|visit| visit.score).min().unwrap().max(best);
                }
            }
        }
    });
}

#[test]
fn a_position_met_again_tries_the_first_reply_that_earned_its_score_before_any_other() {
    let board: Board = LOCKED.parse().unwrap();
    proptest!(ProptestConfig::with_cases(SALTED_CASES), |(salt: u64)| {
        SALT.set(salt);
        let mut store = vec![TableEntry::EMPTY; TABLE_ENTRIES];
        let mut table = TranspositionTable::new(&mut store);
        let Evaluations { outcome: shallow, boards: evaluated } = Salted::evaluations_during(|| {
            Fixture::<Salted>::searched(board, TWO_PLIES, &mut table)
        });
        let principal = shallow.best_move().unwrap();
        let reply = Visit::two_plies_below(&board, &evaluated)
            .into_iter()
            .filter(|visit| visit.root_move == principal)
            .min_by_key(|visit| visit.score)
            .unwrap()
            .reply;
        let remembered = board
            .make_move(principal)
            .and_then(|child| child.make_move(reply))
            .unwrap();
        let deeper =
            Salted::evaluations_during(|| shallow.deepen(&mut table, &Uninterrupted)).boards;
        prop_assert!(Tree::is_child(&remembered, &deeper[0]));
    });
}

#[test]
fn repeating_a_deepening_against_its_own_table_visits_each_root_move_once_and_scores_the_same() {
    let board: Board = LOCKED.parse().unwrap();
    let root_moves = Tree::branches(&board).count();
    proptest!(ProptestConfig::with_cases(SALTED_CASES), |(salt: u64)| {
        SALT.set(salt);
        let mut store = vec![TableEntry::EMPTY; TABLE_ENTRIES];
        let mut table = TranspositionTable::new(&mut store);
        let mut spare_store = vec![TableEntry::EMPTY; TABLE_ENTRIES];
        let mut spare = TranspositionTable::new(&mut spare_store);
        let deepest = Fixture::<Salted>::searched(board, THREE_PLIES, &mut table);
        let shallower = Fixture::<Salted>::searched(board, TWO_PLIES, &mut spare);
        let before = shallower.nodes().count();
        let repeated = shallower.deepen(&mut table, &Uninterrupted);
        prop_assert_eq!(repeated.nodes().count() - before, u64::try_from(root_moves).unwrap());
        prop_assert_eq!(repeated.score(), deepest.score());
    });
}
