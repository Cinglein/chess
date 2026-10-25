use super::matching::Matching;

pub struct Prefix;

impl Matching for Prefix {
    fn holds(name: &str, word: &str) -> bool {
        name.starts_with(word)
    }
}
