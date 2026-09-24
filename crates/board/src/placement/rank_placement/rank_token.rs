use core::iter;

use fen::FenError;

use crate::piece::Piece;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum RankToken {
    Empties(u8),
    Piece(Piece),
}

impl RankToken {
    pub(super) fn squares(self) -> impl Iterator<Item = Option<Piece>> {
        iter::repeat_n(self.square(), self.count())
    }

    const fn square(self) -> Option<Piece> {
        match self {
            RankToken::Empties(_) => None,
            RankToken::Piece(piece) => Some(piece),
        }
    }

    const fn count(self) -> usize {
        match self {
            RankToken::Empties(empties) => empties as usize,
            RankToken::Piece(_) => 1,
        }
    }
}

impl TryFrom<char> for RankToken {
    type Error = FenError;

    fn try_from(letter: char) -> Result<RankToken, FenError> {
        match letter
            .to_digit(10)
            .and_then(|digit| u8::try_from(digit).ok())
        {
            Some(empties) => Ok(RankToken::Empties(empties)),
            None => letter
                .encode_utf8(&mut [0; 4])
                .parse()
                .map(RankToken::Piece)
                .map_err(|_| FenError::Piece(letter)),
        }
    }
}
