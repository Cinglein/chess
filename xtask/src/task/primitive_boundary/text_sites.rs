use syn::visit::Visit;
use syn::{Expr, ExprCall, ExprMacro, ExprMethodCall, Ident, ItemImpl, ItemMod};

use crate::task::site::Site;
use crate::task::source_file::SourceFile;
use crate::task::violation::Violation;

#[derive(Default)]
pub struct TextSites {
    doors: Vec<Ident>,
    found: Vec<Ident>,
}

impl TextSites {
    const TEXT_DOMAIN: &str = "crates/";
    const DOOR_TRAITS: [&str; 3] = ["FromStr", "TryFrom", "Display"];
    const INSPECTING: [&str; 13] = [
        "split_whitespace",
        "split_once",
        "strip_prefix",
        "strip_suffix",
        "starts_with",
        "ends_with",
        "trim",
        "trim_start",
        "trim_end",
        "chars",
        "bytes",
        "char_indices",
        "split_at_checked",
    ];
    const PRODUCING: [&str; 2] = ["to_string", "push_str"];

    pub fn violations(file: &SourceFile) -> Vec<Violation> {
        if !file.path().starts_with(Self::TEXT_DOMAIN) {
            return Vec::new();
        }
        let mut sites = TextSites::default();
        sites.visit_file(file.syntax());
        if !sites.doors.is_empty() {
            return Vec::new();
        }
        sites
            .found
            .into_iter()
            .map(|ident| {
                Violation::new(
                    Site::Line(file.path().to_owned(), ident.span().start().line),
                    format!(
                        "{ident} handles text outside a door; parse in FromStr or render in Display"
                    ),
                )
            })
            .collect()
    }

    fn door_trait(item: &ItemImpl) -> Option<Ident> {
        item.trait_
            .as_ref()
            .and_then(|(_, path, _)| path.segments.last())
            .map(|segment| segment.ident.clone())
            .filter(|ident| Self::DOOR_TRAITS.contains(&ident.to_string().as_str()))
    }
}

impl<'ast> Visit<'ast> for TextSites {
    fn visit_item_mod(&mut self, module: &'ast ItemMod) {
        if !SourceFile::is_test_module(module) {
            syn::visit::visit_item_mod(self, module);
        }
    }

    fn visit_item_impl(&mut self, item: &'ast ItemImpl) {
        self.doors.extend(Self::door_trait(item));
        syn::visit::visit_item_impl(self, item);
    }

    fn visit_expr_method_call(&mut self, call: &'ast ExprMethodCall) {
        let name = call.method.to_string();
        if Self::INSPECTING.contains(&name.as_str()) || Self::PRODUCING.contains(&name.as_str()) {
            self.found.push(call.method.clone());
        }
        syn::visit::visit_expr_method_call(self, call);
    }

    fn visit_expr_macro(&mut self, expr: &'ast ExprMacro) {
        if expr.mac.path.is_ident("format") {
            self.found.extend(expr.mac.path.get_ident().cloned());
        }
        syn::visit::visit_expr_macro(self, expr);
    }

    fn visit_expr_call(&mut self, call: &'ast ExprCall) {
        if let Expr::Path(path) = &*call.func
            && path.path.segments.len() == 2
            && path.path.segments[0].ident == "String"
            && path.path.segments[1].ident == "from"
        {
            self.found.push(path.path.segments[1].ident.clone());
        }
        syn::visit::visit_expr_call(self, call);
    }
}
