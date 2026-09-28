mod export;
mod foreign_references;

use export::Export;
use foreign_references::ForeignReferences;

use crate::task::report::Report;
use crate::task::site::Site;
use crate::task::source_file::SourceFile;
use crate::task::violation::Violation;

pub struct PublicSurface;

impl PublicSurface {
    pub fn report(files: &[SourceFile]) -> Report {
        let references = files
            .iter()
            .map(ForeignReferences::in_file)
            .fold(ForeignReferences::default(), ForeignReferences::absorb);
        Report::new(
            "a crate exports exactly what another crate uses",
            files
                .iter()
                .flat_map(|file| {
                    Export::in_root(file)
                        .into_iter()
                        .filter(|export| {
                            references.depends_on(export.crate_name()) && !references.names(export)
                        })
                        .map(move |export| {
                            Violation::new(
                                Site::Line(file.path().to_owned(), export.line()),
                                format!(
                                    "{} is exported but no other crate names it; export it when a user appears",
                                    export.name()
                                ),
                            )
                        })
                })
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::PublicSurface;
    use crate::task::source_file::SourceFile;

    const LIB: &str = "mod x; pub use x::{Used, Unused}; pub use x::Original as Seen;";
    const MAIN: &str = "use a::Used; fn main() { a::Seen::go(); }";
    const LONELY: &str = "mod y; pub use y::Lonely;";
    const FILES: [(&str, &str); 3] = [
        ("crates/a/src/lib.rs", LIB),
        ("crates/b/src/main.rs", MAIN),
        ("crates/c/src/lib.rs", LONELY),
    ];
    const FLAGGED: &str = "Unused is exported";
    const KEPT: [&str; 3] = ["Used is exported", "Seen", "Lonely"];

    #[test]
    fn flags_an_unnamed_reexport_of_a_crate_that_has_a_dependent() {
        let files = FILES.map(|(path, text)| {
            SourceFile::parse(path.to_owned(), text.to_owned()).expect("valid rust")
        });
        let report = PublicSurface::report(&files).to_string();
        assert!(
            report.contains(FLAGGED) && KEPT.iter().all(|kept| !report.contains(kept)),
            "{report}"
        );
    }
}
