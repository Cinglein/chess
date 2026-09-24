use core::cmp::Reverse;

use board::{Board, ChessMove, MoveList};

use crate::killers::Killers;
use crate::move_priority::MovePriority;

pub(crate) struct OrderedMoves(MoveList);

impl OrderedMoves {
    pub(crate) fn from_board(
        board: &Board,
        killers: Killers,
        principal: Option<ChessMove>,
    ) -> OrderedMoves {
        let mut moves = board.legal_moves();
        moves.sort_unstable_by_key(|chess_move| {
            Reverse(MovePriority::rank(*chess_move, board, killers, principal))
        });
        OrderedMoves(moves)
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl IntoIterator for OrderedMoves {
    type Item = ChessMove;
    type IntoIter = <MoveList as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
