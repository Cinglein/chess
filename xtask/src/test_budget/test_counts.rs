use syn::punctuated::Punctuated;
use syn::visit::Visit;
use syn::{Block, Expr, ExprLit, ExprPath, Macro, Token};

#[derive(Default)]
pub(super) struct TestCounts {
    pub(super) assertions: usize,
    pub(super) literals: usize,
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

    pub(super) fn of(block: &Block) -> Self {
        let mut counts = Self::default();
        counts.visit_block(block);
        counts
    }

    fn is_assertion(invocation: &Macro) -> bool {
        invocation
            .path
            .segments
            .last()
            .is_some_and(|segment| Self::ASSERTIONS.contains(&segment.ident.to_string().as_str()))
    }

    fn is_variant_path(path: &ExprPath) -> bool {
        path.path
            .segments
            .iter()
            .rev()
            .take(2)
            .filter(|segment| Self::is_camel_case(&segment.ident.to_string()))
            .count()
            == 2
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
        self.literals += usize::from(Self::is_variant_path(path));
        syn::visit::visit_expr_path(self, path);
    }

    fn visit_macro(&mut self, invocation: &'ast Macro) {
        self.assertions += usize::from(Self::is_assertion(invocation));
        if let Ok(arguments) =
            invocation.parse_body_with(Punctuated::<Expr, Token![,]>::parse_terminated)
        {
            arguments
                .iter()
                .for_each(|argument| self.visit_expr(argument));
        }
    }
}
