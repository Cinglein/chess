use board::Color;
use core::time::Duration;
use go_limits::{Clock, Increment, Remaining};
use proptest::prelude::*;
#[test]
fn the_mover_loses_what_it_spent_and_gains_its_increment_while_the_other_clock_stands() {
    proptest!(|(clock: Clock, side: Color, spent_millis: u32)| {
        let spent = Duration::from_millis(u64::from(spent_millis));
        let after = clock.minus_spent_plus_increment(side, spent);
        prop_assert_eq!(after.time::<Remaining>(side), clock.time::<Remaining>(side).saturating_sub(spent) + clock.time::<Increment>(side));
        prop_assert_eq!(after.time::<Remaining>(!side), clock.time::<Remaining>(!side));
        prop_assert_eq!(after.time::<Increment>(side), clock.time::<Increment>(side));
    });
}
