use board::{Board, PlyCount};
use rand::rngs::StdRng;
use rand::{RngExt, SeedableRng};

use crate::seed::Seed;

pub struct RandomOpenings {
    generator: StdRng,
    plies: PlyCount,
}

impl RandomOpenings {
    #[must_use]
    pub fn seeded(seed: Seed, plies: PlyCount) -> RandomOpenings {
        RandomOpenings {
            generator: StdRng::seed_from_u64(u64::from(seed)),
            plies,
        }
    }

    fn random_move(&mut self, board: Board) -> Option<Board> {
        let moves = board.legal_moves();
        let chosen = *moves.get(self.generator.random_range(0..moves.len().max(1)))?;
        board.make_move(chosen)
    }
}

impl Iterator for RandomOpenings {
    type Item = Board;

    fn next(&mut self) -> Option<Board> {
        let walked =
            (0..self.plies.plies()).try_fold(Board::START, |board, _| self.random_move(board));
        walked.or_else(|| self.next())
    }
}
