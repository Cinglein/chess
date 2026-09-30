use board::Color;
use enum_map::EnumMap;
use eval::Score;

use crate::rules::Rules;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Streaks {
    level: usize,
    losing: EnumMap<Color, usize>,
}

impl Streaks {
    #[must_use]
    pub fn after(self, mover: Color, score: Option<Score>, rules: &Rules) -> Streaks {
        let mut losing = self.losing;
        losing[mover] = match score {
            Some(score) if rules.resign().is_lost(score) => self.losing[mover] + 1,
            _ => 0,
        };
        Streaks {
            level: match score {
                Some(score) if rules.draw().is_level(score) => self.level + 1,
                _ => 0,
            },
            losing,
        }
    }

    #[must_use]
    pub const fn level(&self) -> usize {
        self.level
    }

    #[must_use]
    pub fn losing(&self, side: Color) -> usize {
        self.losing[side]
    }
}

#[cfg(test)]
mod tests {
    use board::Color;
    use eval::Score;

    use super::Streaks;
    use crate::rules::Rules;

    const MOVER: Color = Color::White;
    const LEVEL: Option<Score> = Some(Score::DRAW);
    const SILENT: Option<Score> = None;

    #[test]
    fn streaks_grow_while_scores_keep_the_same_character_and_reset_when_they_do_not() {
        let rules = Rules::DEFAULT;
        let lost = Some(Score::mated_in(1));
        let grown = Streaks::default()
            .after(MOVER, LEVEL, &rules)
            .after(MOVER, lost, &rules);
        assert_eq!((grown.level(), grown.losing(MOVER)), (0, 1));
        assert_eq!(grown.after(MOVER, SILENT, &rules).losing(MOVER), 0);
    }
}
