use syn::visit::Visit;

use syn::{Ident, LifetimeParam};

use task::Report;
use task::Site;
use task::SourceFile;
use task::Violation;
use task::{Floor, Policy};

pub struct NamedLifetimes;

impl NamedLifetimes {
    #[must_use]
    pub fn report(files: &[SourceFile], policy: &Policy) -> Report {
        Report::new(
            "lifetimes are named with a word, never a letter",
            files
                .iter()
                .flat_map(|file| {
                    Self::short_lifetimes(file.syntax(), policy)
                        .into_iter()
                        .map(move |name| {
                            Violation::new(
                                Site::Line(file.path().into(), name.span().start().line),
                                format!("'{name}"),
                            )
                        })
                })
                .collect(),
        )
    }

    fn short_lifetimes(file: &syn::File, policy: &Policy) -> Vec<Ident> {
        let mut visitor = ShortLifetimes {
            policy,
            found: Vec::new(),
        };
        visitor.visit_file(file);
        visitor.found
    }
}

struct ShortLifetimes<'scan> {
    policy: &'scan Policy,
    found: Vec<Ident>,
}

impl<'ast> Visit<'ast> for ShortLifetimes<'_> {
    fn visit_lifetime_param(&mut self, parameter: &'ast LifetimeParam) {
        if !self
            .policy
            .floor(Floor::LifetimeName)
            .allows(parameter.lifetime.ident.to_string().len())
        {
            self.found.push(parameter.lifetime.ident.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use settings::EmbeddedDefaults;

    use super::NamedLifetimes;
    use task::Policy;

    const SOURCE: &str = "
struct Scan<'a, 'scan> { text: &'a str, path: &'scan Path, owned: &'static str }
impl<'b> Scan<'b, '_> { fn view<'c>(&'c self) -> &'c str { self.text } }
";
    const FLAGGED: [&str; 3] = ["a", "b", "c"];

    #[test]
    fn flags_single_letter_lifetimes_where_they_are_declared() {
        let parsed = syn::parse_file(SOURCE).expect("valid rust");
        let names: Vec<String> = NamedLifetimes::short_lifetimes(&parsed, &Policy::defaults())
            .iter()
            .map(ToString::to_string)
            .collect();
        assert_eq!(names, FLAGGED);
    }
}
