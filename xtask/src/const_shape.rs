use proc_macro2::Span;
use syn::visit::Visit;
use syn::{
    Expr, ExprCast, ExprConst, ExprForLoop, ExprLoop, ExprWhile, ImplItemConst, ImplItemFn,
    ItemConst, ItemFn, ItemMod,
};

use crate::source_file::SourceFile;

pub struct ConstShape;

impl ConstShape {
    const BOUNDARY_FILES: [&str; 3] = [
        "crates/board/src/square.rs",
        "crates/board/src/bitboard.rs",
        "crates/board/src/slider/magic.rs",
    ];

    pub fn check(files: &[SourceFile]) -> Result<(), String> {
        let violations: Vec<String> = files.iter().flat_map(Self::violations).collect();
        if violations.is_empty() {
            println!(
                "casts stay at the bit boundary and const loops walk slices in {} rust files",
                files.len()
            );
            Ok(())
        } else {
            Err(format!(
                "const shape violated; casts live in const fns or boundary files, const loops are while let over slices:\n{}",
                violations.join("\n")
            ))
        }
    }

    fn violations(file: &SourceFile) -> Vec<String> {
        let mut contexts = ConstContexts {
            path: file.path(),
            const_depth: 0,
            violations: Vec::new(),
        };
        contexts.visit_file(file.syntax());
        contexts.violations
    }
}

struct ConstContexts<'scan> {
    path: &'scan str,
    const_depth: usize,
    violations: Vec<String>,
}

impl ConstContexts<'_> {
    fn report(&mut self, span: Span, message: &str) {
        self.violations
            .push(format!("{}:{}: {message}", self.path, span.start().line));
    }

    fn within_const(&mut self, visit: impl FnOnce(&mut Self)) {
        self.const_depth += 1;
        visit(self);
        self.const_depth -= 1;
    }
}

impl<'ast> Visit<'ast> for ConstContexts<'_> {
    fn visit_item_mod(&mut self, module: &'ast ItemMod) {
        if !SourceFile::is_test_module(module) {
            syn::visit::visit_item_mod(self, module);
        }
    }

    fn visit_item_fn(&mut self, function: &'ast ItemFn) {
        if function.sig.constness.is_some() {
            self.within_const(|this| syn::visit::visit_item_fn(this, function));
        } else {
            syn::visit::visit_item_fn(self, function);
        }
    }

    fn visit_impl_item_fn(&mut self, function: &'ast ImplItemFn) {
        if function.sig.constness.is_some() {
            self.within_const(|this| syn::visit::visit_impl_item_fn(this, function));
        } else {
            syn::visit::visit_impl_item_fn(self, function);
        }
    }

    fn visit_item_const(&mut self, item: &'ast ItemConst) {
        self.within_const(|this| syn::visit::visit_item_const(this, item));
    }

    fn visit_impl_item_const(&mut self, item: &'ast ImplItemConst) {
        self.within_const(|this| syn::visit::visit_impl_item_const(this, item));
    }

    fn visit_expr_const(&mut self, block: &'ast ExprConst) {
        self.within_const(|this| syn::visit::visit_expr_const(this, block));
    }

    fn visit_expr_cast(&mut self, cast: &'ast ExprCast) {
        if self.const_depth == 0
            && !ConstShape::BOUNDARY_FILES
                .iter()
                .any(|boundary| self.path.ends_with(boundary))
        {
            self.report(
                cast.as_token.span,
                "as cast outside a const fn or a bit boundary file; use From, TryFrom, or an EnumMap",
            );
        }
        syn::visit::visit_expr_cast(self, cast);
    }

    fn visit_expr_while(&mut self, expr: &'ast ExprWhile) {
        if self.const_depth > 0 && !matches!(*expr.cond, Expr::Let(_)) {
            self.report(
                expr.while_token.span,
                "counter loop in const context; walk a slice with while let [head, rest @ ..]",
            );
        }
        syn::visit::visit_expr_while(self, expr);
    }

    fn visit_expr_loop(&mut self, expr: &'ast ExprLoop) {
        if self.const_depth > 0 {
            self.report(
                expr.loop_token.span,
                "open loop in const context; walk a slice with while let or recurse",
            );
        }
        syn::visit::visit_expr_loop(self, expr);
    }

    fn visit_expr_for_loop(&mut self, expr: &'ast ExprForLoop) {
        if self.const_depth > 0 {
            self.report(expr.for_token.span, "for loop in const context");
        }
        syn::visit::visit_expr_for_loop(self, expr);
    }
}

#[cfg(test)]
mod tests {
    use super::ConstShape;
    use crate::source_file::SourceFile;

    const SOURCE: &str = "
impl Table {
    const fn build() -> [u8; 4] {
        let mut out = [0; 4];
        let mut index = 0;
        while index < 4 { out[index] = index as u8; index += 1; }
        let mut rest: &[u8] = &out;
        while let [head, tail @ ..] = rest { rest = tail; }
        out
    }
    fn lookup(&self, key: Kind) -> u8 { self.0[key as usize] }
}
";
    const REPORTED: [&str; 2] = [":6: counter loop in const context", ":11: as cast outside"];

    #[test]
    fn flags_counter_loops_in_const_and_casts_outside_const_or_boundary_files() {
        let file = SourceFile::parse("table.rs".to_owned(), SOURCE.to_owned()).expect("valid rust");
        let report = ConstShape::check(core::slice::from_ref(&file)).expect_err("violations");
        assert!(
            REPORTED.iter().all(|line| report.contains(line)),
            "{report}"
        );
    }
}
