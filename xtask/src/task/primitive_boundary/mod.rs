mod number_sites;

use number_sites::NumberSites;

use crate::task::report::Report;
use crate::task::site::Site;
use crate::task::source_file::SourceFile;
use crate::task::violation::Violation;

pub struct PrimitiveBoundary;

impl PrimitiveBoundary {
    const BIT_BOUNDARY_FILES: [&str; 7] = [
        "crates/board/src/square.rs",
        "crates/board/src/direction.rs",
        "crates/board/src/slider/magic.rs",
        "crates/board/src/slider/magics.rs",
        "crates/board/src/slider/attack_table.rs",
        "crates/board/src/zobrist_keys/split_mix.rs",
        "xtask/src/task/magics/",
    ];

    pub fn report(files: &[SourceFile]) -> Report {
        Report::new(
            "primitives appear only at a type's boundary",
            files
                .iter()
                .filter(|file| !Self::is_bit_boundary(file.path()))
                .flat_map(|file| {
                    let sites = NumberSites::in_file(file.syntax());
                    let bare = if sites.is_newtype_file() {
                        Vec::new()
                    } else {
                        sites.found()
                    };
                    bare.into_iter().map(move |ident| {
                        Violation::new(
                            Site::Line(file.path().to_owned(), ident.span().start().line),
                            format!("bare {ident}; name what it counts with a newtype"),
                        )
                    })
                })
                .collect(),
        )
    }

    fn is_bit_boundary(path: &str) -> bool {
        Self::BIT_BOUNDARY_FILES
            .iter()
            .any(|boundary| path.starts_with(boundary))
    }
}

#[cfg(test)]
mod tests {
    use super::PrimitiveBoundary;
    use crate::task::source_file::SourceFile;

    const NEWTYPE: &str =
        "pub struct Depth(u8); impl Depth { pub fn plies(self) -> u8 { self.0 } }";
    const BARE: &str = "pub struct Limits { nodes: u64, index: usize } impl Limits { const CAP: u32 = 1; fn read(&self) -> Option<u64> { let x: u8 = 1; self.nodes.into() } }";
    const FILES: [(&str, &str); 2] = [
        ("crates/a/src/depth.rs", NEWTYPE),
        ("crates/a/src/limits.rs", BARE),
    ];
    const FLAGGED_FILE: &str = "limits.rs";
    const FLAGGED_SITES: usize = 3;
    const KEPT: [&str; 2] = ["depth.rs", "bare u8"];

    #[test]
    fn flags_declared_primitives_outside_newtype_files_but_not_usize_or_locals() {
        let files = FILES.map(|(path, text)| {
            SourceFile::parse(path.to_owned(), text.to_owned()).expect("valid rust")
        });
        let report = PrimitiveBoundary::report(&files).to_string();
        assert_eq!(
            report.matches(FLAGGED_FILE).count(),
            FLAGGED_SITES,
            "{report}"
        );
        assert!(KEPT.iter().all(|kept| !report.contains(kept)), "{report}");
    }
}
