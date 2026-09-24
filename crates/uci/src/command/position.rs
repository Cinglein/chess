use super::uci_error::UciError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Position<'line> {
    fen: Option<&'line str>,
    moves: &'line str,
}

impl<'line> Position<'line> {
    #[must_use]
    pub const fn fen(&self) -> Option<&'line str> {
        self.fen
    }

    pub fn moves(&self) -> impl Iterator<Item = &'line str> {
        self.moves.split_whitespace()
    }
}

impl<'line> TryFrom<&'line str> for Position<'line> {
    type Error = UciError;

    fn try_from(rest: &'line str) -> Result<Position<'line>, UciError> {
        let (setup, moves) = rest.split_once("moves").unwrap_or((rest, ""));
        let fen = match setup.trim() {
            "startpos" => None,
            described => Some(
                described
                    .strip_prefix("fen")
                    .ok_or(UciError::UnknownPosition)?
                    .trim(),
            ),
        };
        Ok(Position { fen, moves })
    }
}
