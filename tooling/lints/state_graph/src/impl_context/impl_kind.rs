use super::super::type_name::TypeName;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ImplKind {
    InherentOnVertex,
    InherentOnValue,
    TraitOnVertex(String),
    TraitOnValue(String),
}

impl ImplKind {
    pub fn trait_name(&self) -> Option<&str> {
        match self {
            ImplKind::TraitOnVertex(name) | ImplKind::TraitOnValue(name) => Some(name),
            ImplKind::InherentOnVertex | ImplKind::InherentOnValue => None,
        }
    }

    pub fn forbids_mutation(&self) -> bool {
        matches!(self, ImplKind::InherentOnVertex)
    }

    pub fn edge_source(
        &self,
        self_type: &TypeName,
        first_vertex_parameter: Option<&TypeName>,
    ) -> Option<TypeName> {
        match self {
            ImplKind::InherentOnVertex | ImplKind::TraitOnVertex(_) => Some(self_type.clone()),
            ImplKind::TraitOnValue(_) => first_vertex_parameter.cloned(),
            ImplKind::InherentOnValue => None,
        }
    }
}
