use crate::bitboard::Bitboard;
use crate::color::Color;
use crate::piece_kind::PieceKind;
use crate::placement::PiecePlacement;
use crate::slider::{Bishop, Rook, Slider};
use crate::square::Square;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KingSafety {
    king: Square,
    checkers: Bitboard,
    pinned: Bitboard,
    targets: Bitboard,
}

impl KingSafety {
    #[must_use]
    pub fn new(placement: &PiecePlacement, us: Color) -> Option<KingSafety> {
        let king = placement
            .pieces(us, PieceKind::King)
            .least_significant_bit()?;
        let occupied = placement.occupied();
        let ours = placement.occupied_by(us);
        let checkers = placement.attackers(king, !us, occupied);
        Some(KingSafety {
            king,
            checkers,
            pinned: Self::pins(placement, king, us, occupied),
            targets: match checkers.least_significant_bit() {
                None => !ours,
                Some(checker) if checkers.count() == 1 => king.between(checker) | checkers,
                Some(_) => Bitboard::EMPTY,
            },
        })
    }

    #[must_use]
    pub const fn king(&self) -> Square {
        self.king
    }

    #[must_use]
    pub const fn in_check(&self) -> bool {
        !self.checkers.is_empty()
    }

    #[must_use]
    pub fn allowed(&self, from: Square) -> Bitboard {
        if self.pinned.contains(from) {
            self.targets & self.king.line_through(from)
        } else {
            self.targets
        }
    }

    fn pins(placement: &PiecePlacement, king: Square, us: Color, occupied: Bitboard) -> Bitboard {
        let theirs = |kind| placement.pieces(!us, kind);
        let queens = theirs(PieceKind::Queen);
        let snipers = (Rook::attacks(king, Bitboard::EMPTY) & (theirs(PieceKind::Rook) | queens))
            | (Bishop::attacks(king, Bitboard::EMPTY) & (theirs(PieceKind::Bishop) | queens));
        snipers
            .into_iter()
            .map(|sniper| king.between(sniper) & occupied)
            .filter(|blockers| {
                blockers.count() == 1 && !(*blockers & placement.occupied_by(us)).is_empty()
            })
            .fold(Bitboard::EMPTY, |pinned, blocker| pinned | blocker)
    }
}
