mod bag_candidates;
mod name_uses;

use bag_candidates::BagCandidates;
use name_uses::NameUses;

use crate::task::report::Report;
use crate::task::site::Site;
use crate::task::source_file::SourceFile;
use crate::task::violation::Violation;

pub struct NoParameterBags;

impl NoParameterBags {
    pub fn report(files: &[SourceFile]) -> Report {
        let uses: Vec<NameUses> = files.iter().map(NameUses::in_file).collect();
        Report::new(
            "a struct with no logic of its own that one other file consumes is a parameter bag",
            files
                .iter()
                .enumerate()
                .flat_map(|(index, file)| {
                    let candidates = BagCandidates::in_file(file);
                    candidates
                        .candidates()
                        .filter(|candidate| {
                            Self::consumer_count(&uses, index, candidate.name()) == 1
                        })
                        .map(|candidate| {
                            Violation::new(
                                Site::Line(file.path().to_owned(), candidate.line()),
                                format!(
                                    "{} only carries values into one other file; give it the logic that consumes it",
                                    candidate.name()
                                ),
                            )
                        })
                        .collect::<Vec<Violation>>()
                })
                .collect(),
        )
    }

    fn consumer_count(uses: &[NameUses], own: usize, name: &str) -> usize {
        uses.iter()
            .enumerate()
            .filter(|(index, file_uses)| *index != own && file_uses.mentions(name))
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::NoParameterBags;
    use crate::task::source_file::SourceFile;

    const BAG: &str = "pub(crate) struct Bag { a: u8 } impl Bag { pub fn new(a: u8) -> Bag { Bag { a } } pub fn a(&self) -> u8 { self.a } }";
    const RECORD: &str = "pub(crate) struct Record { a: u8 } impl Record { pub fn new(a: u8) -> Record { Record { a } } pub fn a(&self) -> u8 { self.a } }";
    const WORKER: &str = "pub struct Worker; impl Worker { pub fn run(&self) -> u8 { Bag::new(1).a() + Record::new(2).a() } }";
    const READER: &str = "pub struct Reader; impl Reader { pub fn read(&self, record: Record) -> u8 { record.a() } }";
    const FILES: [(&str, &str); 4] = [
        ("crates/w/src/bag.rs", BAG),
        ("crates/w/src/record.rs", RECORD),
        ("crates/w/src/worker.rs", WORKER),
        ("crates/w/src/reader.rs", READER),
    ];

    #[test]
    fn flags_a_logic_free_struct_consumed_by_one_file_but_not_one_shared_by_two() {
        let files = FILES.map(|(path, text)| {
            SourceFile::parse(path.to_owned(), text.to_owned()).expect("valid rust")
        });
        let report = NoParameterBags::report(&files).to_string();
        assert!(
            report.contains("Bag only carries") && !report.contains("Record only"),
            "{report}"
        );
    }
}
