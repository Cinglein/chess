use board::{Color, PlyCount};
use enum_map::EnumMap;
use eval::Score;

use crate::rules::Rules;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Streaks {
    level: PlyCount,
    losing: EnumMap<Color, PlyCount>,
}

impl Streaks {
    #[must_use]
    pub fn after(self, mover: Color, score: Option<Score>, rules: &Rules) -> Streaks {
        let mut losing = self.losing;
        losing[mover] = match score {
            Some(score) if rules.resign().is_lost(score) => self.losing[mover].incremented(),
            _ => PlyCount::ZERO,
        };
        Streaks {
            level: match score {
                Some(score) if rules.draw().is_level(score) => self.level.incremented(),
                _ => PlyCount::ZERO,
            },
            losing,
        }
    }

    #[must_use]
    pub const fn level(self) -> PlyCount {
        self.level
    }

    #[must_use]
    pub fn losing(self, side: Color) -> PlyCount {
        self.losing[side]
    }
}
