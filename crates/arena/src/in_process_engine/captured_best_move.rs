use board::LongAlgebraic;
use engine::Sink;
use uci::Response;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct CapturedBestMove {
    best_move: Option<LongAlgebraic>,
}

impl CapturedBestMove {
    pub(super) const fn best_move(self) -> Option<LongAlgebraic> {
        self.best_move
    }
}

impl Sink for CapturedBestMove {
    fn emit(&mut self, response: Response<'_>) {
        if response.is_best_move() {
            self.best_move = response.best_move();
        }
    }
}
