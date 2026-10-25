use super::matching::Matching;

pub struct Whole;

impl Matching for Whole {
    fn holds(name: &str, word: &str) -> bool {
        name == word
    }
}
