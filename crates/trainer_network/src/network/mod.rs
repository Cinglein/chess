mod layer;

use bullet_lib::game::inputs::Chess768;
use bullet_lib::nn::optimiser::{AdamW, AdamWOptimiser};
use bullet_lib::trainer::save::SavedFormat;
use bullet_lib::value::{NoOutputBuckets, ValueTrainer, ValueTrainerBuilder};
use eval::NetworkFormat;
use layer::{Biases, Layer, Part, Weights};

pub struct Network;

impl Network {
    #[must_use]
    pub fn trainer() -> ValueTrainer<AdamWOptimiser, Chess768, NoOutputBuckets> {
        ValueTrainerBuilder::default()
            .dual_perspective()
            .optimiser(AdamW)
            .inputs(Chess768)
            .save_format(&[
                SavedFormat::id(Weights::saved_name(Layer::Hidden))
                    .round()
                    .quantise::<i16>(NetworkFormat::HIDDEN_QUANTISATION.factor()),
                SavedFormat::id(Biases::saved_name(Layer::Hidden))
                    .round()
                    .quantise::<i16>(NetworkFormat::HIDDEN_QUANTISATION.factor()),
                SavedFormat::id(Weights::saved_name(Layer::Output))
                    .round()
                    .quantise::<i16>(NetworkFormat::OUTPUT_QUANTISATION.factor()),
                SavedFormat::id(Biases::saved_name(Layer::Output))
                    .round()
                    .quantise::<i16>(NetworkFormat::OUTPUT_BIAS_QUANTISATION.factor()),
            ])
            .loss_fn(|output, target| output.sigmoid().squared_error(target))
            .build(|builder, stm_inputs, ntm_inputs| {
                let hidden = builder.new_affine(
                    Layer::Hidden.as_ref(),
                    NetworkFormat::INPUT_SIZE,
                    NetworkFormat::HIDDEN_SIZE,
                );
                let output = builder.new_affine(
                    Layer::Output.as_ref(),
                    NetworkFormat::BOTH_PERSPECTIVES_SIZE,
                    NetworkFormat::OUTPUT_SIZE,
                );
                let stm_hidden = hidden.forward(stm_inputs).screlu();
                let ntm_hidden = hidden.forward(ntm_inputs).screlu();
                output.forward(stm_hidden.concat(ntm_hidden))
            })
    }
}
