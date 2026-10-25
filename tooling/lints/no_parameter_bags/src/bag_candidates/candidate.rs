use task::Name;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Candidate {
    name: Name,
    line: usize,
}

impl Candidate {
    pub fn new(name: Name, line: usize) -> Candidate {
        Candidate { name, line }
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn line(&self) -> usize {
        self.line
    }
}
