use syn::{ReturnType, Type, TypeTuple};

use super::type_name::TypeName;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Returns {
    Vertex(TypeName),
    TupleHoldingVertex,
    Other,
}

impl Returns {
    pub fn of(output: &ReturnType, vertex: impl Fn(&Type) -> Option<TypeName>) -> Returns {
        let ReturnType::Type(_, ty) = output else {
            return Returns::Other;
        };
        match &**ty {
            Type::Reference(_) => Returns::Other,
            Type::Tuple(tuple) => Self::of_tuple(tuple, vertex),
            other => vertex(other).map_or(Returns::Other, Returns::Vertex),
        }
    }

    pub fn vertex(&self) -> Option<&TypeName> {
        match self {
            Returns::Vertex(name) => Some(name),
            Returns::TupleHoldingVertex | Returns::Other => None,
        }
    }

    pub fn holds_vertex_in_tuple(&self) -> bool {
        matches!(self, Returns::TupleHoldingVertex)
    }

    fn of_tuple(tuple: &TypeTuple, vertex: impl Fn(&Type) -> Option<TypeName>) -> Returns {
        if tuple.elems.iter().any(|element| vertex(element).is_some()) {
            Returns::TupleHoldingVertex
        } else {
            Returns::Other
        }
    }
}
