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
