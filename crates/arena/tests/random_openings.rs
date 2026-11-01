use arena::{RandomOpenings, Seed};
use board::PlyCount;
const SEED: Seed = Seed::new(7);
const OTHER_SEED: Seed = Seed::new(8);
const PLIES: PlyCount = PlyCount::new(8);

#[test]
fn a_seed_reproduces_its_openings_another_seed_differs_and_each_has_walked_the_asked_plies() {
    let first = RandomOpenings::seeded(SEED, PLIES).next().unwrap();
    let again = RandomOpenings::seeded(SEED, PLIES).next().unwrap();
    assert_eq!(first, again);
    assert_ne!(
        RandomOpenings::seeded(OTHER_SEED, PLIES).next(),
        Some(first)
    );
    assert_eq!(first.fullmove_number().to_string(), "5");
}
