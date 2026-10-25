use derive_more::Display;

use crate::score::Score;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display)]
pub struct EvalScale(i16);

impl EvalScale {
    #[must_use]
    pub const fn new(centipawns: i16) -> EvalScale {
        EvalScale(centipawns)
    }

    #[must_use]
    pub fn centipawns(self) -> Score {
        Score::new(i32::from(self.0))
    }

    #[must_use]
    pub fn per_unit(self) -> f32 {
        f32::from(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::Score;

    use super::EvalScale;

    const SCALE: EvalScale = EvalScale::new(40);

    #[test]
    fn an_eval_scale_is_the_same_number_as_a_score_and_as_a_float() {
        assert_eq!(
            (SCALE.centipawns(), SCALE.per_unit()),
            (Score::new(40), 40.0)
        );
    }
}
