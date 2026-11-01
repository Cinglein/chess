use board::Incremented;
use proptest::prelude::*;
use search::Depth;
#[test]
fn a_depth_reads_back_its_plies_and_a_step_down_then_up_returns_to_itself() {
    proptest!(|(plies: u8)| {
        let depth = Depth::new(plies);
        prop_assert_eq!(
            (depth.plies(), depth.to_string().parse(), depth.decremented().map(Depth::incremented)),
            (plies, Ok(depth), (plies > 0).then_some(depth))
        );
    });
}
