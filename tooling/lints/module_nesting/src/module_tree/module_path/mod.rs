mod ancestor;
mod inside_sibling;
mod relation;
mod same;

pub use ancestor::Ancestor;
pub use inside_sibling::InsideSibling;
pub use relation::Relation;
pub use same::Same;

use std::fmt;
use std::path::Path;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ModulePath {
    crate_root: String,
    segments: Vec<String>,
    file: String,
}

impl ModulePath {
    const SOURCE_ROOTS: [&str; 2] = ["src/lib.rs", "src/main.rs"];

    pub fn of_file(path: &Path) -> Option<ModulePath> {
        let file = path.to_string_lossy();
        let (crate_root, inside) = file.split_once("/src/")?;
        let inside = Path::new(inside);
        let stem = inside.file_stem()?.to_string_lossy().into_owned();
        let segments = inside
            .parent()
            .into_iter()
            .flat_map(Path::components)
            .map(|component| component.as_os_str().to_string_lossy().into_owned())
            .chain((!Self::is_crate_root(&file) && stem != "mod").then_some(stem))
            .collect();
        Some(ModulePath {
            crate_root: crate_root.to_owned(),
            segments,
            file: file.into_owned(),
        })
    }

    pub fn from_use(
        file: &ModulePath,
        path: &[String],
        modules: &[ModulePath],
    ) -> Option<ModulePath> {
        let first = path.first()?.as_str();
        let start = match first {
            "crate" => Vec::new(),
            "super" => file.parent()?.segments,
            _ => file.segments.clone(),
        };
        let skip = usize::from(matches!(first, "crate" | "super" | "self"));
        (skip != 0
            || modules
                .iter()
                .any(|module| module.relates::<Same>(&file.child(first))))
        .then_some(())?;
        let segments =
            path.iter()
                .skip(skip)
                .try_fold(start, |mut segments, segment| match segment.as_str() {
                    "super" => segments.pop().and(Some(segments)),
                    other => {
                        segments.push(other.to_owned());
                        Some(segments)
                    }
                })?;
        Some(ModulePath {
            crate_root: file.crate_root.clone(),
            segments,
            file: String::new(),
        })
    }

    pub fn deepest_module_containing<'tree>(
        modules: &'tree [ModulePath],
        target: &ModulePath,
        user: &ModulePath,
    ) -> Option<&'tree ModulePath> {
        modules
            .iter()
            .filter(|module| module.relates::<Ancestor>(target) && !module.relates::<Same>(user))
            .max_by_key(|module| module.segments.len())
    }

    pub fn file(&self) -> &str {
        &self.file
    }

    pub fn segments(&self) -> &[String] {
        &self.segments
    }

    pub fn parent(&self) -> Option<ModulePath> {
        let (_, parents) = self.segments.split_last()?;
        Some(ModulePath {
            crate_root: self.crate_root.clone(),
            segments: parents.to_vec(),
            file: String::new(),
        })
    }

    pub fn relates<R: Relation>(&self, other: &ModulePath) -> bool {
        R::holds(self, other)
    }

    pub fn crosses_directory_boundary(&self, user: &ModulePath, modules: &[ModulePath]) -> bool {
        (1..self.segments.len()).any(|cut| {
            let boundary = self.truncated(cut);
            modules
                .iter()
                .any(|module| module.relates::<Same>(&boundary) && module.file.ends_with("/mod.rs"))
                && !boundary.relates::<Ancestor>(user)
        })
    }

    pub fn truncated(&self, depth: usize) -> ModulePath {
        ModulePath {
            crate_root: self.crate_root.clone(),
            segments: self.segments.iter().take(depth).cloned().collect(),
            file: String::new(),
        }
    }

    fn child(&self, name: &str) -> ModulePath {
        ModulePath {
            crate_root: self.crate_root.clone(),
            segments: self
                .segments
                .iter()
                .cloned()
                .chain(Some(name.to_owned()))
                .collect(),
            file: String::new(),
        }
    }

    fn is_crate_root(file: &str) -> bool {
        Self::SOURCE_ROOTS.iter().any(|root| file.ends_with(root))
    }
}

impl fmt::Display for ModulePath {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "crate")?;
        self.segments
            .iter()
            .try_for_each(|segment| write!(formatter, "::{segment}"))
    }
}
