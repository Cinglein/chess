use fen::DashOr;
use proptest::prelude::any;

laws::roundtrip!(
    a_dash_or_a_value_prints_and_parses_back_to_itself,
    any::<Option<u8>>(),
    |value: &Option<u8>| DashOr::from(*value).to_string(),
    |text: &String| text.parse::<DashOr<u8>>().map(Option::from),
    Ok
);
