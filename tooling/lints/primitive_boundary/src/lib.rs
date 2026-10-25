mod number_sites;
mod text_sites;

use number_sites::NumberSites;
use text_sites::TextSites;

use task::Report;
use task::SourceFile;

pub struct PrimitiveBoundary;

impl PrimitiveBoundary {
    #[must_use]
    pub fn report(files: &[SourceFile]) -> Report {
        Report::new(
            "primitives appear only at a type's boundary",
            files
                .iter()
                .flat_map(|file| {
                    NumberSites::violations(file)
                        .into_iter()
                        .chain(TextSites::violations(file))
                })
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::PrimitiveBoundary;
    use task::SourceFile;

    const BARE: &str = "
pub struct Limits { nodes: u64, index: usize, rounds: usize } pub enum Kind { Depth(u8) }
impl Limits { const CAP: u32 = 1; fn read(&self) -> Option<u64> { let x: u8 = 1; self.nodes.into() } }
const LIMIT: u64 = 1; pub trait Bound { const MOST: u8; } pub type Count = u32;
mod inner { pub fn depth() -> u16 { 1 } }
#[cfg(test)] mod tests { pub fn depth() -> u16 { 1 } }
";
    const NEWTYPE: &str =
        "pub struct Depth(u8); impl Depth { pub fn plies(self) -> u8 { self.0 } }";
    const FILES: [(&str, &str); 3] = [
        ("crates/a/src/depth.rs", NEWTYPE),
        ("crates/a/src/limits.rs", BARE),
        ("crates/a/src/pair.rs", "\npub struct Pair(u8, u8);"),
    ];
    const REPORTED: [&str; 11] = [
        "limits.rs:2: bare u64 in nodes", "limits.rs:2: bare u8 in Kind",
        "limits.rs:3: bare u32 in CAP", "limits.rs:3: bare u64 in read",
        "limits.rs:4: bare u64 in LIMIT", "limits.rs:4: bare u8 in MOST",
        "limits.rs:4: bare u32 in Count", "limits.rs:5: bare u16 in depth",
        "limits.rs:2: usize in rounds", "pair.rs:2: bare u8 in Pair", "pair.rs:2: bare u8 in Pair",
    ];
    const DOOR: &str = "pub struct Word; impl core::str::FromStr for Word { type Err = (); fn from_str(text: &str) -> Result<Word, ()> { text.trim().parse() } } impl Word { fn shout(&self) -> String { format!(\"{self}\") } }";
    const ROOM: &str = "
pub struct Room;
impl Room { fn label(&self, text: &str) -> bool { let kept = text.to_string(); text.trim().is_empty() } }
impl Room { fn greet(&self) -> String { let fresh = String::new(); let parts = Vec::from(fresh); format!(\"{fresh}\") } fn copy(&self) -> String { String::from(\"room\") } }
mod inner { fn shout(text: &str) -> String { text.trim().to_owned() } }
#[cfg(test)] mod tests { fn shout(text: &str) -> String { text.trim().to_owned() } }
";
    const TEXT_FILES: [(&str, &str); 2] = [
        ("crates/a/src/word.rs", DOOR),
        ("crates/a/src/room.rs", ROOM),
    ];
    const TEXT_REPORTED: [&str; 5] = [
        "room.rs:3: to_string handles", "room.rs:3: trim handles", "room.rs:4: format handles",
        "room.rs:4: from handles", "room.rs:5: trim handles",
    ];

    #[test]
    fn flags_declared_primitives_and_text_handling_but_not_newtypes_doors_indices_locals_or_tests()
    {
        let files = FILES.map(|(path, text)| SourceFile::parse(path, text.to_owned()).unwrap());
        let texts =
            TEXT_FILES.map(|(path, text)| SourceFile::parse(path, text.to_owned()).unwrap());
        let report = PrimitiveBoundary::report(&files);
        let text_report = PrimitiveBoundary::report(&texts);
        assert!(report.lists_exactly(&REPORTED), "{report}");
        assert!(text_report.lists_exactly(&TEXT_REPORTED), "{text_report}");
    }
}
