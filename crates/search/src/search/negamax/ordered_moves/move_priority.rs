use board::{Board, ChessMove, MoveKind, PieceKind};
use eval::{PieceKindValue, Score};

use super::killers::Killers;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum MovePriority {
    Quiet,
    Killer,
    Capture(CaptureGain),
    Promotion,
    Principal,
}

impl MovePriority {
    pub(crate) fn rank(
        chess_move: ChessMove,
        board: &Board,
        killers: Killers,
        principal: Option<ChessMove>,
    ) -> MovePriority {
        let placement = board.placement();
        let victim = chess_move.victim(placement);
        let attacker = placement.piece_at(chess_move.origin());
        match (victim, attacker) {
            _ if principal == Some(chess_move) => MovePriority::Principal,
            _ if chess_move.promotion_piece().is_some() => MovePriority::Promotion,
            (Some(victim), Some(attacker)) => {
                MovePriority::Capture(CaptureGain::new(victim.kind(), attacker.kind()))
            }
            _ if killers.remembers(chess_move) => MovePriority::Killer,
            _ => MovePriority::Quiet,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct CaptureGain {
    victim: Score,
    cheapest_attacker: Score,
}

impl CaptureGain {
    fn new(victim: PieceKind, attacker: PieceKind) -> CaptureGain {
        CaptureGain {
            victim: PieceKindValue::material(victim),
            cheapest_attacker: -PieceKindValue::material(attacker),
        }
    }
}

#[cfg(test)]
mod tests {
    use board::{Board, ChessMove};

    use super::{Killers, MovePriority};

    const BOARD: &str = "r3k3/1P6/8/3p4/4P3/8/8/4K3 w - - 0 1";
    const PRINCIPAL: &str = "e1d1";
    const PROMOTION: &str = "b7a8q";
    const CAPTURE: &str = "e4d5";
    const KILLER: &str = "e4e5";
    const QUIET: &str = "e1e2";
    const RANKED: [(&str, MovePriority); 4] = [
        (PRINCIPAL, MovePriority::Principal),
        (PROMOTION, MovePriority::Promotion),
        (KILLER, MovePriority::Killer),
        (QUIET, MovePriority::Quiet),
    ];

    struct Fixture;

    impl Fixture {
        fn resolved(board: &Board, text: &str) -> ChessMove {
            board.resolve_move(text.parse().unwrap()).unwrap()
        }
    }

    #[test]
    fn each_move_is_ranked_by_what_it_does_for_the_search() {
        let board: Board = BOARD.parse().unwrap();
        let killers = Killers::NONE.remembering(Fixture::resolved(&board, KILLER));
        let principal = Some(Fixture::resolved(&board, PRINCIPAL));
        for (text, priority) in RANKED {
            let chess_move = Fixture::resolved(&board, text);
            assert_eq!(
                MovePriority::rank(chess_move, &board, killers, principal),
                priority,
                "{text}"
            );
        }
        let capture = Fixture::resolved(&board, CAPTURE);
        assert!(matches!(
            MovePriority::rank(capture, &board, killers, principal),
            MovePriority::Capture(_)
        ));
    }
}
