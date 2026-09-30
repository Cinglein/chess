use board::ChessMove;

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

    pub(crate) fn remember(&mut self, distance: RootDistance, chess_move: ChessMove) {
        if let Some(killers) = self.0.get_mut(usize::from(distance.plies())) {
            *killers = killers.remembering(chess_move);
        }
    }
}
