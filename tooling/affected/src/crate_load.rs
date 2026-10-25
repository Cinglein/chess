use task::CrateName;
use task::Millis;
use task::{Interval, Policy};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CrateLoad {
    name: CrateName,
    cost: Millis,
}

impl CrateLoad {
    #[must_use]
    pub fn new(name: CrateName, mutants: usize, millis_each: Millis, policy: &Policy) -> CrateLoad {
        let cost = policy
            .interval(Interval::PerCrate)
            .plus(millis_each.times(mutants));
        CrateLoad { name, cost }
    }

    #[must_use]
    pub fn name(&self) -> &CrateName {
        &self.name
    }

    #[must_use]
    pub fn cost(&self) -> Millis {
        self.cost
    }
}
