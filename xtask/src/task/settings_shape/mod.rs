mod deserialized_struct;
mod keys;
mod schema_scan;

use std::collections::BTreeSet;

use deserialized_struct::DeserializedStruct;
use keys::Keys;
use schema_scan::SchemaScan;

use task::Report;
use task::Site;
use task::SourceFile;
use task::Violation;

pub struct SettingsShape;

impl SettingsShape {
    pub fn report(files: &[SourceFile]) -> Report {
        let scans: Vec<SchemaScan> = files.iter().map(SchemaScan::in_file).collect();
        let structs: Vec<&DeserializedStruct> =
            scans.iter().flat_map(SchemaScan::structs).collect();
        let owned = Self::owned(
            scans.iter().flat_map(SchemaScan::seeds).cloned().collect(),
            &structs,
        );
        Report::new(
            "a settings struct refuses keys it does not know",
            structs
                .iter()
                .filter(|item| owned.contains(item.name()) && item.keys() == Keys::Open)
                .map(|item| {
                    Violation::new(
                        Site::Line(item.path().into(), item.line()),
                        "struct is read from a settings file we write but lacks #[serde(deny_unknown_fields)]; a misspelt key must be an error, not a silent default",
                    )
                })
                .collect(),
        )
    }

    fn owned(seeds: BTreeSet<String>, structs: &[&DeserializedStruct]) -> BTreeSet<String> {
        let grown: BTreeSet<String> = structs
            .iter()
            .filter(|item| seeds.contains(item.name()))
            .flat_map(|item| item.field_types().iter().cloned())
            .chain(seeds.iter().cloned())
            .collect();
        if grown == seeds {
            seeds
        } else {
            Self::owned(grown, structs)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SettingsShape;
    use task::SourceFile;

    const SOURCE: &str = "
        #[derive(Deserialize)]
        pub struct Settings { schedule: Schedule, rounds: usize }
        impl EmbeddedDefaults for Settings { const DEFAULTS: &str = \"\"; }
        #[derive(Deserialize)]
        pub struct Schedule { batches: usize }
        #[derive(Deserialize)]
        pub struct Metadata { packages: Vec<Package> }
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        pub struct Strict { rounds: usize }
        impl EmbeddedDefaults for Strict { const DEFAULTS: &str = \"\"; }
    ";
    const PATH: &str = "crates/a/src/x.rs";
    const REPORTED: [&str; 2] = [":3:", ":6:"];
    const CLEAN: [&str; 2] = [":8:", ":11:"];

    #[test]
    fn flags_structs_reachable_from_embedded_defaults_that_do_not_deny_unknown_keys() {
        let file = SourceFile::parse(PATH.to_owned(), SOURCE.to_owned()).expect("valid rust");
        let report = SettingsShape::report(&[file]).to_string();
        assert!(
            REPORTED.iter().all(|line| report.contains(line))
                && CLEAN.iter().all(|line| !report.contains(line)),
            "{report}"
        );
    }
}
