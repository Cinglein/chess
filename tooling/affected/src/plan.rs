use std::collections::{BTreeMap, BTreeSet};

use serde::Serialize;

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
    pub fn with_full_sharded(self, mutants: usize, policy: &Policy) -> Plan {
        Plan {
            jobs: self
                .jobs
                .into_iter()
                .map(|(job, plan)| match job {
                    Job::Full => (job, plan.sharded(mutants, policy)),
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
