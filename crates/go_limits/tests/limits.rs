use go_limits::GoLimits;
use proptest::prelude::*;
#[test]
fn limits_print_back_to_themselves_and_expose_exactly_their_own_kind() {
    proptest!(|(limits: GoLimits)| {
        prop_assert_eq!(GoLimits::from(limits.to_string().as_str()), limits);
        let exposed = [limits.depth().is_some(), limits.nodes().is_some(), limits.move_time().is_some(), limits.clock().is_some()];
        prop_assert_eq!(exposed.iter().filter(|kind| **kind).count(), usize::from(limits != GoLimits::Infinite));
    });
}
