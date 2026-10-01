mod evaluator;
mod piece_kind_value;

pub use evaluator::Evaluator;
pub use piece_kind_value::PieceKindValue;

use board::{Board, Color, PieceKind, Square};
use enum_map::EnumMap;
use strum::VariantArray;

use crate::score::Score;

pub struct PieceSquareTables;

impl PieceSquareTables {
    #[rustfmt::skip]
    const PLACEMENT: EnumMap<PieceKind, EnumMap<Square, Score>> = EnumMap::from_array([
        EnumMap::from_array(Score::table([
              0,   0,   0,   0,   0,   0,   0,   0,
              5,  10,  10, -20, -20,  10,  10,   5,
              5,  -5, -10,   0,   0, -10,  -5,   5,
              0,   0,   0,  20,  20,   0,   0,   0,
              5,   5,  10,  25,  25,  10,   5,   5,
             10,  10,  20,  30,  30,  20,  10,  10,
             50,  50,  50,  50,  50,  50,  50,  50,
              0,   0,   0,   0,   0,   0,   0,   0,
        ])),
        EnumMap::from_array(Score::table([
            -50, -40, -30, -30, -30, -30, -40, -50,
            -40, -20,   0,   5,   5,   0, -20, -40,
            -30,   5,  10,  15,  15,  10,   5, -30,
            -30,   0,  15,  20,  20,  15,   0, -30,
            -30,   5,  15,  20,  20,  15,   5, -30,
            -30,   0,  10,  15,  15,  10,   0, -30,
            -40, -20,   0,   0,   0,   0, -20, -40,
            -50, -40, -30, -30, -30, -30, -40, -50,
        ])),
        EnumMap::from_array(Score::table([
            -20, -10, -10, -10, -10, -10, -10, -20,
            -10,   5,   0,   0,   0,   0,   5, -10,
            -10,  10,  10,  10,  10,  10,  10, -10,
            -10,   0,  10,  10,  10,  10,   0, -10,
            -10,   5,   5,  10,  10,   5,   5, -10,
            -10,   0,   5,  10,  10,   5,   0, -10,
            -10,   0,   0,   0,   0,   0,   0, -10,
            -20, -10, -10, -10, -10, -10, -10, -20,
        ])),
        EnumMap::from_array(Score::table([
              0,   0,   0,   5,   5,   0,   0,   0,
             -5,   0,   0,   0,   0,   0,   0,  -5,
             -5,   0,   0,   0,   0,   0,   0,  -5,
             -5,   0,   0,   0,   0,   0,   0,  -5,
             -5,   0,   0,   0,   0,   0,   0,  -5,
             -5,   0,   0,   0,   0,   0,   0,  -5,
              5,  10,  10,  10,  10,  10,  10,   5,
              0,   0,   0,   0,   0,   0,   0,   0,
        ])),
        EnumMap::from_array(Score::table([
            -20, -10, -10,  -5,  -5, -10, -10, -20,
            -10,   0,   5,   0,   0,   0,   0, -10,
            -10,   5,   5,   5,   5,   5,   0, -10,
              0,   0,   5,   5,   5,   5,   0,  -5,
             -5,   0,   5,   5,   5,   5,   0,  -5,
            -10,   0,   5,   5,   5,   5,   0, -10,
            -10,   0,   0,   0,   0,   0,   0, -10,
            -20, -10, -10,  -5,  -5, -10, -10, -20,
        ])),
        EnumMap::from_array(Score::table([
             20,  30,  10,   0,   0,  10,  30,  20,
             20,  20,   0,   0,   0,   0,  20,  20,
            -10, -20, -20, -20, -20, -20, -20, -10,
            -20, -30, -30, -40, -40, -30, -30, -20,
            -30, -40, -40, -50, -50, -40, -40, -30,
            -30, -40, -40, -50, -50, -40, -40, -30,
            -30, -40, -40, -50, -50, -40, -40, -30,
            -30, -40, -40, -50, -50, -40, -40, -30,
        ])),
    ]);

    #[must_use]
    pub fn piece_value(color: Color, kind: PieceKind, square: Square) -> Score {
        let from_white = match color {
            Color::White => square,
            Color::Black => square.mirrored(),
        };
        PieceKindValue::material(kind) + Self::PLACEMENT[kind][from_white]
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

    use super::{Evaluator, PieceSquareTables};
    use crate::score::Score;

    const CAPTURED_KNIGHT: &str = "rnbqkb1r/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    const KNIGHT: PieceKind = PieceKind::Knight;
    const HOME: Square = Square::G8;
    const CENTRE: Square = Square::E4;
    const CORNER: Square = Square::A1;

    #[test]
    fn the_start_position_is_balanced_a_lost_piece_costs_its_value_and_its_square_adds_to_it() {
        assert_eq!(PieceSquareTables::evaluate(&Board::START), Score::DRAW);
        let down_a_knight: Board = CAPTURED_KNIGHT.parse().unwrap();
        let knight = PieceSquareTables::piece_value(Color::Black, KNIGHT, HOME);
        assert_eq!(PieceSquareTables::evaluate(&down_a_knight), knight);
        assert!(
            PieceSquareTables::piece_value(Color::White, KNIGHT, CENTRE)
                > PieceSquareTables::piece_value(Color::White, KNIGHT, CORNER)
        );
    }
}
