use affected::{Affected, Changes};
use task::{Branch, Failure, Policy, Selection, Workspace};

pub struct Scope;

impl Scope {
    const AFFECTED_FLAG: &str = "--affected";

    pub fn selection(workspace: &Workspace, flags: &[String]) -> Result<Selection, Failure> {
        let changes = flags.iter().try_fold(Changes::Everything, |_, flag| {
            if flag == Self::AFFECTED_FLAG {
                Ok(Changes::SinceMain)
            } else {
                Err(Failure::UnknownFlag {
                    flag: flag.clone(),
                    known: Self::AFFECTED_FLAG,
                })
            }
        })?;
        Affected::selection(
            workspace,
            changes,
            Policy::read(workspace.root())?.branch(Branch::Base),
        )
    }
}
