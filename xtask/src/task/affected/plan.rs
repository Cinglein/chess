use std::collections::{BTreeMap, BTreeSet};

use serde::Serialize;

use super::job::Job;
use super::job_plan::JobPlan;
use crate::task::selection::Selection;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Plan {
    jobs: BTreeMap<Job, JobPlan>,
}

impl Plan {
    pub fn new(jobs: BTreeMap<Job, JobPlan>) -> Plan {
        Plan { jobs }
    }

    pub fn selection(&self, job: Job) -> Selection {
        self.jobs.get(&job).map_or_else(
            || Selection::Crates(BTreeSet::default()),
            JobPlan::selection,
        )
    }
}
