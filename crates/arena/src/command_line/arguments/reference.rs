use super::Arguments;
use super::contestant::Contestant;
use super::role::Role;

pub struct Reference;

impl Role for Reference {
    fn pick(arguments: &Arguments) -> &Contestant {
        &arguments.reference
    }
}
