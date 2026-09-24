use syn::visit::Visit;
use syn::{Ident, LifetimeParam};

use crate::report::Report;
use crate::site::Site;
use crate::source_file::SourceFile;
use crate::violation::Violation;

pub struct NamedLifetimes;

impl NamedLifetimes {
    const MIN_NAME_LENGTH: usize = 2;

    pub fn report(files: &[SourceFile]) -> Report {
        Report::new(
            "lifetimes are named with a word, never a letter",
            files
                .iter()
                .flat_map(|file| {
                    Self::short_lifetimes(file.syntax())
                        .into_iter()
                        .map(move |name| {
                            Violation::new(
                                Site::Line(file.path().to_owned(), name.span().start().line),
                                format!("'{name}"),
                            )
                        })
                })
                .collect(),
        )
    }

    fn short_lifetimes(file: &syn::File) -> Vec<Ident> {
        let mut visitor = ShortLifetimes::default();
        visitor.visit_file(file);
        visitor.0
    }
}

#[derive(Default)]
struct ShortLifetimes(Vec<Ident>);

impl<'ast> Visit<'ast> for ShortLifetimes {
    fn visit_lifetime_param(&mut self, parameter: &'ast LifetimeParam) {
        if parameter.lifetime.ident.to_string().len() < NamedLifetimes::MIN_NAME_LENGTH {
            self.0.push(parameter.lifetime.ident.clone());
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
            .iter()
            .map(ToString::to_string)
            .collect();
        assert_eq!(names, FLAGGED);
    }
}
