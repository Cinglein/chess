use proptest::prelude::*;
use uci::{Elo, EngineOption, LineText, Switch};
const OTHER: EngineOption<'static> = EngineOption::Other {
    name: LineText::new("Hash"),
    setting: LineText::new("16"),
};

#[test]
fn typed_and_foreign_options_print_back_to_themselves() {
    proptest!(|(rating: u16, limited: bool)| {
        let switch = if limited { Switch::True } else { Switch::False };
        for option in [EngineOption::Elo(Elo::new(rating)), EngineOption::LimitStrength(switch), OTHER] {
            let line = option.to_string();
            prop_assert_eq!(EngineOption::try_from(line.as_str()), Ok(option));
        }
    });
}
