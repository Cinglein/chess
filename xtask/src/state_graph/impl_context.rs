use syn::{ImplItemFn, ItemImpl, Visibility};

use super::declarations::Declarations;
use super::type_name::TypeName;

pub struct ImplContext {
    self_type: TypeName,
    trait_name: Option<String>,
    is_vertex: bool,
}

impl ImplContext {
    pub fn of(item: &ItemImpl, declarations: &Declarations) -> ImplContext {
        let self_type = declarations.resolve(TypeName::of(&item.self_ty));
        ImplContext {
            is_vertex: declarations.is_vertex(&self_type),
            trait_name: item
                .trait_
                .as_ref()
                .and_then(|(_, path, _)| path.segments.last())
                .map(|segment| segment.ident.to_string()),
            self_type,
        }
    }

    pub fn self_type(&self) -> &TypeName {
        &self.self_type
    }

    pub fn is_vertex(&self) -> bool {
        self.is_vertex
    }

    pub fn in_trait(&self) -> bool {
        self.trait_name.is_some()
    }

    pub fn is_public(&self, function: &ImplItemFn) -> bool {
        self.in_trait() || matches!(function.vis, Visibility::Public(_))
    }

    pub fn edge_name(&self, function: &ImplItemFn) -> String {
        self.trait_name
            .clone()
            .unwrap_or_else(|| function.sig.ident.to_string())
    }
}
