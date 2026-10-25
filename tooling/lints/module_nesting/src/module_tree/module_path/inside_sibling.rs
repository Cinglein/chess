use super::ModulePath;
use super::ancestor::Ancestor;
use super::relation::Relation;
use super::same::Same;

pub struct InsideSibling;

impl Relation for InsideSibling {
    fn holds(this: &ModulePath, module: &ModulePath) -> bool {
        let depth = module.segments.len();
        module
            .parent()
            .is_some_and(|parent| parent.relates::<Ancestor>(this))
            && this.segments.len() >= depth
            && !this.truncated(depth).relates::<Same>(module)
    }
}
