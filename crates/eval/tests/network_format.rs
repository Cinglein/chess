use eval::{NetworkFormat, Score};

const EVAL_CENTIPAWNS: i32 = 400;
const EVAL_UNITS: f32 = 400.0;
const OUTPUT_BIAS_FACTOR: i16 = 16320;

#[test]
fn the_eval_scale_is_the_same_number_as_a_score_and_as_a_float() {
    let scale = NetworkFormat::EVAL_SCALE;
    assert_eq!(
        (scale.centipawns(), scale.per_unit()),
        (Score::new(EVAL_CENTIPAWNS), EVAL_UNITS)
    );
}

#[test]
fn quantisations_multiply_when_two_quantised_layers_meet() {
    assert_eq!(
        NetworkFormat::OUTPUT_BIAS_QUANTISATION.factor(),
        OUTPUT_BIAS_FACTOR
    );
}
