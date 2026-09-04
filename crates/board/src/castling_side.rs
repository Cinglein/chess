use strum::{EnumIter, VariantArray};

use crate::file::File;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, EnumIter, VariantArray)]
#[repr(u8)]
pub enum CastlingSide {
    Kingside,
    Queenside,
}

impl CastlingSide {
    #[must_use]
    pub const fn from_king_destination_file(file: File) -> Option<CastlingSide> {
        match file {
            File::G => Some(CastlingSide::Kingside),
            File::C => Some(CastlingSide::Queenside),
            _ => None,
        }
    }

    #[must_use]
    pub const fn king_destination_file(self) -> File {
        match self {
            CastlingSide::Kingside => File::G,
            CastlingSide::Queenside => File::C,
        }
    }

    #[must_use]
    pub const fn rook_file(self) -> File {
        match self {
            CastlingSide::Kingside => File::H,
            CastlingSide::Queenside => File::A,
        }
    }

    #[must_use]
    pub const fn rook_destination_file(self) -> File {
        match self {
            CastlingSide::Kingside => File::F,
            CastlingSide::Queenside => File::D,
        }
    }
}
