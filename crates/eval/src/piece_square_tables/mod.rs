mod evaluator;
mod piece_kind_value;
mod placement_table;

pub use evaluator::Evaluator;
pub use piece_kind_value::PieceKindValue;

use board::{Board, Color, PieceKind, Square};
use placement_table::PlacementTable;
use strum::VariantArray;

use crate::score::Score;

pub struct PieceSquareTables;

impl PieceSquareTables {
    #[must_use]
    pub fn piece_value(color: Color, kind: PieceKind, square: Square) -> Score {
        PieceKindValue::material(kind) + PlacementTable::bonus(color, kind, square)
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

    use super::Evaluator;
    use super::PieceSquareTables;
    use crate::score::Score;

    const CAPTURED_KNIGHT: &str = "rnbqkb1r/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    const WHITE: Color = Color::White;
    const KNIGHT: PieceKind = PieceKind::Knight;
    const CENTRE: Square = Square::E4;
    const CORNER: Square = Square::A1;

    #[test]
    fn the_start_position_is_balanced_and_losing_a_piece_costs_its_value() {
        assert_eq!(PieceSquareTables::evaluate(&Board::START), Score::DRAW);
        let down_a_knight: Board = CAPTURED_KNIGHT.parse().unwrap();
        let knight = PieceSquareTables::piece_value(Color::Black, PieceKind::Knight, Square::G8);
        assert_eq!(PieceSquareTables::evaluate(&down_a_knight), knight);
    }

    #[test]
    fn a_centralised_knight_outscores_a_cornered_one() {
        let centred = PieceSquareTables::piece_value(WHITE, KNIGHT, CENTRE);
        let cornered = PieceSquareTables::piece_value(WHITE, KNIGHT, CORNER);
        assert!(centred > cornered);
    }

    #[test]
    fn tables_are_mirror_images_between_the_colours() {
        for kind in PieceKind::iter() {
            for square in Square::iter() {
                assert_eq!(
                    PieceSquareTables::piece_value(Color::White, kind, square),
                    PieceSquareTables::piece_value(Color::Black, kind, square.mirrored())
                );
            }
        }
    }
}
