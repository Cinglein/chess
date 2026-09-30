use std::fmt;

use board::State;

use super::label::Label;
use super::outcome::Outcome;
use super::record::Record;

#[derive(Debug)]
pub struct Finished {
    record: Record,
    outcome: Outcome,
}

impl State for Finished {}

impl Finished {
    #[must_use]
    pub fn new(record: Record, outcome: Outcome) -> Finished {
        Finished { record, outcome }
    }

    #[must_use]
    pub const fn outcome(&self) -> &Outcome {
        &self.outcome
    }

    pub fn labels(&self) -> impl Iterator<Item = Label> + '_ {
        self.record.labelled(self.outcome.verdict())
    }
}

impl fmt::Display for Finished {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(formatter, "{}", self.record)?;
        write!(formatter, "{}", self.outcome)
    }
}
