use task::CrateName;
use task::Millis;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CrateLoad {
    name: CrateName,
    tested_packages: usize,
    cost: Millis,
}

impl CrateLoad {
    #[must_use]
    pub fn new(
        name: CrateName,
        mutants: usize,
        tested_packages: usize,
        millis_each: Millis,
    ) -> CrateLoad {
        CrateLoad {
            name,
            tested_packages,
            cost: millis_each.times(mutants),
        }
    }

    #[must_use]
    pub fn name(&self) -> &CrateName {
        &self.name
    }

    #[must_use]
    pub fn tested_packages(&self) -> usize {
        self.tested_packages
    }

    #[must_use]
    pub fn cost(&self) -> Millis {
        self.cost
    }
}
