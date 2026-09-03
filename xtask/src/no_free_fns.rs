use std::fs;

use syn::{Item, ItemFn};

use crate::workspace::Workspace;

pub struct NoFreeFns;

impl NoFreeFns {
    pub fn check(workspace: &Workspace) -> Result<(), String> {
        let files = workspace.rust_files()?;
        let mut violations = Vec::new();
        for file in &files {
            let source =
                fs::read_to_string(file).map_err(|error| format!("{}: {error}", file.display()))?;
            let parsed =
                syn::parse_file(&source).map_err(|error| format!("{}: {error}", file.display()))?;
            let relative = workspace.relative(file);
            violations.extend(
                Self::free_fns(&parsed.items)
                    .into_iter()
                    .map(|(line, name)| format!("{relative}:{line}: fn {name}")),
            );
        }
        if violations.is_empty() {
            println!("no free functions found in {} rust files", files.len());
            Ok(())
        } else {
            Err(format!(
                "free functions are not allowed; make them associated functions or methods:\n{}",
                violations.join("\n")
            ))
        }
    }

    fn free_fns(items: &[Item]) -> Vec<(usize, String)> {
        items
            .iter()
            .flat_map(|item| match item {
                Item::Fn(function) if !Self::is_exempt(function) => {
                    let name = function.sig.ident.to_string();
                    vec![(function.sig.ident.span().start().line, name)]
                }
                Item::Mod(module) => module
                    .content
                    .as_ref()
                    .map_or_else(Vec::new, |(_, items)| Self::free_fns(items)),
                _ => Vec::new(),
            })
            .collect()
    }

    fn is_exempt(function: &ItemFn) -> bool {
        function.sig.ident == "main"
            || function
                .attrs
                .iter()
                .any(|attribute| attribute.path().is_ident("test"))
    }
}

#[cfg(test)]
mod tests {
    use super::NoFreeFns;

    #[test]
    fn flags_module_level_functions_but_not_methods_main_or_tests() {
        let source = "
fn main() {}
fn helper() {}
struct S;
impl S { fn method(&self) {} fn associated() {} }
trait T { fn required(); fn provided() {} }
mod inner {
    fn nested() {}
    #[cfg(test)]
    mod tests {
        #[test]
        fn a_test() {}
        fn test_helper() {}
    }
}
";
        let parsed = syn::parse_file(source).expect("valid rust");
        let names: Vec<String> = NoFreeFns::free_fns(&parsed.items)
            .into_iter()
            .map(|(_, name)| name)
            .collect();
        assert_eq!(names, ["helper", "nested", "test_helper"]);
    }
}
