mod declarations;
mod edge_scan;
mod field_visibility;
mod function_shape;
mod impl_context;
mod type_name;
mod variant_matches;

use declarations::Declarations;
use edge_scan::EdgeScan;
use field_visibility::FieldVisibility;
use variant_matches::VariantMatches;

use task::Report;
use task::SourceFile;
use task::{Limit, Policy};

pub struct StateGraph;

impl StateGraph {
    #[must_use]
    pub fn report(files: &[SourceFile], policy: &Policy) -> Report {
        let declarations = Declarations::collect(files);
        let edges = files
            .iter()
            .fold(EdgeScan::new(&declarations), |mut edges, file| {
                edges.scan(file);
                edges
            });
        println!(
            "state graph: {} vertices, {} edges",
            declarations.vertex_count(),
            edges.edge_count()
        );
        Report::new(
            "state graph violated",
            files
                .iter()
                .flat_map(|file| {
                    FieldVisibility::violations(file)
                        .into_iter()
                        .chain(VariantMatches::violations(file, &declarations))
                })
                .chain(edges.violations(policy.limit(Limit::EdgesPerVertex).count()))
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use settings::EmbeddedDefaults;

    use super::{Declarations, EdgeScan, StateGraph};
    use task::{Policy, SourceFile};

    const VERTICES: &str = "
pub struct Hub; impl State for Hub {} pub struct Spoke; impl State for Spoke {}
pub struct Rim; pub type Edge = Rim; impl State for Edge {}
pub enum Wire { Live(u8), Dead, Faulty { code: u8 } } pub enum Side { Left, Right }
pub struct Leaky { pub inner: u8 }
impl Hub {
    fn hidden(self) -> Spoke { Spoke } fn hidden_rim(self) -> Rim { Rim }
    pub fn first(self) -> Spoke { Spoke } pub fn again(self) -> Option<Spoke> { None } pub fn back(self) -> Hub { Hub }
    pub fn peek(&self) -> Spoke { Spoke } pub fn fresh() -> Spoke { Spoke } pub fn held(&self) -> Spoke { self.spoke }
    pub fn borrowed(&self, other: &Hub) -> Spoke { other.spoke } pub fn pair(self) -> (Spoke, u8) { (Spoke, 0) } pub fn poke(&mut self) {}
}
impl Spoke { pub fn home(self) -> Hub { Hub } }
impl Wire { fn is_live(&self) -> bool { match self { Wire::Live(_) => true, Wire::Dead => false } } }
";
    const MATCHER: &str = "
pub trait Step { fn step(hub: Hub) -> Spoke; } pub struct Walker;
impl Step for Walker { fn step(hub: Hub) -> Spoke { Spoke } }
impl Walker {
    fn relay(hub: Hub) -> Spoke { Spoke } fn bump(&mut self) {}
    fn label(wire: Wire, side: Side) -> u8 { match side { Side::Left => wire.weight(), Side::Right => 0 }; match wire { Wire::Live(n) => n.max(1), Wire::Dead => 0 } }
    fn table(wire: Wire) -> i8 { match wire { Wire::Live(_) => -(1), Wire::Dead => [(0, 1)] } }
    fn peel(wire: Wire) -> u8 { if let Wire::Live(n) = wire { n } else { 0 } }
    fn force(wire: Wire) -> u8 { let Wire::Live(n) = wire else { return 0 }; n }
    fn dead(wire: Wire) -> u8 { match wire { Wire::Dead => wire.weight(), _ => 0 } }
    fn faulty(wire: Wire) -> u8 { match wire { Wire::Faulty { code } => code.max(1), _ => 0 } }
}
";
    const FILES: [(&str, &str); 2] = [("hub.rs", VERTICES), ("other.rs", MATCHER)];
    const REPORTED: [&str; 14] = [
        "hub.rs:5: field inner of Leaky", "other.rs:6: match on Wire", "other.rs:8: match on Wire",
        "other.rs:9: match on Wire", "other.rs:10: match on Wire", "other.rs:11: match on Wire",
        "hub.rs:7: is a hidden edge Hub -> Spoke", "hub.rs:7: is a hidden edge Hub -> Rim",
        "hub.rs:9: is a view returning vertex", "hub.rs:10: is a view returning vertex",
        "hub.rs:10: returns a tuple holding", "hub.rs:10: takes &mut self on vertex",
        "other.rs:5: takes a vertex and returns", "Hub -> Spoke has 3 edges, at most one",
    ];
    const VERTEX_COUNT: usize = 3;
    const EDGE_COUNT: usize = 5;

    #[test]
    fn reports_every_broken_edge_field_and_match_and_counts_vertices_through_aliases() {
        let files = FILES.map(|(path, text)| SourceFile::parse(path, text.to_owned()).unwrap());
        let report = StateGraph::report(&files, &Policy::defaults());
        let declarations = Declarations::collect(&files);
        let mut edges = EdgeScan::new(&declarations);
        for file in &files {
            edges.scan(file);
        }
        assert!(report.lists_exactly(&REPORTED), "{report}");
        assert_eq!(
            (declarations.vertex_count(), edges.edge_count()),
            (VERTEX_COUNT, EDGE_COUNT)
        );
    }
}
