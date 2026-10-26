use settings::EmbeddedDefaults;
use strum::IntoEnumIterator;

use corpus_check::LintName;
use task::Failure;
use task::Policy;
use task::Report;
use task::Workspace;

pub struct Lint;

impl Lint {
    pub fn run(workspace: &Workspace) -> Result<(), Failure> {
        let files = workspace.source_files()?;
        let policy = Policy::defaults();
        let reports: Vec<Report> = LintName::iter()
            .map(|lint| lint.report(&files, &policy))
            .filter(|report| !report.is_clean())
            .collect();
        if reports.is_empty() {
            println!("lint: {} rust files clean", files.len());
            Ok(())
        } else {
            Err(Failure::Lint(reports))
        }
    }
}
