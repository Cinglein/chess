use core::time::Duration;

use board::Color;

use super::Clock;

pub trait ClockPart {
    fn read(clock: &Clock, side: Color) -> Duration;
}
