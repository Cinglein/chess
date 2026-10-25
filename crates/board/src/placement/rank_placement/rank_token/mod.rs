mod empty_run;

use core::iter;

use empty_run::EmptyRun;
use fen::FenError;

use crate::piece::Piece;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum RankToken {
    Empties(EmptyRun),
    Piece(Piece),
}

impl RankToken {
    pub(super) fn squares(self) -> impl Iterator<Item = Option<Piece>> {
        iter::repeat_n(self.square(), self.length())
    }

    const fn square(self) -> Option<Piece> {
        match self {
            RankToken::Empties(_) => None,
            RankToken::Piece(piece) => Some(piece),
        }
    }

    const fn length(self) -> usize {
        match self {
            RankToken::Empties(run) => run.length(),
            RankToken::Piece(_) => 1,
        }
    }
}

impl TryFrom<char> for RankToken {
    type Error = FenError;

    fn try_from(letter: char) -> Result<RankToken, FenError> {
        match u8::try_from(letter)
            .ok()
            .filter(|_| letter.is_ascii_digit())
            .map(|digit| EmptyRun::new(digit - b'0'))
        {
            Some(run) => Ok(RankToken::Empties(run)),
            None => letter
                .encode_utf8(&mut [0; size_of::<char>()])
                .parse()
                .map(RankToken::Piece)
                .map_err(|_| FenError::Piece(letter)),
        }
    }
}
