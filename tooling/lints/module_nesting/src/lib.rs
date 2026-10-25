mod module_tree;

use module_tree::ModuleTree;

use task::Report;
use task::SourceFile;
use task::{Limit, Policy};

pub struct ModuleNesting;

impl ModuleNesting {
    #[must_use]
    pub fn report(files: &[SourceFile], policy: &Policy) -> Report {
        let tree = ModuleTree::collect(files);
        Report::new(
            "modules nest by use: a file used only inside one sibling belongs inside it",
            tree.violations()
                .chain(tree.crowded(policy.limit(Limit::Reexports).count()))
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use settings::EmbeddedDefaults;

    use super::ModuleNesting;
    use task::Policy;
    use task::SourceFile;

    const LIB: &str =
        "mod hub; mod spoke; mod rim; pub use hub::Hub; pub use rim::{A, B, C, D, E, F, G, H};";
    const HUB: &str = "use crate::spoke::Spoke; use crate::spoke::part::Part; pub struct Hub(Spoke, Part); pub use crate::rim::{A, B, C, D, E, F, G, H};";
    const SPOKE: &str = "mod part; use crate::rim::inner::Rim; pub struct Spoke(Rim);";
    const PART: &str = "pub struct Part;";
    const RIM: &str = "mod inner; mod axle; pub use inner::{Rim, A, B, C, D, E, F, G, H};";
    const AXLE: &str = "pub struct Axle;";
    const INNER: &str = "use super::axle::Axle; pub struct Rim(Axle);";
    const FILES: [(&str, &str); 7] = [
        ("crates/w/src/lib.rs", LIB),
        ("crates/w/src/hub.rs", HUB),
        ("crates/w/src/spoke.rs", SPOKE),
        ("crates/w/src/spoke/part.rs", PART),
        ("crates/w/src/rim/mod.rs", RIM),
        ("crates/w/src/rim/axle.rs", AXLE),
        ("crates/w/src/rim/inner.rs", INNER),
    ];
    const REPORTED: [&str; 4] = [
        "spoke.rs: used only from crate::hub; move it inside that module",
        "axle.rs: used only from crate::rim::inner; move it inside that module",
        "spoke.rs: reaches into crate::rim::inner",
        "rim/mod.rs: re-exports 9 items, at most 8 allowed",
    ];

    #[test]
    fn flags_modules_used_by_one_sibling_uses_past_a_mod_rs_and_crowded_reexports_outside_the_root()
    {
        let files = FILES.map(|(path, text)| {
            SourceFile::parse(path.to_owned(), text.to_owned()).expect("valid rust")
        });
        let report = ModuleNesting::report(&files, &Policy::defaults());
        assert!(report.lists_exactly(&REPORTED), "{report}");
    }
}
