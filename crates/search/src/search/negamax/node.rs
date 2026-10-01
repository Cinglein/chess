use core::ops::ControlFlow;

use board::{Board, ChessMove, MoveKind};
use eval::{Evaluator, Score};

use super::super::depth::Depth;
use super::super::interrupt::Interrupt;
use super::Negamax;
use super::ordered_moves::OrderedMoves;
use super::regime::Regime;
use super::table::{RootDistance, TableEntry};
use super::window::{Bounds, Window};

#[derive(Clone, Copy)]
pub(crate) struct Node<'position> {
    board: &'position Board,
    depth: Depth,
    distance: RootDistance,
    window: Window,
    hash_move: Option<ChessMove>,
}

impl<'position> Node<'position> {
    pub(crate) const fn new(
        board: &'position Board,
        depth: Depth,
        distance: RootDistance,
        window: Window,
    ) -> Self {
        Node {
            board,
            depth,
            distance,
            window,
            hash_move: None,
        }
    }

    pub(crate) const fn remembering(self, hash_move: Option<ChessMove>) -> Self {
        Node { hash_move, ..self }
    }

    pub(crate) const fn at_depth(self, depth: Depth) -> Self {
        Node { depth, ..self }
    }

    pub(crate) fn search<R: Regime, E: Evaluator, I: Interrupt>(
        self,
        regime: &R,
        negamax: &mut Negamax<'_, '_, '_, E, I>,
    ) -> Score {
        let floor = regime.floor::<E>(self.board);
        if self.window.upper().excludes(floor) {
            return floor;
        }
        let moves = OrderedMoves::from_board(
            self.board,
            negamax.killers.at_ply(self.distance),
            self.hash_move,
        );
        if moves.is_empty() {
            return Negamax::<E, I>::terminal(self.board, self.distance);
        }
        let searched = moves
            .into_iter()
            .filter(|chess_move| regime.considers(*chess_move, self.board))
            .filter_map(|chess_move| {
                self.board
                    .make_move(chess_move)
                    .map(|child| (chess_move, child))
            })
            .try_fold(
                Bounds::new(self.window, floor),
                |bounds, (chess_move, child)| {
                    let window = bounds.child_window();
                    let score = -negamax.score(&child, self.depth, self.distance.deeper(), window);
                    let admitted = bounds.admit(chess_move, score);
                    if admitted.is_break() && !chess_move.captures(self.board.placement()) {
                        negamax.killers.remember(self.distance, chess_move);
                    }
                    admitted
                },
            );
        let conclusion = match searched {
            ControlFlow::Break(bounds) | ControlFlow::Continue(bounds) => bounds.conclude(),
        };
        negamax.table.store(TableEntry::remember(
            self.board.hash(),
            self.depth.incremented(),
            self.distance,
            conclusion,
        ));
        conclusion.score()
    }
}

#[cfg(test)]
mod tests {
    use board::{Board, LongAlgebraic, NodeCount};
    use eval::{PieceSquareTables, Score};

    use super::super::ordered_moves::Killers;
    use super::super::table::{RootDistance, TableEntry, TranspositionTable};
    use super::super::window::{Bound, Window};
    use super::super::{FullWidth, Negamax};
    use super::Node;
    use crate::search::Uninterrupted;
    use crate::search::depth::Depth;

    const KNIGHT_FORK: &str = "q3k3/8/8/1N6/8/8/8/4K3 w - - 0 1";
    const HANGING_QUEEN: &str = "4k3/8/8/3q4/3Q4/8/8/4K3 w - - 0 1";
    const FORKING_CHECK: &str = "b5c7";
    const DEPTH: Depth = Depth::new(3);

    struct Searched {
        nodes: NodeCount,
        root_killers: Killers,
    }

    impl Searched {
        fn from_root(fen: &str, hash_move: Option<&str>, window: Window) -> Searched {
            let board: Board = fen.parse().unwrap();
            let hash_move = hash_move
                .map(|text| text.parse::<LongAlgebraic>().unwrap())
                .and_then(|notation| board.resolve_move(notation));
            let mut store: [TableEntry; 0] = [];
            let mut table = TranspositionTable::new(&mut store);
            let mut negamax =
                Negamax::<PieceSquareTables, Uninterrupted>::new(&mut table, &Uninterrupted);
            Node::new(&board, DEPTH, RootDistance::ROOT, window)
                .remembering(hash_move)
                .search(&FullWidth, &mut negamax);
            Searched {
                nodes: negamax.nodes(),
                root_killers: negamax.killers.at_ply(RootDistance::ROOT),
            }
        }
    }

    #[test]
    fn a_remembered_best_move_is_tried_first_and_saves_nodes() {
        let without = Searched::from_root(KNIGHT_FORK, None, Window::FULL).nodes;
        let with = Searched::from_root(KNIGHT_FORK, Some(FORKING_CHECK), Window::FULL).nodes;
        assert!(with < without, "{with} vs {without}");
    }

    #[test]
    fn a_capture_that_cuts_the_search_saves_nodes_and_is_not_remembered_as_a_killer() {
        let narrow = Window::FULL.below(Bound::new(Score::DRAW));
        let searched = Searched::from_root(HANGING_QUEEN, None, narrow);
        let wide = Searched::from_root(HANGING_QUEEN, None, Window::FULL);
        assert_eq!(searched.root_killers, Killers::NONE);
        assert!(
            searched.nodes < wide.nodes,
            "{} vs {}",
            searched.nodes,
            wide.nodes
        );
    }
}
