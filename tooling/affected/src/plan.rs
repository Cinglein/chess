use std::collections::{BTreeMap, BTreeSet};

use serde::Serialize;

use super::crate_load::CrateLoad;
use super::job::Job;
use super::job_plan::JobPlan;
use task::Policy;
use task::Selection;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Plan {
    jobs: BTreeMap<Job, JobPlan>,
}

impl Plan {
    #[must_use]
    pub fn new(jobs: BTreeMap<Job, JobPlan>) -> Plan {
        Plan { jobs }
    }

    #[must_use]
    pub fn job(&self, job: Job) -> Option<&JobPlan> {
        self.jobs.get(&job)
    }

    #[must_use]
    pub fn with_full_sharded(self, loads: &[CrateLoad], policy: &Policy) -> Plan {
        Plan {
            jobs: self
                .jobs
                .into_iter()
                .map(|(job, plan)| match job {
                    Job::Full => (job, plan.sharded(loads, policy)),
                    _ => (job, plan),
                })
                .collect(),
        }
    }

    pub fn selection(&self, job: Job) -> Selection {
        self.jobs.get(&job).map_or_else(
            || Selection::Crates(BTreeSet::default()),
            JobPlan::selection,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{Job, JobPlan, Plan};
    use crate::crate_load::CrateLoad;
    use settings::EmbeddedDefaults;
    use std::collections::{BTreeMap, BTreeSet};
    use task::{Interval, MutantCount, Policy};
    const ONE: usize = 1;
    const EVAL: &str = "eval";
    const SHARDED: &str = r#""shards":["eval/0/2","eval/1/2"],"shard_count":2"#;
    const UNSHARDED: &str = r#""shards":[],"shard_count":0"#;

    #[test]
    fn only_the_full_job_is_sharded_and_a_missing_job_has_no_plan() {
        let policy = Policy::defaults();
        let (full, test, lint) = (Job::Full, Job::Test, Job::Lint);
        let eval = BTreeSet::from([EVAL.parse().unwrap()]);
        let jobs = BTreeMap::from([(full, JobPlan::over(&eval)), (test, JobPlan::over(&eval))]);
        let each = policy.interval(Interval::PerShard);
        let load = CrateLoad::new(EVAL.parse().unwrap(), MutantCount::from(ONE), each, &policy);
        let sharded = Plan::new(jobs).with_full_sharded(core::slice::from_ref(&load), &policy);
        let json = serde_json::to_string(&sharded).unwrap();
        assert!(sharded.job(lint).is_none() && sharded.job(test).is_some());
        assert!(json.contains(SHARDED) && json.contains(UNSHARDED));
    }
}
