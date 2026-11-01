use std::str::FromStr;

use super::code::Code;

pub(super) struct Codes(Vec<Code>);

impl Codes {
    pub(super) fn contains(&self, code: Code) -> bool {
        self.0.contains(&code)
    }
}

impl FromStr for Codes {
    type Err = std::num::ParseIntError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        text.split_whitespace()
            .map(str::parse)
            .collect::<Result<_, _>>()
            .map(Self)
    }
}
