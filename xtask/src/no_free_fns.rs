use syn::{Item, ItemFn, ItemMod};

use crate::source_file::SourceFile;

pub struct NoFreeFns;

impl NoFreeFns {
    pub fn check(files: &[SourceFile]) -> Result<(), String> {
        let violations: Vec<String> = files
            .iter()
            .flat_map(|file| {
                Self::free_fns(&file.syntax().items)
                    .into_iter()
                    .map(move |(line, name)| format!("{}:{line}: fn {name}", file.path()))
            })
            .collect();
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
                Item::Fn(function) if !Self::is_exempt(function) => vec![(
                    function.sig.ident.span().start().line,
                    function.sig.ident.to_string(),
                )],
                Item::Mod(module) => Self::nested(module),
                _ => Vec::new(),
            })
            .collect()
    }

    fn nested(module: &ItemMod) -> Vec<(usize, String)> {
        module
            .content
            .as_ref()
            .map_or_else(Vec::new, |(_, items)| Self::free_fns(items))
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

    const SOURCE: &str = "
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
    const FLAGGED: [&str; 3] = ["helper", "nested", "test_helper"];

    #[test]
    fn flags_module_level_functions_but_not_methods_main_or_tests() {
        let parsed = syn::parse_file(SOURCE).expect("valid rust");
        let names: Vec<String> = NoFreeFns::free_fns(&parsed.items)
            .into_iter()
            .map(|(_, name)| name)
            .collect();
        assert_eq!(names, FLAGGED);
    }
}
