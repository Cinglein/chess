mod accumulator;
mod saved_network_error;
mod side;
mod table;
mod weight;

pub use saved_network_error::SavedNetworkError;

use accumulator::Accumulator;
use board::Board;
use side::Side;
use table::Table;
use weight::Weight;

use crate::score::Score;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SavedNetwork<'bytes>(&'bytes [u8]);

impl SavedNetwork<'_> {
    const WEIGHT_BYTES: usize = size_of::<i16>();
    const BYTE_LENGTH: usize = Table::WEIGHT_LENGTH * Self::WEIGHT_BYTES;

    #[must_use]
    pub fn evaluate(&self, board: &Board) -> Score {
        let us = board.side_to_move();
        let hidden = Accumulator::fresh(self, us, board).screlu_dot(self, Side::Ours)
            + Accumulator::fresh(self, !us, board).screlu_dot(self, Side::Theirs);
        hidden.output_centipawns(self.weight(Table::OutputBias, 0))
    }

    fn weight(&self, table: Table, index: usize) -> Weight {
        let start = (table.start_index() + index) * Self::WEIGHT_BYTES;
        Weight::new(i16::from_le_bytes([self.0[start], self.0[start + 1]]))
    }
}

impl<'bytes> TryFrom<&'bytes [u8]> for SavedNetwork<'bytes> {
    type Error = SavedNetworkError;

    fn try_from(bytes: &'bytes [u8]) -> Result<SavedNetwork<'bytes>, SavedNetworkError> {
        (bytes.len() == Self::BYTE_LENGTH)
            .then_some(SavedNetwork(bytes))
            .ok_or(SavedNetworkError::Length {
                expected_bytes: Self::BYTE_LENGTH,
                found_bytes: bytes.len(),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::{SavedNetwork, SavedNetworkError, Table};
    use crate::network_format::NetworkFormat;
    use crate::score::Score;
    use board::{Board, Color, Piece, PieceKind, Square};
    use proptest::prelude::*;
    use strum::{EnumCount, VariantArray};

    const CASES: u32 = 16;
    const MOST: i16 = 64;
    const SPAN: u64 = 2 * MOST as u64 + 1;

    struct Reference {
        weights: Vec<i16>,
    }

    impl Reference {
        const OUTPUT_WEIGHTS: usize =
            NetworkFormat::INPUT_SIZE * NetworkFormat::HIDDEN_SIZE + NetworkFormat::HIDDEN_SIZE;
        const BYTES: usize = (Self::OUTPUT_WEIGHTS
            + NetworkFormat::BOTH_PERSPECTIVES_SIZE
            + NetworkFormat::OUTPUT_SIZE)
            * size_of::<i16>();

        fn seeded(seed: u64) -> Reference {
            let weights = (0..Table::WEIGHT_LENGTH)
                .scan(seed | 1, |state, _| {
                    *state ^= *state << 13;
                    *state ^= *state >> 7;
                    *state ^= *state << 17;
                    Some(i16::try_from(*state % SPAN).unwrap() - MOST)
                })
                .collect();
            Reference { weights }
        }

        fn bytes(&self) -> Vec<u8> {
            self.weights
                .iter()
                .flat_map(|weight| weight.to_le_bytes())
                .collect()
        }

        fn perspective_dot(&self, perspective: Color, board: &Board) -> f64 {
            let placement = board.placement();
            let features: Vec<usize> = placement
                .occupied()
                .into_iter()
                .filter_map(|square| {
                    placement
                        .piece_at(square)
                        .map(|piece| Perspective(perspective).feature(piece, square))
                })
                .collect();
            let half =
                usize::from(perspective != board.side_to_move()) * NetworkFormat::HIDDEN_SIZE;
            (0..NetworkFormat::HIDDEN_SIZE)
                .map(|index| {
                    let bias = f64::from(
                        self.weights
                            [NetworkFormat::INPUT_SIZE * NetworkFormat::HIDDEN_SIZE + index],
                    );
                    let summed: f64 = features
                        .iter()
                        .map(|feature| {
                            f64::from(self.weights[feature * NetworkFormat::HIDDEN_SIZE + index])
                        })
                        .sum();
                    let activation = ((bias + summed)
                        / f64::from(NetworkFormat::HIDDEN_QUANTISATION.factor()))
                    .clamp(0.0, 1.0);
                    let output_weight = self.weights[Self::OUTPUT_WEIGHTS + half + index];
                    activation * activation * f64::from(output_weight)
                        / f64::from(NetworkFormat::OUTPUT_QUANTISATION.factor())
                })
                .sum()
        }

        fn oracle(&self, board: &Board) -> f64 {
            let bias = f64::from(
                self.weights[Self::OUTPUT_WEIGHTS + NetworkFormat::BOTH_PERSPECTIVES_SIZE],
            ) / f64::from(NetworkFormat::OUTPUT_BIAS_QUANTISATION.factor());
            let dots = self.perspective_dot(Color::White, board)
                + self.perspective_dot(Color::Black, board);
            (dots + bias) * f64::from(NetworkFormat::EVAL_SCALE.per_unit())
        }
    }

    struct Perspective(Color);

    impl Perspective {
        fn feature(&self, piece: Piece, square: Square) -> usize {
            let side = if piece.color() == self.0 {
                0
            } else {
                PieceKind::COUNT * Square::COUNT
            };
            let shown = if self.0 == Color::White {
                square
            } else {
                square.mirrored()
            };
            let kind = PieceKind::VARIANTS
                .iter()
                .position(|kind| *kind == piece.kind())
                .unwrap();
            side + kind * Square::COUNT
                + Square::VARIANTS.iter().position(|at| *at == shown).unwrap()
        }
    }

    struct ColourFlip;

    impl ColourFlip {
        fn swapped_case(text: &str) -> String {
            text.chars()
                .map(|letter| {
                    if letter.is_ascii_uppercase() {
                        letter.to_ascii_lowercase()
                    } else {
                        letter.to_ascii_uppercase()
                    }
                })
                .collect()
        }

        fn flipped_fen(fen: &str) -> String {
            let fields: Vec<&str> = fen.split(' ').collect();
            let placement = fields[0]
                .split('/')
                .rev()
                .map(Self::swapped_case)
                .collect::<Vec<String>>()
                .join("/");
            let side = if fields[1] == "w" { "b" } else { "w" };
            let swapped = Self::swapped_case(fields[2]);
            let castling: String = "KQkq"
                .chars()
                .filter(|right| swapped.contains(*right))
                .collect();
            let en_passant: String = fields[3]
                .chars()
                .map(|letter| match letter {
                    '3' => '6',
                    '6' => '3',
                    other => other,
                })
                .collect();
            format!(
                "{placement} {side} {} {en_passant} {} {}",
                if castling.is_empty() { "-" } else { &castling },
                fields[4],
                fields[5]
            )
        }
    }

    #[test]
    fn the_integer_evaluation_tracks_the_real_network_within_the_rounding_quantisation_allows() {
        proptest!(ProptestConfig::with_cases(CASES), |(seed: u64, board: Board)| {
            let reference = Reference::seeded(seed);
            let bytes = reference.bytes();
            let network = SavedNetwork::try_from(bytes.as_slice()).unwrap();
            let evaluated = f64::from(network.evaluate(&board).centipawns());
            let expected = reference.oracle(&board);
            prop_assert!((evaluated - expected).abs() < 2.0, "{evaluated} is not {expected}");
        });
    }

    #[test]
    fn a_colour_flipped_position_scores_the_same_because_the_network_sees_sides_not_colours() {
        proptest!(ProptestConfig::with_cases(CASES), |(seed: u64, board: Board)| {
            let bytes = Reference::seeded(seed).bytes();
            let network = SavedNetwork::try_from(bytes.as_slice()).unwrap();
            let flipped: Board = ColourFlip::flipped_fen(&board.to_string()).parse().unwrap();
            prop_assert_eq!(network.evaluate(&flipped), network.evaluate(&board));
        });
    }

    #[test]
    fn a_wrong_length_is_refused_and_the_loudest_network_never_reads_as_a_mate() {
        assert_eq!(
            SavedNetwork::try_from(&[][..]),
            Err(SavedNetworkError::Length {
                expected_bytes: Reference::BYTES,
                found_bytes: 0
            })
        );
        let loudest = i16::MAX.to_le_bytes().repeat(Table::WEIGHT_LENGTH);
        let score = SavedNetwork::try_from(loudest.as_slice())
            .unwrap()
            .evaluate(&Board::START);
        assert_eq!(score.mate_in_moves(), None);
        assert!(score > Score::new(i32::from(i16::MAX) / 2));
    }
}
