use core::fmt::{self, Display};
use core::str::FromStr;

pub struct DashOr<T>(pub Option<T>);

impl<T: Display> Display for DashOr<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Some(value) => value.fmt(formatter),
            None => formatter.write_str("-"),
        }
    }
}

impl<T: FromStr> FromStr for DashOr<T> {
    type Err = T::Err;

    fn from_str(text: &str) -> Result<Self, T::Err> {
        match text {
            "-" => Ok(DashOr(None)),
            value => value.parse().map(|value| DashOr(Some(value))),
        }
    }
}
