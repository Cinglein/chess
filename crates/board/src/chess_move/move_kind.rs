use enum_dispatch::enum_dispatch;

use crate::file::File;
use crate::piece_placement::PiecePlacement;
use crate::promotion_piece::PromotionPiece;
use crate::square::Square;

#[enum_dispatch]
pub trait MoveKind {
    fn origin(&self) -> Square;

    fn destination(&self) -> Square;

    fn play(self, placement: PiecePlacement) -> Option<PiecePlacement>;

    fn en_passant_file(&self) -> Option<File> {
        None
    }

    fn promotion_piece(&self) -> Option<PromotionPiece> {
        None
    }
}
