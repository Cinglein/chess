use super::Policy;
use super::vocabulary::Vocabulary;

pub struct WireType;

impl Vocabulary for WireType {
    fn entries(policy: &Policy) -> &[String] {
        &policy.wire_types
    }
}
