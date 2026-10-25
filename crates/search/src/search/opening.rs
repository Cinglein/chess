use board::Board;
use proptest::prelude::*;

pub(crate) struct Opening;

impl Opening {
    const OPENING_LENGTH: usize = 8;

    pub(crate) fn boards() -> impl Strategy<Value = Board> {
        prop::collection::vec(any::<usize>(), 0..=Self::OPENING_LENGTH).prop_map(Self::indexed)
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
