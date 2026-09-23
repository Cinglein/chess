use std::collections::{BTreeMap, BTreeSet};

use syn::visit::Visit;
use syn::{ImplItem, ImplItemFn, ItemImpl, Receiver};

use super::declarations::Declarations;
use super::edge::Edge;
use super::function_shape::FunctionShape;
use super::impl_context::ImplContext;
use super::type_name::TypeName;
use crate::site::Site;
use crate::source_file::SourceFile;
use crate::violation::Violation;

pub struct EdgeScan<'scan> {
    declarations: &'scan Declarations,
    path: String,
    edges: BTreeMap<Edge, BTreeMap<String, Site>>,
    violations: Vec<Violation>,
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

    pub fn violations(mut self, max_edges_per_vertex: usize) -> Vec<Violation> {
        let duplicates =
            self.edges
                .iter()
                .filter(|(_, names)| names.len() > 1)
                .map(|(edge, names)| {
                    let sites: Vec<String> = names
                        .iter()
                        .map(|(name, site)| format!("{name} at {site}"))
                        .collect();
                    Violation::new(
                        Site::Workspace,
                        format!(
                            "{edge} has {} edges, at most one allowed: {}",
                            names.len(),
                            sites.join(", ")
                        ),
                    )
                });
        let fan_out = self.edges.iter().fold(
            BTreeMap::<&TypeName, BTreeSet<&String>>::new(),
            |mut fan_out, (edge, names)| {
                fan_out
                    .entry(edge.source())
                    .or_default()
                    .extend(names.keys());
                fan_out
            },
        );
        let crowded = fan_out
            .into_iter()
            .filter(|(_, names)| names.len() > max_edges_per_vertex)
            .map(|(source, names)| {
                Violation::new(
                    Site::Workspace,
                    format!(
                        "{source} has {} edges leaving it, at most {max_edges_per_vertex} allowed",
                        names.len()
                    ),
                )
            });
        let structural: Vec<Violation> = duplicates.chain(crowded).collect();
        self.violations.extend(structural);
        self.violations
    }

    fn check_function(&mut self, context: &ImplContext, function: &ImplItemFn) {
        let shape = FunctionShape::of(&self.path, function, context, self.declarations);
        if shape.returns().holds_vertex_in_tuple() {
            self.violations.push(Violation::new(
                shape.site().clone(),
                "returns a tuple holding a vertex; return the vertex",
            ));
        }
        match (function.sig.receiver(), shape.returns().vertex()) {
            (None, Some(target)) if context.in_trait() => {
                let edge = shape
                    .vertex_parameters()
                    .first()
                    .map(|source| Edge::new(source.clone(), target.clone()));
                self.check_edge(context, function, &shape, edge);
            }
            (None, Some(target)) if !shape.vertex_parameters().is_empty() => {
                self.violations.push(Violation::new(
                    shape.site().clone(),
                    format!("takes a vertex and returns {target}; make it a method on its source"),
                ));
            }
            (Some(receiver), _) if receiver.reference.is_some() => {
                self.check_view(context, receiver, &shape);
            }
            (Some(_), Some(target)) => {
                let edge = context
                    .edge_source(&shape)
                    .map(|source| Edge::new(source, target.clone()));
                match edge {
                    None => self.violations.push(Violation::new(
                        shape.site().clone(),
                        format!(
                            "returns vertex {target} from non-vertex {}; declare the vertex or move the edge",
                            context.self_type()
                        ),
                    )),
                    Some(edge) => self.record_edge(context, function, &shape, edge),
                }
            }
            _ => {}
        }
    }

    fn check_edge(
        &mut self,
        context: &ImplContext,
        function: &ImplItemFn,
        shape: &FunctionShape,
        edge: Option<Edge>,
    ) {
        if let Some(edge) = edge {
            self.record_edge(context, function, shape, edge);
        }
    }

    fn record_edge(
        &mut self,
        context: &ImplContext,
        function: &ImplItemFn,
        shape: &FunctionShape,
        edge: Edge,
    ) {
        if context.is_public(function) {
            self.edges
                .entry(edge)
                .or_default()
                .entry(context.edge_name(function))
                .or_insert_with(|| shape.site().clone());
        } else {
            self.violations.push(Violation::new(
                shape.site().clone(),
                format!("is a hidden edge {edge}; edges are pub"),
            ));
        }
    }

    fn check_view(&mut self, context: &ImplContext, receiver: &Receiver, shape: &FunctionShape) {
        if receiver.mutability.is_some() {
            if context.forbids_mutation() {
                self.violations.push(Violation::new(
                    shape.site().clone(),
                    format!(
                        "takes &mut self on vertex {}; transitions consume self",
                        context.self_type()
                    ),
                ));
            }
        } else if let Some(target) = shape.returns().vertex()
            && !shape.body().projects_field()
        {
            self.violations.push(Violation::new(
                shape.site().clone(),
                format!(
                    "is a view returning vertex {target}; take self by value or return a reference"
                ),
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
