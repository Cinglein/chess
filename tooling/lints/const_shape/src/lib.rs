use proc_macro2::Span;
use std::path::Path;
use syn::visit::Visit;

use syn::{
    Expr, ExprCast, ExprConst, ExprForLoop, ExprLoop, ExprWhile, ImplItemConst, ImplItemFn,
    ItemConst, ItemFn, ItemMod,
};

use task::Report;
use task::Site;
use task::SourceFile;
use task::Violation;

pub struct ConstShape;

impl ConstShape {
    const BOUNDARY_FILES: [&str; 3] = [
        "crates/board/src/square/", "crates/board/src/bitboard/",
        "crates/board/src/slider/magic.rs",
    ];

    pub fn report(files: &[SourceFile]) -> Report {
        Report::new(
            "casts live in const fns or boundary files, and const loops walk slices",
            files.iter().flat_map(Self::violations).collect(),
        )
    }

    fn violations(file: &SourceFile) -> Vec<Violation> {
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
    path: &'scan Path,
    const_depth: usize,
    violations: Vec<Violation>,
}

impl ConstContexts<'_> {
    fn report(&mut self, span: Span, message: &str) {
        self.violations.push(Violation::new(
            Site::Line(self.path.into(), span.start().line),
            message,
        ));
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
                .any(|boundary| self.path.starts_with(boundary))
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
    use task::SourceFile;

    const SOURCE: &str = "
const fn free(mut rest: &[u8]) -> u8 { let mut i = 0; while i < 2 { i += 1; } while let [_, tail @ ..] = rest { rest = tail; } loop { break i } }
const CELLS: u8 = { let mut i = 0; while i < 2 { i += 1; } i };
impl Table { const SIZE: u8 = { for n in 0..2 {} 0 }; const fn build(key: u8) -> u8 { key as u8 } }
impl Table { fn lookup(&self, key: Kind) -> u8 { let z = const { loop {} }; let mut i = 0; while i < 2 { i += 1; } for n in 0..2 {} loop { break self.0[key as usize] } } }
mod inner { fn widen(x: u8) -> u16 { x as u16 } }
#[cfg(test)] mod tests { fn widen(x: u8) -> u16 { x as u16 } }
";
    const REPORTED: [&str; 7] = [
        ":2: counter loop", ":2: open loop", ":3: counter loop", ":4: for loop", ":5: open loop",
        ":5: as cast", ":6: as cast",
    ];

    #[test]
    fn flags_counter_and_open_loops_in_const_context_and_casts_outside_const_or_boundary_files() {
        let file = SourceFile::parse("table.rs", SOURCE.to_owned()).unwrap();
        let report = ConstShape::report(core::slice::from_ref(&file));
        assert!(report.lists_exactly(&REPORTED), "{report}");
    }
}
