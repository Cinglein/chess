use board::{Board, Color, PieceKind, Rank, Square};
use enum_map::EnumMap;
use strum::{EnumCount, VariantArray};

use crate::evaluator::Evaluator;
use crate::score::Score;

pub struct PieceSquareTables;

impl PieceSquareTables {
    const MATERIAL: EnumMap<PieceKind, i32> =
        EnumMap::from_array([100, 320, 330, 500, 900, 20_000]);

    #[rustfmt::skip]
    const PLACEMENT: EnumMap<PieceKind, [i32; Square::COUNT]> = EnumMap::from_array([
        [
              0,   0,   0,   0,   0,   0,   0,   0,
              5,  10,  10, -20, -20,  10,  10,   5,
              5,  -5, -10,   0,   0, -10,  -5,   5,
              0,   0,   0,  20,  20,   0,   0,   0,
              5,   5,  10,  25,  25,  10,   5,   5,
             10,  10,  20,  30,  30,  20,  10,  10,
             50,  50,  50,  50,  50,  50,  50,  50,
              0,   0,   0,   0,   0,   0,   0,   0,
        ],
        [
            -50, -40, -30, -30, -30, -30, -40, -50,
            -40, -20,   0,   5,   5,   0, -20, -40,
            -30,   5,  10,  15,  15,  10,   5, -30,
            -30,   0,  15,  20,  20,  15,   0, -30,
            -30,   5,  15,  20,  20,  15,   5, -30,
            -30,   0,  10,  15,  15,  10,   0, -30,
            -40, -20,   0,   0,   0,   0, -20, -40,
            -50, -40, -30, -30, -30, -30, -40, -50,
        ],
        [
            -20, -10, -10, -10, -10, -10, -10, -20,
            -10,   5,   0,   0,   0,   0,   5, -10,
            -10,  10,  10,  10,  10,  10,  10, -10,
            -10,   0,  10,  10,  10,  10,   0, -10,
            -10,   5,   5,  10,  10,   5,   5, -10,
            -10,   0,   5,  10,  10,   5,   0, -10,
            -10,   0,   0,   0,   0,   0,   0, -10,
            -20, -10, -10, -10, -10, -10, -10, -20,
        ],
        [
              0,   0,   0,   5,   5,   0,   0,   0,
             -5,   0,   0,   0,   0,   0,   0,  -5,
             -5,   0,   0,   0,   0,   0,   0,  -5,
             -5,   0,   0,   0,   0,   0,   0,  -5,
             -5,   0,   0,   0,   0,   0,   0,  -5,
             -5,   0,   0,   0,   0,   0,   0,  -5,
              5,  10,  10,  10,  10,  10,  10,   5,
              0,   0,   0,   0,   0,   0,   0,   0,
        ],
        [
            -20, -10, -10,  -5,  -5, -10, -10, -20,
            -10,   0,   5,   0,   0,   0,   0, -10,
            -10,   5,   5,   5,   5,   5,   0, -10,
              0,   0,   5,   5,   5,   5,   0,  -5,
             -5,   0,   5,   5,   5,   5,   0,  -5,
            -10,   0,   5,   5,   5,   5,   0, -10,
            -10,   0,   0,   0,   0,   0,   0, -10,
            -20, -10, -10,  -5,  -5, -10, -10, -20,
        ],
        [
             20,  30,  10,   0,   0,  10,  30,  20,
             20,  20,   0,   0,   0,   0,  20,  20,
            -10, -20, -20, -20, -20, -20, -20, -10,
            -20, -30, -30, -40, -40, -30, -30, -20,
            -30, -40, -40, -50, -50, -40, -40, -30,
            -30, -40, -40, -50, -50, -40, -40, -30,
            -30, -40, -40, -50, -50, -40, -40, -30,
            -30, -40, -40, -50, -50, -40, -40, -30,
        ],
    ]);

    #[must_use]
    pub fn piece_value(color: Color, kind: PieceKind, square: Square) -> Score {
        let from_white = match color {
            Color::White => square,
            Color::Black => Square::new(square.file(), Self::mirrored(square.rank())),
        };
        Score::new(Self::MATERIAL[kind] + Self::PLACEMENT[kind][from_white as usize])
    }

    fn mirrored(rank: Rank) -> Rank {
        Rank::VARIANTS[Rank::COUNT - 1 - rank as usize]
    }

    fn side_value(board: &Board, color: Color) -> Score {
        PieceKind::VARIANTS
            .iter()
            .flat_map(|kind| {
                board
                    .placement()
                    .pieces(color, *kind)
                    .into_iter()
                    .map(move |square| Self::piece_value(color, *kind, square))
            })
            .sum()
    }
}

impl Evaluator for PieceSquareTables {
    fn evaluate(board: &Board) -> Score {
        let us = board.side_to_move();
        Self::side_value(board, us) - Self::side_value(board, !us)
    }
}

#[cfg(test)]
mod tests {
    use board::{Board, Color, PieceKind, Square};
    use strum::IntoEnumIterator;

    use super::PieceSquareTables;
    use crate::evaluator::Evaluator;
    use crate::score::Score;

    const CAPTURED_KNIGHT: &str = "rnbqkb1r/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    #[test]
    fn the_start_position_is_balanced_and_losing_a_piece_costs_its_value() {
        assert_eq!(PieceSquareTables::evaluate(&Board::START), Score::DRAW);
        let down_a_knight: Board = CAPTURED_KNIGHT.parse().unwrap();
        let knight = PieceSquareTables::piece_value(Color::Black, PieceKind::Knight, Square::G8);
        assert_eq!(PieceSquareTables::evaluate(&down_a_knight), knight);
    }

    #[test]
    fn tables_are_mirror_images_between_the_colours() {
        for kind in PieceKind::iter() {
            for square in Square::iter() {
                let mirrored =
                    Square::new(square.file(), PieceSquareTables::mirrored(square.rank()));
                assert_eq!(
                    PieceSquareTables::piece_value(Color::White, kind, square),
                    PieceSquareTables::piece_value(Color::Black, kind, mirrored)
                );
            }
        }
    }
}
