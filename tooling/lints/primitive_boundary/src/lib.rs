mod finding;
mod number_sites;
mod text_sites;

use number_sites::NumberSites;
use text_sites::TextSites;

use task::Policy;
use task::Report;
use task::SourceFile;

pub struct PrimitiveBoundary;

impl PrimitiveBoundary {
    #[must_use]
    pub fn report(files: &[SourceFile], policy: &Policy) -> Report {
        Report::new(
            "primitives appear only at a type's boundary",
            files
                .iter()
                .flat_map(|file| {
                    NumberSites::violations(file, policy)
                        .into_iter()
                        .chain(TextSites::violations(file))
                })
                .collect(),
        )
    }
}
