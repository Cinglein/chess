use super::Arguments;
use super::role::Role;
use opponent_ports::Contestant;

pub struct Reference;

impl Role for Reference {
    fn pick(arguments: &Arguments) -> &Contestant {
        &arguments.reference
    }
}
