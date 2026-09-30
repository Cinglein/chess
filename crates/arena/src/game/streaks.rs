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

#[cfg(test)]
mod tests {
    use board::{Color, PlyCount};
    use eval::Score;

    use super::Streaks;
    use crate::rules::Rules;

    const MOVER: Color = Color::White;
    const LEVEL: Option<Score> = Some(Score::DRAW);
    const SILENT: Option<Score> = None;
    const ONE: PlyCount = PlyCount::new(1);

    #[test]
    fn streaks_grow_while_scores_keep_the_same_character_and_reset_when_they_do_not() {
        let rules = Rules::DEFAULT;
        let lost = Some(Score::mated_in(1));
        let grown = Streaks::default()
            .after(MOVER, LEVEL, &rules)
            .after(MOVER, lost, &rules);
        assert_eq!((grown.level(), grown.losing(MOVER)), (PlyCount::ZERO, ONE));
        assert_eq!(
            grown.after(MOVER, SILENT, &rules).losing(MOVER),
            PlyCount::ZERO
        );
    }
}
