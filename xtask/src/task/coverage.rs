use affected::{Affected, Changes, Job, Plan};
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
                Affected::plan(workspace, Changes::SinceMain).map(Coverage::Planned)
            } else {
                Err(Failure::UnknownFlag {
                    flag,
                    known: Self::AFFECTED_FLAG,
                })
            }
        })
    }

    pub fn selection(&self, job: Job) -> Selection {
        match self {
            Coverage::Everything => Selection::Workspace,
            Coverage::Planned(plan) => plan.selection(job),
        }
    }

    pub fn plan(self, workspace: &Workspace) -> Result<Plan, Failure> {
        match self {
            Coverage::Everything => Affected::plan(workspace, Changes::Everything),
            Coverage::Planned(plan) => Ok(plan),
        }
    }
}
