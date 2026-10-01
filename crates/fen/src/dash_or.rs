use core::fmt::{self, Display};
use core::str::FromStr;

pub enum DashOr<T> {
    Dash,
    Value(T),
}

impl<T> From<Option<T>> for DashOr<T> {
    fn from(option: Option<T>) -> Self {
        option.map_or(DashOr::Dash, DashOr::Value)
    }
}

impl<T> From<DashOr<T>> for Option<T> {
    fn from(field: DashOr<T>) -> Self {
        match field {
            DashOr::Dash => None,
            DashOr::Value(value) => Some(value),
        }
    }
}

impl<T: Display> Display for DashOr<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DashOr::Dash => formatter.write_str("-"),
            DashOr::Value(value) => value.fmt(formatter),
        }
    }
}

impl<T: FromStr> FromStr for DashOr<T> {
    type Err = T::Err;

    fn from_str(text: &str) -> Result<Self, T::Err> {
        match text {
            "-" => Ok(DashOr::Dash),
            value => value.parse().map(DashOr::Value),
        }
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::DashOr;

    #[test]
    fn an_absent_value_is_a_dash_and_a_present_one_is_itself_in_both_directions() {
        proptest!(|(value: Option<u8>)| {
            let field = DashOr::from(value).to_string();
            prop_assert_eq!(field == "-", value.is_none());
            prop_assert_eq!(field.parse::<DashOr<u8>>().map(Option::from), Ok(value));
        });
    }
}
