mod bag_candidates;
mod name_uses;

use bag_candidates::BagCandidates;
use name_uses::NameUses;

use task::Report;
use task::Site;
use task::SourceFile;
use task::Violation;

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
                                Site::Line(file.path().into(), candidate.line()),
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
    use task::SourceFile;

    const BAG: &str = "
pub(crate) struct Bag { a: u8 } impl Bag { pub fn new(a: u8) -> Bag { Bag { a } } pub fn a(&self) -> u8 { self.a } pub fn again(self, a: u8) -> Bag { Bag { a, ..self } } }
mod inner { pub(crate) struct Crate { a: u8 } impl Crate { pub fn new(a: u8) -> Crate { Crate { a } } } }
#[cfg(test)]
mod tests { pub(crate) struct Probe { a: u8 } }
";
    const RECORD: &str = "pub(crate) struct Record { a: u8 } impl Record { pub fn new(a: u8) -> Record { Record { a } } pub fn a(&self) -> u8 { self.a } }";
    const WORKER: &str = "pub struct Worker; impl Worker { pub fn run(&self) -> u8 { Bag::new(1).a() + Record::new(2).a() + Crate::new(3).a } pub fn probe() -> Probe { Probe } }";
    const READER: &str = "pub struct Reader; impl Reader { pub fn read(&self, record: Record) -> u8 { record.a() } }";
    const WEIGHT: &str = "pub(crate) struct Weight { a: u8 } impl Weight { pub fn new(a: u8) -> Weight { Weight { a } } pub fn settings(&self) -> Settings { Settings { a: self.a } } }";
    const TALLY: &str = "pub(crate) struct Tally { a: u8 } impl Tally { pub fn new(a: u8) -> Tally { Tally { a } } pub fn first(pair: Pair) -> u8 { pair.a } }";
    const SCALE: &str = "pub struct Scale; impl Scale { pub fn run(&self) -> Settings { Weight::new(1).settings() } pub fn tally(&self) -> u8 { Tally::new(1).a } }";
    const FILES: [(&str, &str); 7] = [
        ("crates/w/src/bag.rs", BAG),
        ("crates/w/src/record.rs", RECORD),
        ("crates/w/src/worker.rs", WORKER),
        ("crates/w/src/reader.rs", READER),
        ("crates/w/src/weight.rs", WEIGHT),
        ("crates/w/src/tally.rs", TALLY),
        ("crates/w/src/scale.rs", SCALE),
    ];
    const REPORTED: [&str; 2] = ["bag.rs:2: Bag only carries", "bag.rs:3: Crate only carries"];

    #[test]
    fn flags_a_logic_free_struct_consumed_by_one_file_but_not_one_shared_or_one_with_logic() {
        let files = FILES.map(|(path, text)| {
            SourceFile::parse(path.to_owned(), text.to_owned()).expect("valid rust")
        });
        let report = NoParameterBags::report(&files);
        assert!(report.lists_exactly(&REPORTED), "{report}");
    }
}
