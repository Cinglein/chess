use arena::Verdict;
use board::Color;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WhiteResult(f32);

impl WhiteResult {
    const WIN: WhiteResult = WhiteResult(1.0);
    const DRAW: WhiteResult = WhiteResult(0.5);
    const LOSS: WhiteResult = WhiteResult(0.0);

    pub const fn fraction(self) -> f32 {
        self.0
    }
}

impl From<Verdict> for WhiteResult {
    fn from(verdict: Verdict) -> WhiteResult {
        match verdict.winner() {
            Some(Color::White) => WhiteResult::WIN,
            Some(Color::Black) => WhiteResult::LOSS,
            None => WhiteResult::DRAW,
        }
    }
}
