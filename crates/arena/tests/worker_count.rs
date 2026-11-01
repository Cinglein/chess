use arena::WorkerCount;

const THREE: usize = 3;
const NONE: usize = 0;
const ONE: usize = 1;

#[test]
fn a_worker_count_keeps_its_value_but_never_falls_below_one() {
    assert_eq!(
        (
            WorkerCount::from(THREE).at_least_one(),
            WorkerCount::from(NONE).at_least_one()
        ),
        (THREE, ONE)
    );
    assert!(WorkerCount::one_per_core() >= WorkerCount::from(ONE));
}
