use super::Arguments;
use super::contestant::Contestant;
use super::role::Role;

pub struct Challenger;

impl Role for Challenger {
    fn pick(arguments: &Arguments) -> &Contestant {
        &arguments.challenger
    }
}
