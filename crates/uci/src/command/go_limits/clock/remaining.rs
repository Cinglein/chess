use core::time::Duration;

use board::Color;

use super::Clock;
use super::clock_part::ClockPart;

pub struct Remaining;

impl ClockPart for Remaining {
    fn read(clock: &Clock, side: Color) -> Duration {
        match side {
            Color::White => clock.white,
            Color::Black => clock.black,
        }
    }
}
