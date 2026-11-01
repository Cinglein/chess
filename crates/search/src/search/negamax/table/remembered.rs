use board::ChessMove;

pub(crate) trait Remembered {
    fn best_move(&self) -> Option<ChessMove>;
}
