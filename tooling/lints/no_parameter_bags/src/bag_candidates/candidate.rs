use task::LineNumber;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Candidate {
    name: String,
    line: LineNumber,
}

impl Candidate {
    pub fn new(name: String, line: LineNumber) -> Candidate {
        Candidate { name, line }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn line(&self) -> LineNumber {
        self.line
    }
}
