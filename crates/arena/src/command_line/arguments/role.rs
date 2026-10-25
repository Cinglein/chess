use super::Arguments;
use super::contestant::Contestant;

pub trait Role {
    fn pick(arguments: &Arguments) -> &Contestant;
}
