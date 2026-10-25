use core::time::Duration;

use board::Color;

use super::Clock;
use super::clock_part::ClockPart;

pub struct Increment;

impl ClockPart for Increment {
    fn read(clock: &Clock, side: Color) -> Duration {
        match side {
            Color::White => clock.white_increment,
            Color::Black => clock.black_increment,
        }
    }
}
