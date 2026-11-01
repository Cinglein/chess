use arena::WorkerCount;
const THREE: WorkerCount = WorkerCount::new(3);
const NONE: WorkerCount = WorkerCount::new(0);
const ONE: WorkerCount = WorkerCount::new(1);

#[test]
fn a_worker_count_keeps_its_value_but_never_falls_below_one() {
    assert_eq!((THREE.at_least_one(), NONE.at_least_one()), (3, 1));
    assert!(WorkerCount::one_per_core() >= ONE);
}
