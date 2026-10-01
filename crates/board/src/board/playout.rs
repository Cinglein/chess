use super::Board;
use crate::zobrist_keys::SplitMix64;

#[derive(Clone, Copy)]
pub(super) struct Playout {
    board: Board,
    generator: SplitMix64,
}

impl Playout {
    pub(super) const fn new(generator: SplitMix64) -> Playout {
        Playout {
            board: Board::START,
            generator,
        }
    }

    pub(super) const fn board(&self) -> Board {
        self.board
    }

    pub(super) fn advanced(self) -> Playout {
        let generator = self.generator.next();
        let moves = self.board.legal_moves();
        let index =
            usize::try_from(generator.output() % u64::try_from(moves.len().max(1)).unwrap_or(1))
                .unwrap_or(0);
        moves
            .get(index)
            .and_then(|chess_move| self.board.make_move(*chess_move))
            .map_or(self, |board| Playout { board, generator })
    }
}
