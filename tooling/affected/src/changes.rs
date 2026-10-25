use crate::reach::Reach;
use task::Failure;
use task::Workspace;
use task::{GitArgument, Program, Stdout};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Changes {
    SinceMain,
    Everything,
}

impl Changes {
    const BASE: &str = "origin/main";

    pub fn reaches(self, workspace: &Workspace) -> Result<Vec<Reach>, Failure> {
        match self {
            Changes::Everything => Ok(vec![Reach::Everything]),
            Changes::SinceMain => Ok(workspace
                .line(Program::Git)
                .argument(GitArgument::Diff)
                .argument(GitArgument::NameOnly)
                .argument(GitArgument::MergeBase)
                .argument(Self::BASE)
                .captured::<Stdout>()?
                .lines()
                .map(Reach::reached_by)
                .collect()),
        }
    }
}
