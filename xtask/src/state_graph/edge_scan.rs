use std::collections::{BTreeMap, BTreeSet};

use syn::visit::Visit;
use syn::{ImplItem, ImplItemFn, ItemImpl, Receiver};

use super::declarations::Declarations;
use super::function_shape::FunctionShape;
use super::impl_context::ImplContext;
use super::type_name::TypeName;
use crate::source_file::SourceFile;

pub struct EdgeScan<'scan> {
    declarations: &'scan Declarations,
    path: String,
    edges: BTreeMap<(TypeName, TypeName), BTreeMap<String, String>>,
    violations: Vec<String>,
}

impl<'scan> EdgeScan<'scan> {
    pub fn new(declarations: &'scan Declarations) -> Self {
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
        let shape = FunctionShape::of(&self.path, function, context, self.declarations);
        let site = shape.site();
        if shape.tuple_holds_vertex() {
            self.violations.push(format!(
                "{site} returns a tuple holding a vertex; return the vertex"
            ));
        }
        match (function.sig.receiver(), shape.target()) {
            (None, Some(target)) if !shape.vertex_parameters().is_empty() => {
                self.violations.push(format!(
                    "{site} takes a vertex and returns {target}; make it a method on its source"
                ));
            }
            (Some(receiver), _) if receiver.reference.is_some() => {
                self.check_view(context, receiver, &shape);
            }
            (Some(_), Some(target)) => self.check_edge(context, function, &shape, target.clone()),
            _ => {}
        }
    }

    fn check_edge(
        &mut self,
        context: &ImplContext,
        function: &ImplItemFn,
        shape: &FunctionShape,
        target: TypeName,
    ) {
        let site = shape.site();
        let source = if context.is_vertex() {
            Some(context.self_type().clone())
        } else if context.in_trait() {
            shape.vertex_parameters().first().cloned()
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

    fn check_view(&mut self, context: &ImplContext, receiver: &Receiver, shape: &FunctionShape) {
        let site = shape.site();
        if receiver.mutability.is_some() {
            if context.is_vertex() && !context.in_trait() {
                self.violations.push(format!(
                    "{site} takes &mut self on vertex {}; transitions consume self",
                    context.self_type()
                ));
            }
        } else if let Some(target) = shape.target()
            && !shape.projects_field()
        {
            self.violations.push(format!(
                "{site} is a view returning vertex {target}; take self by value or return a reference"
            ));
        }
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
