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

#[cfg(test)]
mod tests {
    use board::Board;

    use super::{KillerTable, RootDistance};

    #[test]
    fn a_move_remembered_at_a_ply_is_recalled_there_and_nowhere_else() {
        let chess_move = Board::START.legal_moves()[0];
        let ply = RootDistance::ROOT.deeper();
        let mut table = KillerTable::new();
        table.remember_quiet(ply, chess_move, &Board::START);
        assert!(table.at_ply(ply).remembers(chess_move));
        assert!(!table.at_ply(RootDistance::ROOT).remembers(chess_move));
    }
}
