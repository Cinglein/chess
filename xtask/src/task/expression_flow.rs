use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::visit::Visit;
use syn::{
    Block, Expr, ExprBreak, ExprContinue, ExprForLoop, ExprIf, ExprLoop, ExprReturn, ExprWhile,
    ImplItemConst, ImplItemFn, ItemConst, ItemFn, ItemMod, Stmt, TraitItemFn,
};

use crate::task::report::Report;
use crate::task::site::Site;
use crate::task::source_file::SourceFile;
use crate::task::violation::Violation;

pub struct ExpressionFlow;

impl ExpressionFlow {
    const COVERED: &str = "crates/";

    pub fn report(files: &[SourceFile]) -> Report {
        Report::new(
            "control flow is an expression; every branch yields the value the function returns",
            files
                .iter()
                .filter(|file| file.path().starts_with(Self::COVERED))
                .flat_map(Self::violations)
                .collect(),
        )
    }

    fn violations(file: &SourceFile) -> Vec<Violation> {
        let mut flows = Flows {
            path: file.path(),
            violations: Vec::new(),
        };
        flows.visit_file(file.syntax());
        flows.violations
    }

    fn is_dropped(statement: &Stmt, last: bool) -> bool {
        match statement {
            Stmt::Expr(Expr::If(_) | Expr::Match(_), semicolon) => semicolon.is_some() || !last,
            _ => false,
        }
    }
}

struct Flows<'scan> {
    path: &'scan str,
    violations: Vec<Violation>,
}

impl Flows<'_> {
    fn report(&mut self, span: Span, message: &str) {
        self.violations.push(Violation::new(
            Site::Line(self.path.to_owned(), span.start().line),
            message,
        ));
    }
}

impl<'ast> Visit<'ast> for Flows<'_> {
    fn visit_item_mod(&mut self, module: &'ast ItemMod) {
        if !SourceFile::is_test_module(module) {
            syn::visit::visit_item_mod(self, module);
        }
    }

    fn visit_item_const(&mut self, _: &'ast ItemConst) {}

    fn visit_impl_item_const(&mut self, _: &'ast ImplItemConst) {}

    fn visit_item_fn(&mut self, function: &'ast ItemFn) {
        if function.sig.constness.is_none() {
            self.visit_block(&function.block);
        }
    }

    fn visit_impl_item_fn(&mut self, function: &'ast ImplItemFn) {
        if function.sig.constness.is_none() {
            self.visit_block(&function.block);
        }
    }

    fn visit_trait_item_fn(&mut self, function: &'ast TraitItemFn) {
        if let Some(body) = function
            .default
            .as_ref()
            .filter(|_| function.sig.constness.is_none())
        {
            self.visit_block(body);
        }
    }

    fn visit_block(&mut self, block: &'ast Block) {
        let last = block.stmts.len().saturating_sub(1);
        let dropped: Vec<Span> = block
            .stmts
            .iter()
            .enumerate()
            .filter(|(index, statement)| ExpressionFlow::is_dropped(statement, *index == last))
            .map(|(_, statement)| statement.span())
            .collect();
        for span in dropped {
            self.report(
                span,
                "if or match whose value is dropped; let it produce the value the function returns",
            );
        }
        syn::visit::visit_block(self, block);
    }

    fn visit_expr_if(&mut self, expr: &'ast ExprIf) {
        if expr.else_branch.is_none() {
            self.report(
                expr.if_token.span,
                "if without else yields nothing; use then_some, filter, or an else branch that produces the value",
            );
        }
        syn::visit::visit_expr_if(self, expr);
    }

    fn visit_expr_return(&mut self, expr: &'ast ExprReturn) {
        self.report(
            expr.return_token.span,
            "early return; make the value the tail expression or exit through ? on a Result",
        );
        syn::visit::visit_expr_return(self, expr);
    }

    fn visit_expr_break(&mut self, expr: &'ast ExprBreak) {
        self.report(
            expr.break_token.span,
            "break; fold with ControlFlow or find the element with an iterator",
        );
        syn::visit::visit_expr_break(self, expr);
    }

    fn visit_expr_continue(&mut self, expr: &'ast ExprContinue) {
        self.report(
            expr.continue_token.span,
            "continue; filter the iterator instead",
        );
    }

    fn visit_expr_loop(&mut self, expr: &'ast ExprLoop) {
        self.report(
            expr.loop_token.span,
            "loop; drive an iterator with combinators, const fns excepted",
        );
        syn::visit::visit_expr_loop(self, expr);
    }

    fn visit_expr_for_loop(&mut self, expr: &'ast ExprForLoop) {
        self.report(
            expr.for_token.span,
            "for loop; drive the iterator with combinators, const fns excepted",
        );
        syn::visit::visit_expr_for_loop(self, expr);
    }

    fn visit_expr_while(&mut self, expr: &'ast ExprWhile) {
        self.report(
            expr.while_token.span,
            "while loop; drive an iterator with combinators, const fns excepted",
        );
        syn::visit::visit_expr_while(self, expr);
    }
}

#[cfg(test)]
mod tests {
    use super::ExpressionFlow;
    use crate::task::source_file::SourceFile;

    const SOURCE: &str = "
impl Flow {
    fn guards(&self, items: &[u8]) -> u8 {
        if items.is_empty() { return 0; }
        for item in items { self.note(item); }
        match items.len() { 0 => self.note(0), _ => self.note(1) };
        if items.len() > 1 { self.note(2) } else { self.note(3) }
    }
    const fn walk(mut rest: &[u8]) -> u8 { while let [head, tail @ ..] = rest { rest = tail; } 0 }
}
";
    const REPORTED: [&str; 4] = [
        ":4: if without else",
        ":4: early return",
        ":5: for loop",
        ":6: if or match whose value is dropped",
    ];
    const CLEAN: [&str; 2] = [":7:", ":9:"];

    #[test]
    fn reports_returns_loops_and_dropped_branches_outside_const_fns() {
        let file =
            SourceFile::parse("crates/flow.rs".to_owned(), SOURCE.to_owned()).expect("valid rust");
        let report = ExpressionFlow::report(core::slice::from_ref(&file)).to_string();
        assert!(
            REPORTED.iter().all(|line| report.contains(line))
                && CLEAN.iter().all(|line| !report.contains(line)),
            "{report}"
        );
    }
}
