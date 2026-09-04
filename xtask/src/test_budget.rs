use std::fs;

use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::visit::Visit;
use syn::{Expr, ExprLit, ExprMacro, ExprPath, Item, ItemFn, Lit, Token};

use crate::workspace::Workspace;

#[derive(Default)]
pub struct TestBudget {
    files: usize,
    lines: usize,
    tests: usize,
    test_lines: usize,
    violations: Vec<String>,
}

impl TestBudget {
    const MAX_TESTS_PER_FILE: usize = 3;
    const MAX_ASSERTIONS_PER_TEST: usize = 3;
    const MAX_LINES_PER_TEST: usize = 20;
    const MAX_LITERALS_PER_TEST: usize = 4;
    const MAX_INTEGER_LITERAL: u64 = 64;
    const MAX_AVERAGE_TESTS_PER_FILE: usize = 1;
    const MAX_TEST_LINE_PERCENT: usize = 20;

    pub fn check(workspace: &Workspace) -> Result<(), String> {
        let mut budget = TestBudget::default();
        for file in workspace.rust_files()? {
            let source = fs::read_to_string(&file)
                .map_err(|error| format!("{}: {error}", file.display()))?;
            let parsed =
                syn::parse_file(&source).map_err(|error| format!("{}: {error}", file.display()))?;
            budget.check_file(&workspace.relative(&file), &source, &parsed.items);
        }
        budget.check_totals();
        if budget.violations.is_empty() {
            println!(
                "test budget: {} tests in {} files, {} of {} lines",
                budget.tests, budget.files, budget.test_lines, budget.lines
            );
            Ok(())
        } else {
            Err(format!(
                "test budget exceeded:\n{}",
                budget.violations.join("\n")
            ))
        }
    }

    fn check_file(&mut self, path: &str, source: &str, items: &[Item]) {
        self.files += 1;
        self.lines += source.lines().count();
        let tests = Self::tests(items);
        self.tests += tests.len();
        if tests.len() > Self::MAX_TESTS_PER_FILE {
            self.violations.push(format!(
                "{path}: {} tests, at most {} allowed",
                tests.len(),
                Self::MAX_TESTS_PER_FILE
            ));
        }
        for test in tests {
            self.check_test(path, test);
        }
        for module in Self::test_modules(items) {
            let span = module.span();
            self.test_lines += span.end().line - span.start().line + 1;
            let mut integers = IntegerLiterals::default();
            integers.visit_item_mod(module);
            for (line, value) in integers.too_large {
                self.violations.push(format!(
                    "{path}:{line}: integer literal {value} in test code, at most {} allowed",
                    Self::MAX_INTEGER_LITERAL
                ));
            }
        }
    }

    fn check_test(&mut self, path: &str, test: &ItemFn) {
        let name = &test.sig.ident;
        let line = name.span().start().line;
        let body_lines = test.block.span().end().line - test.block.span().start().line - 1;
        let mut counts = TestCounts::default();
        counts.visit_block(&test.block);
        let measurements = [
            (
                "assertions",
                counts.assertions,
                Self::MAX_ASSERTIONS_PER_TEST,
            ),
            ("body lines", body_lines, Self::MAX_LINES_PER_TEST),
            ("literals", counts.literals, Self::MAX_LITERALS_PER_TEST),
        ];
        for (kind, actual, limit) in measurements {
            if actual > limit {
                self.violations.push(format!(
                    "{path}:{line}: fn {name} has {actual} {kind}, at most {limit} allowed"
                ));
            }
        }
    }

    fn check_totals(&mut self) {
        if self.tests > self.files * Self::MAX_AVERAGE_TESTS_PER_FILE {
            self.violations.push(format!(
                "{} tests across {} files, at most {} per file on average allowed",
                self.tests,
                self.files,
                Self::MAX_AVERAGE_TESTS_PER_FILE
            ));
        }
        if self.test_lines * 100 > self.lines * Self::MAX_TEST_LINE_PERCENT {
            self.violations.push(format!(
                "{} of {} lines are test code, at most {}% allowed",
                self.test_lines,
                self.lines,
                Self::MAX_TEST_LINE_PERCENT
            ));
        }
    }

    fn tests(items: &[Item]) -> Vec<&ItemFn> {
        items
            .iter()
            .flat_map(|item| match item {
                Item::Fn(function) if Self::is_test(function) => vec![function],
                Item::Mod(module) => module
                    .content
                    .as_ref()
                    .map_or_else(Vec::new, |(_, items)| Self::tests(items)),
                _ => Vec::new(),
            })
            .collect()
    }

