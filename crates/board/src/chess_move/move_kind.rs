use enum_dispatch::enum_dispatch;

use crate::file::File;
use crate::placement::PiecePlacement;
use crate::promotion_piece::PromotionPiece;
use crate::square::Square;

#[enum_dispatch]
pub trait MoveKind {
    fn origin(&self) -> Square;

    fn destination(&self) -> Square;

    fn play(self, placement: PiecePlacement) -> Option<PiecePlacement>;

    fn captures(&self, placement: &PiecePlacement) -> bool {
        placement.piece_at(self.destination()).is_some()
    }

    fn en_passant_file(&self) -> Option<File> {
        None
    }

    fn promotion_piece(&self) -> Option<PromotionPiece> {
        None
    }
}
