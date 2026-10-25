mod edge_policy;
mod fn_shape_policy;
mod millis;
mod most;
mod mutation_policy;
mod name_policy;
mod nesting_policy;
mod private_fn_policy;
mod test_policy;
mod time_policy;

pub use edge_policy::EdgePolicy;
pub use fn_shape_policy::FnShapePolicy;
pub use mutation_policy::MutationPolicy;
pub use name_policy::NamePolicy;
pub use nesting_policy::NestingPolicy;
pub use private_fn_policy::PrivateFnPolicy;
pub use test_policy::TestPolicy;
pub use time_policy::TimePolicy;

use serde::{Deserialize, Serialize};
use settings::EmbeddedDefaults;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Policy {
    fn_shape: FnShapePolicy,
    names: NamePolicy,
    module_nesting: NestingPolicy,
    private_fns: PrivateFnPolicy,
    state_graph: EdgePolicy,
    test_budget: TestPolicy,
    test_time: TimePolicy,
    mutants: MutationPolicy,
}

impl EmbeddedDefaults for Policy {
    const DEFAULTS: &str = include_str!("../../../policy.toml");
}

impl Policy {
    pub fn fn_shape(&self) -> &FnShapePolicy {
        &self.fn_shape
    }

    pub fn names(&self) -> &NamePolicy {
        &self.names
    }

    pub fn module_nesting(&self) -> &NestingPolicy {
        &self.module_nesting
    }

    pub fn private_fns(&self) -> &PrivateFnPolicy {
        &self.private_fns
    }

    pub fn state_graph(&self) -> &EdgePolicy {
        &self.state_graph
    }

    pub fn test_budget(&self) -> &TestPolicy {
        &self.test_budget
    }

    pub fn test_time(&self) -> &TimePolicy {
        &self.test_time
    }

    pub fn mutants(&self) -> &MutationPolicy {
        &self.mutants
    }
}

#[cfg(test)]
mod tests {
    use settings::EmbeddedDefaults;

    use super::Policy;

    #[test]
    fn the_checked_in_policy_parses_and_survives_a_trip_through_toml() {
        let written = toml::to_string_pretty(&Policy::defaults()).unwrap();
        let read: Policy = toml::from_str(&written).unwrap();
        assert_eq!(read, Policy::defaults());
    }
}
