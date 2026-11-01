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
        let lost = score.filter(|score| rules.resign().is_lost(*score));
        Streaks {
            level: Self::lengthened(
                self.level,
                score.filter(|score| rules.draw().is_level(*score)),
            ),
            losing: self.losing.map(|side, run| {
                if side == mover {
                    Self::lengthened(run, lost)
                } else {
                    run
                }
            }),
        }
    }

    fn lengthened(run: PlyCount, continued: Option<Score>) -> PlyCount {
        continued.map_or(PlyCount::ZERO, |_| run.incremented())
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
