#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Candidate {
    name: String,
    line: usize,
}

impl Candidate {
    pub fn new(name: String, line: usize) -> Candidate {
        Candidate { name, line }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn line(&self) -> usize {
        self.line
    }
}
