use std::collections::{BTreeMap, BTreeSet};

use serde::Serialize;

use super::job::Job;
use super::job_plan::JobPlan;
use crate::task::policy::Policy;
use crate::task::selection::Selection;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Plan {
    jobs: BTreeMap<Job, JobPlan>,
}

impl Plan {
    pub fn new(jobs: BTreeMap<Job, JobPlan>) -> Plan {
        Plan { jobs }
    }

    pub fn job(&self, job: Job) -> Option<&JobPlan> {
        self.jobs.get(&job)
    }

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
