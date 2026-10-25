use crate::task::affected::{Affected, Job, Plan};
use task::Failure;
use task::Selection;
use task::Workspace;

pub enum Coverage {
    Everything,
    Planned(Plan),
}

impl Coverage {
    const AFFECTED_FLAG: &str = "--affected";

    pub fn from_flags(
        mut flags: impl Iterator<Item = String>,
        workspace: &Workspace,
    ) -> Result<Coverage, Failure> {
        flags.try_fold(Coverage::Everything, |_, flag| {
            if flag == Self::AFFECTED_FLAG {
                Affected::plan(workspace).map(Coverage::Planned)
            } else {
                Err(Failure::Usage(format!(
                    "unknown flag {flag}; the only flag is {}",
                    Self::AFFECTED_FLAG
                )))
            }
        })
    }

    pub fn selection(&self, job: Job) -> Selection {
        match self {
            Coverage::Everything => Selection::Workspace,
            Coverage::Planned(plan) => plan.selection(job),
        }
    }
}
