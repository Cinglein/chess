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
