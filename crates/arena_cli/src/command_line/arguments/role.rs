use super::Arguments;
use opponent_ports::Contestant;

pub trait Role {
    fn pick(arguments: &Arguments) -> &Contestant;
}
