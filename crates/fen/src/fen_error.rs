use core::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FenError {
    FieldCount,
    RankCount,
    RankWidth,
    Piece(char),
    SideToMove,
    CastlingRights,
    EnPassant,
    HalfmoveClock,
    FullmoveNumber,
}

impl fmt::Display for FenError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FenError::FieldCount => formatter.write_str("expected six space separated fields"),
            FenError::RankCount => formatter.write_str("expected eight ranks separated by slashes"),
            FenError::RankWidth => {
                formatter.write_str("a rank must describe exactly eight squares")
            }
            FenError::Piece(letter) => write!(formatter, "unknown piece letter {letter}"),
            FenError::SideToMove => formatter.write_str("side to move must be w or b"),
            FenError::CastlingRights => {
                formatter.write_str("castling rights must be - or a subset of KQkq without repeats")
            }
            FenError::EnPassant => formatter.write_str("en passant square must be - or a square"),
            FenError::HalfmoveClock => {
                formatter.write_str("halfmove clock must be a small non-negative number")
            }
            FenError::FullmoveNumber => {
                formatter.write_str("fullmove number must be a positive number")
            }
        }
    }
}

impl core::error::Error for FenError {}
