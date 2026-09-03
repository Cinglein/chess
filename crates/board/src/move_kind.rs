use crate::promotion::Promotion;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MoveKind {
    Normal,
    Promotion(Promotion),
    EnPassant,
    Castling,
}
