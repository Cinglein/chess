use syn::LifetimeParam;
use syn::visit::Visit;

use crate::source_file::SourceFile;

pub struct NamedLifetimes;

impl NamedLifetimes {
    const MIN_NAME_LENGTH: usize = 2;

    pub fn check(files: &[SourceFile]) -> Result<(), String> {
        let violations: Vec<String> = files
            .iter()
            .flat_map(|file| {
                Self::short_lifetimes(file.syntax())
                    .into_iter()
                    .map(move |(line, name)| format!("{}:{line}: '{name}", file.path()))
            })
            .collect();
        if violations.is_empty() {
            println!("every lifetime is named in {} rust files", files.len());
            Ok(())
        } else {
            Err(format!(
                "lifetimes are named with a word, never a letter:\n{}",
                violations.join("\n")
            ))
        }
    }

    fn short_lifetimes(file: &syn::File) -> Vec<(usize, String)> {
        let mut visitor = ShortLifetimes::default();
        visitor.visit_file(file);
        visitor.0
    }
}

#[derive(Default)]
struct ShortLifetimes(Vec<(usize, String)>);

impl<'ast> Visit<'ast> for ShortLifetimes {
    fn visit_lifetime_param(&mut self, parameter: &'ast LifetimeParam) {
        let name = parameter.lifetime.ident.to_string();
        if name.len() < NamedLifetimes::MIN_NAME_LENGTH {
            self.0.push((parameter.lifetime.span().start().line, name));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NamedLifetimes;

    const SOURCE: &str = "
struct Scan<'a, 'scan> { text: &'a str, path: &'scan str, owned: &'static str }
impl<'b> Scan<'b, '_> { fn view<'c>(&'c self) -> &'c str { self.text } }
";
    const FLAGGED: [&str; 3] = ["a", "b", "c"];

    #[test]
    fn flags_single_letter_lifetimes_where_they_are_declared() {
        let parsed = syn::parse_file(SOURCE).expect("valid rust");
        let names: Vec<String> = NamedLifetimes::short_lifetimes(&parsed)
            .into_iter()
            .map(|(_, name)| name)
            .collect();
        assert_eq!(names, FLAGGED);
    }
}
