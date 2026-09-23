use syn::spanned::Spanned;
use syn::visit::Visit;
use syn::{Expr, ExprLet, ExprMatch, Local, Pat};

use super::declarations::Declarations;
use super::variant_paths::VariantPaths;
use crate::site::Site;
use crate::source_file::SourceFile;
use crate::violation::Violation;

pub struct VariantMatches<'scan> {
    declarations: &'scan Declarations,
    path: &'scan str,
    violations: Vec<Violation>,
}

impl<'scan> VariantMatches<'scan> {
    pub fn violations(
        file: &'scan SourceFile,
        declarations: &'scan Declarations,
    ) -> Vec<Violation> {
        let mut matches = VariantMatches {
            declarations,
            path: file.path(),
            violations: Vec::new(),
        };
        matches.visit_file(file.syntax());
        matches.violations
    }

    fn foreign_variant(&self, pat: &Pat) -> Option<String> {
        VariantPaths::in_pat(pat)
            .into_iter()
            .find(|enum_name| self.declarations.is_foreign_sum_type(enum_name, self.path))
    }

    fn report(&mut self, pat: &Pat, enum_name: &str) {
        self.violations.push(Violation::new(
            Site::Line(self.path.to_owned(), pat.span().start().line),
            format!("match on {enum_name} outside its file; dispatch through a trait"),
        ));
    }

    fn is_table(expr: &Expr) -> bool {
        match expr {
            Expr::Lit(_) | Expr::Path(_) => true,
            Expr::Unary(unary) => Self::is_table(&unary.expr),
            Expr::Paren(paren) => Self::is_table(&paren.expr),
            Expr::Tuple(tuple) => tuple.elems.iter().all(Self::is_table),
            Expr::Array(array) => array.elems.iter().all(Self::is_table),
            _ => false,
        }
    }
}

impl<'ast> Visit<'ast> for VariantMatches<'_> {
    fn visit_expr_match(&mut self, expr: &'ast ExprMatch) {
        for arm in &expr.arms {
            if let Some(enum_name) = self.foreign_variant(&arm.pat)
                && !Self::is_table(&arm.body)
            {
                self.report(&arm.pat, &enum_name);
            }
        }
        syn::visit::visit_expr_match(self, expr);
    }

    fn visit_expr_let(&mut self, expr: &'ast ExprLet) {
        if let Some(enum_name) = self.foreign_variant(&expr.pat) {
            self.report(&expr.pat, &enum_name);
        }
        syn::visit::visit_expr_let(self, expr);
    }

    fn visit_local(&mut self, local: &'ast Local) {
        if local
            .init
            .as_ref()
            .is_some_and(|init| init.diverge.is_some())
            && let Some(enum_name) = self.foreign_variant(&local.pat)
        {
            self.report(&local.pat, &enum_name);
        }
        syn::visit::visit_local(self, local);
    }
}
