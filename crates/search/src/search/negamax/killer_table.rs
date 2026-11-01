use board::{Board, ChessMove, MoveKind};

use super::ordered_moves::Killers;
use super::table::RootDistance;

#[derive(Clone, Copy, Debug)]
pub(crate) struct KillerTable([Killers; KillerTable::PLY_CAPACITY]);

impl KillerTable {
    pub(crate) const PLY_CAPACITY: usize = 128;

    pub(crate) const fn new() -> KillerTable {
        KillerTable([Killers::NONE; Self::PLY_CAPACITY])
    }

    pub(crate) fn at_ply(&self, distance: RootDistance) -> Killers {
        self.0
            .get(usize::from(distance.plies()))
            .copied()
            .unwrap_or(Killers::NONE)
    }

    pub(crate) fn remember_quiet(
        &mut self,
        distance: RootDistance,
        chess_move: ChessMove,
        board: &Board,
    ) {
        self.0
            .get_mut(usize::from(distance.plies()))
            .filter(|_| !chess_move.captures(board.placement()))
            .into_iter()
            .for_each(|killers| *killers = killers.remembering(chess_move));
    }
}
