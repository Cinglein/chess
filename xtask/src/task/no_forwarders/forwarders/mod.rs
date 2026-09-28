mod parameter;
mod struct_fields;

use std::collections::BTreeMap;

use parameter::Parameter;
use struct_fields::StructFields;
use syn::visit::Visit;
use syn::{Expr, ExprField, ImplItemFn, ItemImpl, ItemMod, ItemStruct, Stmt, Type};

use super::forwarder::Forwarder;
use super::workspace_types::WorkspaceTypes;
use crate::task::source_file::SourceFile;

pub struct Forwarders<'types> {
    types: &'types WorkspaceTypes,
    structs: BTreeMap<String, StructFields>,
    self_type: String,
    concrete: Vec<String>,
    found: Vec<Forwarder>,
}

impl<'types> Forwarders<'types> {
    pub fn new(types: &'types WorkspaceTypes) -> Forwarders<'types> {
        Forwarders {
            types,
            structs: BTreeMap::new(),
            self_type: String::new(),
            concrete: Vec::new(),
            found: Vec::new(),
        }
    }

    pub fn found(self) -> Vec<Forwarder> {
        self.found
    }

    fn classify(&self, function: &ImplItemFn) -> Option<Forwarder> {
        let [Stmt::Expr(expr, None)] = function.block.stmts.as_slice() else {
            return None;
        };
        let params: Vec<Parameter> = function
            .sig
            .inputs
            .iter()
            .filter_map(Parameter::describe)
            .collect();
        let callee = match expr {
            Expr::MethodCall(call)
                if self
                    .receiver_type(&call.receiver, &params)
                    .is_some_and(|name| self.types.declares(&name))
                    && call.args.iter().all(|arg| Self::forwards(arg, &params)) =>
            {
                call.method.to_string()
            }
            Expr::Call(call) if call.args.iter().all(|arg| Self::forwards(arg, &params)) => {
                self.workspace_callee(&call.func)?
            }
            _ => return None,
        };
        Some(Forwarder::new(function.sig.ident.clone(), callee))
    }

    fn receiver_type(&self, expr: &Expr, params: &[Parameter]) -> Option<String> {
        match expr {
            Expr::Path(path) if path.path.is_ident("self") => Some(self.self_type.clone()),
            Expr::Path(path) => Parameter::named_by(params, &path.path)?
                .type_name()
                .map(str::to_owned),
            Expr::Field(field) => self.field_type(field),
            Expr::Reference(reference) => self.receiver_type(&reference.expr, params),
            _ => None,
        }
    }

    fn field_type(&self, field: &ExprField) -> Option<String> {
        let Expr::Path(base) = &*field.base else {
            return None;
        };
        if !base.path.is_ident("self") {
            return None;
        }
        self.structs
            .get(&self.self_type)?
            .field_type(&field.member, &self.concrete)
    }

    fn workspace_callee(&self, func: &Expr) -> Option<String> {
        let Expr::Path(path) = func else { return None };
        let segments: Vec<String> = path
            .path
            .segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect();
        let [.., owner, method] = segments.as_slice() else {
            return None;
        };
        let owner = if owner == "Self" {
            &self.self_type
        } else {
            owner
        };
        (self.types.declares(owner) && method.starts_with(char::is_lowercase))
            .then(|| format!("{owner}::{method}"))
    }
}

impl Forwarders<'_> {
    fn forwards(expr: &Expr, params: &[Parameter]) -> bool {
        match expr {
            Expr::Path(path) => Parameter::named_by(params, &path.path).is_some(),
            Expr::Field(field) => Self::forwards(&field.base, params),
            Expr::Reference(reference) => Self::forwards(&reference.expr, params),
            _ => false,
        }
    }
}

impl<'ast> Visit<'ast> for Forwarders<'_> {
    fn visit_item_mod(&mut self, module: &'ast ItemMod) {
        if !SourceFile::is_test_module(module) {
            syn::visit::visit_item_mod(self, module);
        }
    }

    fn visit_item_struct(&mut self, item: &'ast ItemStruct) {
        self.structs
            .insert(item.ident.to_string(), StructFields::describe(item));
    }

    fn visit_item_impl(&mut self, item: &'ast ItemImpl) {
        if item.trait_.is_some() {
            return;
        }
        let Type::Path(path) = &*item.self_ty else {
            return;
        };
        let Some(segment) = path.path.segments.last() else {
            return;
        };
        self.self_type = segment.ident.to_string();
        self.concrete = StructFields::type_arguments(segment);
        syn::visit::visit_item_impl(self, item);
    }

    fn visit_impl_item_fn(&mut self, function: &'ast ImplItemFn) {
        self.found.extend(self.classify(function));
        syn::visit::visit_impl_item_fn(self, function);
    }
}
