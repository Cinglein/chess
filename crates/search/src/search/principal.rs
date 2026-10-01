use board::ChessMove;
use eval::Score;

use super::negamax::{Bound, Lower, Window};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Principal {
    chess_move: Option<ChessMove>,
    score: Score,
}

impl Principal {
    pub(crate) const NONE: Principal = Principal {
        chess_move: None,
        score: Score::INFINITY.negated(),
    };

    pub(crate) const fn chess_move(self) -> Option<ChessMove> {
        self.chess_move
    }

    pub(crate) const fn score(self) -> Score {
        self.score
    }

    pub(crate) fn window(self) -> Window {
        Window::FULL.below(-Bound::<Lower>::new(self.score))
    }

    pub(crate) fn improved(self, chess_move: ChessMove, score: Score) -> Principal {
        if score > self.score {
            Principal {
                chess_move: Some(chess_move),
                score,
            }
        } else {
            self
        }
    }
}

#[cfg(test)]
mod tests {
    use board::Board;
    use eval::Score;

    use super::Principal;

    const BETTER: Score = Score::new(10);

    #[test]
    fn a_later_move_replaces_the_principal_only_by_scoring_strictly_higher() {
        let [first, second, ..] = Board::START.legal_moves()[..] else {
            panic!()
        };
        let principal = Principal::NONE
            .improved(first, Score::DRAW)
            .improved(second, Score::DRAW);
        assert_eq!(
            (principal.chess_move(), principal.score()),
            (Some(first), Score::DRAW)
        );
        assert_eq!(
            principal.improved(second, BETTER).chess_move(),
            Some(second)
        );
    }
}
