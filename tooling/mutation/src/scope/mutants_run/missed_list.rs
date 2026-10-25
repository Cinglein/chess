use std::fs;
use std::io::ErrorKind;
use std::path::Path;

use task::Failure;
use task::Site;

pub struct MissedList(Vec<String>);

impl MissedList {
    const FILE: &str = "mutants.out/missed.txt";

    pub fn in_output(output: &Path) -> Result<MissedList, Failure> {
        let path = output.join(Self::FILE);
        match fs::read_to_string(&path) {
            Ok(listed) => Ok(MissedList(
                listed
                    .lines()
                    .filter(|line| !line.trim().is_empty())
                    .map(str::to_owned)
                    .collect(),
            )),
            Err(error) if error.kind() == ErrorKind::NotFound => Ok(MissedList(Vec::new())),
            Err(error) => Err(Failure::Io {
                site: Site::File(path),
                error,
            }),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::MissedList;
    const FILE: &str = "mutants.out/missed.txt";

    #[test]
    fn a_missing_list_is_empty_and_a_written_one_counts_its_lines() {
        let output = std::env::temp_dir().join(format!("chess-missed-{}", std::process::id()));
        let absent = MissedList::in_output(&output).unwrap();
        std::fs::create_dir_all(output.join(FILE)).unwrap();
        let unreadable = MissedList::in_output(&output).is_err();
        std::fs::remove_dir(output.join(FILE)).unwrap();
        std::fs::write(output.join(FILE), "a.rs:1:1: delete x").unwrap();
        let present = MissedList::in_output(&output).unwrap();
        std::fs::remove_dir_all(&output).unwrap();
        assert!(absent.is_empty() && unreadable && !present.is_empty());
    }
}