    fn test_modules(items: &[Item]) -> Vec<&syn::ItemMod> {
        items
            .iter()
            .filter_map(|item| match item {
                Item::Mod(module) if Self::is_cfg_test(module) => Some(module),
                _ => None,
            })
            .collect()
    }

    fn is_test(function: &ItemFn) -> bool {
        function
            .attrs
            .iter()
            .any(|attribute| attribute.path().is_ident("test"))
    }

    fn is_cfg_test(module: &syn::ItemMod) -> bool {
        module.attrs.iter().any(|attribute| {
            attribute.path().is_ident("cfg")
                && attribute
                    .parse_args::<syn::Ident>()
                    .is_ok_and(|ident| ident == "test")
        })
    }
}

#[derive(Default)]
struct TestCounts {
    assertions: usize,
    literals: usize,
}

impl TestCounts {
    const ASSERTIONS: [&str; 6] = [
        "assert",
        "assert_eq",
        "assert_ne",
        "prop_assert",
        "prop_assert_eq",
        "prop_assert_ne",
    ];

    fn is_variant_path(path: &ExprPath) -> bool {
        let mut segments = path.path.segments.iter().rev();
        match (segments.next(), segments.next()) {
            (Some(last), Some(parent)) => {
                Self::is_camel_case(&last.ident.to_string())
                    && Self::is_camel_case(&parent.ident.to_string())
            }
            _ => false,
        }
    }

    fn is_camel_case(name: &str) -> bool {
        name.starts_with(|letter: char| letter.is_ascii_uppercase())
            && name
                .chars()
                .any(|letter| letter.is_ascii_lowercase() || letter.is_ascii_digit())
    }
}

impl<'ast> Visit<'ast> for TestCounts {
    fn visit_expr_lit(&mut self, literal: &'ast ExprLit) {
        self.literals += 1;
        syn::visit::visit_expr_lit(self, literal);
    }

    fn visit_expr_path(&mut self, path: &'ast ExprPath) {
        if Self::is_variant_path(path) {
            self.literals += 1;
        }
        syn::visit::visit_expr_path(self, path);
    }

    fn visit_expr_macro(&mut self, invocation: &'ast ExprMacro) {
        self.visit_macro(&invocation.mac);
    }

    fn visit_macro(&mut self, invocation: &'ast syn::Macro) {
        let name = invocation
            .path
            .segments
            .last()
            .map(|segment| segment.ident.to_string());
        if name.is_some_and(|name| Self::ASSERTIONS.contains(&name.as_str())) {
            self.assertions += 1;
        }
        if let Ok(arguments) =
            invocation.parse_body_with(Punctuated::<Expr, Token![,]>::parse_terminated)
        {
            for argument in &arguments {
                self.visit_expr(argument);
            }
        }
    }
}

#[derive(Default)]
struct IntegerLiterals {
    too_large: Vec<(usize, String)>,
}

impl<'ast> Visit<'ast> for IntegerLiterals {
    fn visit_lit(&mut self, literal: &'ast Lit) {
        if let Lit::Int(integer) = literal
            && integer
                .base10_parse::<u64>()
                .is_ok_and(|value| value > TestBudget::MAX_INTEGER_LITERAL)
        {
            self.too_large
                .push((integer.span().start().line, integer.to_string()));
        }
    }

    fn visit_macro(&mut self, invocation: &'ast syn::Macro) {
        if let Ok(arguments) =
            invocation.parse_body_with(Punctuated::<Expr, Token![,]>::parse_terminated)
        {
            for argument in &arguments {
                self.visit_expr(argument);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TestBudget;

    const SOURCE: &str = "
#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        let squares = [Square::A1, Square::H8];
        assert_eq!(squares.len(), 2);
        assert!(Bitboard::EMPTY.is_empty());
        assert_ne!(\"a\", \"b\");
        assert_eq!(seed(), 0x2545_F491);
    }
}
";
    const REPORTED: [&str; 3] = [
        "has 4 assertions",
        "has 6 literals",
        "integer literal 0x2545_F491",
    ];

    #[test]
    fn counts_assertions_literals_and_variants_inside_macros() {
        let parsed = syn::parse_file(SOURCE).expect("valid rust");
        let mut budget = TestBudget::default();
        budget.check_file("example.rs", SOURCE, &parsed.items);
        let joined = budget.violations.join("\n");
        assert!(
            REPORTED.iter().all(|report| joined.contains(report)),
            "{joined}"
        );
    }
}
