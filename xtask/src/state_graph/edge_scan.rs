use std::collections::{BTreeMap, BTreeSet};

use syn::visit::Visit;
use syn::{Block, Expr, FnArg, ImplItem, ImplItemFn, ItemImpl, Receiver, ReturnType, Stmt, Type};

use super::declarations::Declarations;
use super::impl_context::ImplContext;
use super::type_name::TypeName;
use crate::source_file::SourceFile;

pub struct EdgeScan<'a> {
    declarations: &'a Declarations,
    path: String,
    edges: BTreeMap<(TypeName, TypeName), BTreeMap<String, String>>,
    violations: Vec<String>,
}

impl<'a> EdgeScan<'a> {
    pub fn new(declarations: &'a Declarations) -> Self {
        EdgeScan {
            declarations,
            path: String::new(),
            edges: BTreeMap::new(),
            violations: Vec::new(),
        }
    }

    pub fn scan(&mut self, file: &SourceFile) {
        file.path().clone_into(&mut self.path);
        self.visit_file(file.syntax());
    }

    pub fn edge_count(&self) -> usize {
        self.edges.values().map(BTreeMap::len).sum()
    }

    pub fn violations(mut self, max_edges_per_vertex: usize) -> Vec<String> {
        let mut fan_out: BTreeMap<&TypeName, BTreeSet<&String>> = BTreeMap::new();
        let mut structural = Vec::new();
        for ((source, target), names) in &self.edges {
            if names.len() > 1 {
                let sites: Vec<String> = names
                    .iter()
                    .map(|(name, site)| format!("{name} at {site}"))
                    .collect();
                structural.push(format!(
                    "{source} -> {target} has {} edges, at most one allowed: {}",
                    names.len(),
                    sites.join(", ")
                ));
            }
            fan_out.entry(source).or_default().extend(names.keys());
        }
        structural.extend(
            fan_out
                .into_iter()
                .filter(|(_, names)| names.len() > max_edges_per_vertex)
                .map(|(source, names)| {
                    format!(
                        "{source} has {} edges leaving it, at most {max_edges_per_vertex} allowed",
                        names.len()
                    )
                }),
        );
        self.violations.extend(structural);
        self.violations
    }

    fn check_function(&mut self, context: &ImplContext, function: &ImplItemFn) {
        let name = &function.sig.ident;
        let site = format!("{}:{}: fn {name}", self.path, name.span().start().line);
        let target = self.return_vertex(&function.sig.output, context, &site);
        match function.sig.receiver() {
            None => {
                if let Some(target) = target
                    && !self.vertex_parameters(function, context).is_empty()
                {
                    self.violations.push(format!(
                        "{site} takes a vertex and returns {target}; make it a method on its source"
                    ));
                }
            }
            Some(receiver) if receiver.reference.is_none() => {
                if let Some(target) = target {
                    self.check_edge(context, function, target, &site);
                }
            }
            Some(receiver) => self.check_view(context, function, receiver, target, &site),
        }
    }

    fn check_edge(
        &mut self,
        context: &ImplContext,
        function: &ImplItemFn,
        target: TypeName,
        site: &str,
    ) {
        let source = if context.is_vertex() {
            Some(context.self_type().clone())
        } else if context.in_trait() {
            self.vertex_parameters(function, context).into_iter().next()
        } else {
            None
        };
        match source {
            None => self.violations.push(format!(
                "{site} returns vertex {target} from non-vertex {}; declare the vertex or move the edge",
                context.self_type()
            )),
            Some(_) if !context.is_public(function) => self.violations.push(format!(
                "{site} is a hidden edge to {target}; edges are pub"
            )),
            Some(source) => {
                self.edges
                    .entry((source, target))
                    .or_default()
                    .entry(context.edge_name(function))
                    .or_insert_with(|| site.to_owned());
            }
        }
    }

    fn check_view(
        &mut self,
        context: &ImplContext,
        function: &ImplItemFn,
        receiver: &Receiver,
        target: Option<TypeName>,
        site: &str,
    ) {
        if receiver.mutability.is_some() {
            if context.is_vertex() && !context.in_trait() {
                self.violations.push(format!(
                    "{site} takes &mut self on vertex {}; transitions consume self",
                    context.self_type()
                ));
            }
        } else if let Some(target) = target
            && !Self::is_field_projection(&function.block)
        {
            self.violations.push(format!(
                "{site} is a view returning vertex {target}; take self by value or return a reference"
            ));
        }
    }

    fn return_vertex(
        &mut self,
        output: &ReturnType,
        context: &ImplContext,
        site: &str,
    ) -> Option<TypeName> {
        let ReturnType::Type(_, ty) = output else {
            return None;
        };
        match &**ty {
            Type::Reference(_) => None,
            Type::Tuple(tuple) => {
                if tuple
                    .elems
                    .iter()
                    .any(|element| self.declarations.is_vertex(&self.resolve(element, context)))
                {
                    self.violations.push(format!(
                        "{site} returns a tuple holding a vertex; return the vertex"
                    ));
                }
                None
            }
            other => {
                let name = self.resolve(other, context);
                self.declarations.is_vertex(&name).then_some(name)
            }
        }
    }

    fn vertex_parameters(&self, function: &ImplItemFn, context: &ImplContext) -> Vec<TypeName> {
        function
            .sig
            .inputs
            .iter()
            .filter_map(|argument| match argument {
                FnArg::Typed(typed) => Some(self.resolve(&typed.ty, context)),
                FnArg::Receiver(_) => None,
            })
            .filter(|parameter| self.declarations.is_vertex(parameter))
            .collect()
    }

    fn resolve(&self, ty: &Type, context: &ImplContext) -> TypeName {
        self.declarations
            .resolve(TypeName::of(ty).or_self(context.self_type()))
    }

    fn is_field_projection(block: &Block) -> bool {
        matches!(
            block.stmts.as_slice(),
            [Stmt::Expr(Expr::Field(field), None)]
                if matches!(&*field.base, Expr::Path(base) if base.path.is_ident("self"))
        )
    }
}

impl<'ast> Visit<'ast> for EdgeScan<'_> {
    fn visit_item_impl(&mut self, item: &'ast ItemImpl) {
        let context = ImplContext::of(item, self.declarations);
        for function in item.items.iter().filter_map(|item| match item {
            ImplItem::Fn(function) => Some(function),
            _ => None,
        }) {
            self.check_function(&context, function);
        }
        syn::visit::visit_item_impl(self, item);
    }
}
