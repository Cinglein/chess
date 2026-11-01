mod time_control_error;

use std::str::FromStr;
use std::time::Duration;

use serde::Deserialize;
use time_control_error::TimeControlError;
use uci::Clock;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(try_from = "String")]
pub struct TimeControl {
    base: Duration,
    increment: Duration,
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
