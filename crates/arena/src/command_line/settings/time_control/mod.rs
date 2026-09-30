mod time_control_error;

use std::fmt;
use std::str::FromStr;
use std::time::Duration;

use board::Color;
use serde::{Deserialize, Serialize};
use time_control_error::TimeControlError;
use uci::Clock;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct TimeControl {
    base: Duration,
    increment: Duration,
}

impl TimeControl {
    pub const fn new(base: Duration, increment: Duration) -> TimeControl {
        TimeControl { base, increment }
    }
}

impl From<TimeControl> for Clock {
    fn from(time_control: TimeControl) -> Clock {
        Clock::new(
            time_control.base,
            time_control.base,
            time_control.increment,
            time_control.increment,
        )
    }
}

impl From<Clock> for TimeControl {
    fn from(clock: Clock) -> TimeControl {
        TimeControl {
            base: clock.remaining(Color::White),
            increment: clock.increment(Color::White),
        }
    }
}

impl FromStr for TimeControl {
    type Err = TimeControlError;

    fn from_str(text: &str) -> Result<TimeControl, TimeControlError> {
        let (base, increment) = text.trim().split_once('+').ok_or(TimeControlError::Shape)?;
        Ok(TimeControl {
            base: Duration::from_secs_f64(base.trim().parse()?),
            increment: Duration::from_secs_f64(increment.trim().parse()?),
        })
    }
}

impl TryFrom<String> for TimeControl {
    type Error = TimeControlError;

    fn try_from(text: String) -> Result<TimeControl, TimeControlError> {
        text.parse()
    }
}

impl From<TimeControl> for String {
    fn from(time_control: TimeControl) -> String {
        time_control.to_string()
    }
}

impl fmt::Display for TimeControl {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}+{}",
            self.base.as_secs_f64(),
            self.increment.as_secs_f64()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::TimeControl;

    const WRITTEN: &str = "10+0.1";

    #[test]
    fn a_time_control_reads_as_seconds_plus_increment_and_prints_back_the_same() {
        let parsed: TimeControl = WRITTEN.parse().unwrap();
        assert_eq!(parsed.to_string(), WRITTEN);
        assert!("10".parse::<TimeControl>().is_err());
    }
}
