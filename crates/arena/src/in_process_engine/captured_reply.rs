use board::LongAlgebraic;
use engine::Sink;
use eval::Score;
use uci::Response;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct CapturedReply {
    best_move: Option<LongAlgebraic>,
    score: Option<Score>,
}

impl CapturedReply {
    pub(super) const fn best_move(self) -> Option<LongAlgebraic> {
        self.best_move
    }

    pub(super) const fn score(self) -> Option<Score> {
        self.score
    }
}

impl Sink for CapturedReply {
    fn emit(&mut self, response: Response<'_>) {
        *self = CapturedReply {
            best_move: if response.is_best_move() {
                response.best_move()
            } else {
                self.best_move
            },
            score: response
                .info()
                .map_or(self.score, |info| Some(info.score())),
        };
    }
}
