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
            generator: StdRng::seed_from_u64(seed.bits()),
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

#[cfg(test)]
mod tests {
    use board::PlyCount;

    use super::{RandomOpenings, Seed};

    const SEED: Seed = Seed::new(7);
    const OTHER_SEED: Seed = Seed::new(8);
    const PLIES: PlyCount = PlyCount::new(8);

    #[test]
    fn a_seed_reproduces_its_openings_another_seed_differs_and_each_has_walked_the_asked_plies() {
        let first = RandomOpenings::seeded(SEED, PLIES).next().unwrap();
        let again = RandomOpenings::seeded(SEED, PLIES).next().unwrap();
        assert_eq!(first, again);
        assert_ne!(
            RandomOpenings::seeded(OTHER_SEED, PLIES).next(),
            Some(first)
        );
        assert_eq!(first.fullmove_number().to_string(), "5");
    }
}
