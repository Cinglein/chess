use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Error)]
pub enum FenError {
    #[error("expected six space separated fields")]
    FieldCount,
    #[error("expected eight ranks separated by slashes")]
    RankCount,
    #[error("a rank must describe exactly eight squares")]
    RankWidth,
    #[error("unknown piece letter {0}")]
    Piece(char),
    #[error("side to move must be w or b")]
    SideToMove,
    #[error("castling rights must be - or a subset of KQkq without repeats")]
    CastlingRights,
    #[error("en passant square must be - or a square")]
    EnPassant,
    #[error("halfmove clock must be a small non-negative number")]
    HalfmoveClock,
    #[error("fullmove number must be a positive number")]
    FullmoveNumber,
}
