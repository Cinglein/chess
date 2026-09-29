use derive_more::{Display, FromStr};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Display, FromStr)]
pub struct Elo(u16);

impl Elo {
    pub const STOCKFISH_FLOOR: Elo = Elo(1320);
}
