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
    use board::{Board, ChessMove, PieceKind};

    use super::{CaptureGain, Killers, MovePriority};

    const BOARD: &str = "r3k3/1P6/8/3p4/4P3/2N5/8/4K3 w - - 0 1";
    const PRINCIPAL: &str = "e1d1";
    const KILLER: &str = "e4e5";
    const RANKED: [&str; 6] = [PRINCIPAL, "b7a8q", "e4d5", "c3d5", KILLER, "e1e2"];

    struct Fixture;

    impl Fixture {
        fn resolved(board: &Board, text: &str) -> ChessMove {
            board.resolve_move(text.parse().unwrap()).unwrap()
        }

        fn expected() -> [MovePriority; 6] {
            [
                MovePriority::Principal,
                MovePriority::Promotion,
                MovePriority::Capture(CaptureGain::new(PieceKind::Pawn, PieceKind::Pawn)),
                MovePriority::Capture(CaptureGain::new(PieceKind::Pawn, PieceKind::Knight)),
                MovePriority::Killer,
                MovePriority::Quiet,
            ]
        }
    }

    #[test]
    fn each_move_is_ranked_by_what_it_does_for_the_search_and_the_ranks_fall_in_that_order() {
        let board: Board = BOARD.parse().unwrap();
        let killers = Killers::NONE.remembering(Fixture::resolved(&board, KILLER));
        let principal = Some(Fixture::resolved(&board, PRINCIPAL));
        let ranks: Vec<MovePriority> = RANKED
            .iter()
            .map(|text| {
                MovePriority::rank(Fixture::resolved(&board, text), &board, killers, principal)
            })
            .collect();
        assert_eq!(ranks, Fixture::expected());
        assert!(
            ranks.is_sorted_by(|higher, lower| higher > lower),
            "{ranks:?}"
        );
    }
}
