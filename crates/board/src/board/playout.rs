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

#[cfg(test)]
mod tests {
    use super::{Board, Playout, SplitMix64};
    #[test]
    fn a_playout_leaves_the_start_position_by_following_its_generator() {
        let played = Playout::new(SplitMix64::new(7)).advanced().advanced();
        assert!(played.board() != Board::START && played.board().legal_moves().len() > 1);
    }
}
