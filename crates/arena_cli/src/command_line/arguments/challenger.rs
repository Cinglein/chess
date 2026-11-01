use super::Arguments;
use super::role::Role;
use opponent_ports::Contestant;

pub struct Challenger;

impl Role for Challenger {
    fn pick(arguments: &Arguments) -> &Contestant {
        &arguments.challenger
    }
}
