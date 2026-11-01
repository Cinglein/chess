use std::collections::BTreeSet;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

pub(super) struct Missed(BTreeSet<String>);

impl Missed {
    const FILE: &'static str = "missed.txt";

    pub(super) fn under(folder: &Path) -> Self {
        Self(
            Self::files(folder)
                .iter()
                .filter_map(|path| fs::read_to_string(path).ok())
                .flat_map(|text| text.lines().map(str::to_owned).collect::<Vec<_>>())
                .collect(),
        )
    }

    pub(super) fn any(&self) -> Option<()> {
        (!self.0.is_empty()).then_some(())
    }

    fn files(folder: &Path) -> Vec<PathBuf> {
        fs::read_dir(folder)
            .into_iter()
            .flatten()
            .flatten()
            .map(|entry| entry.path())
            .flat_map(|path| {
                if path.is_dir() {
                    Self::files(&path)
                } else {
                    vec![path]
                }
            })
            .filter(|path| path.file_name().is_some_and(|name| name == Self::FILE))
            .collect()
    }
}

impl fmt::Display for Missed {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.iter().try_for_each(|line| writeln!(out, "{line}"))
    }
}
