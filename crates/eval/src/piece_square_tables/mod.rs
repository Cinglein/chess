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
