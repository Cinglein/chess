use syn::{ItemImpl, PathSegment, Type};

pub enum ImplTarget<'ast> {
    Inherent(&'ast PathSegment),
    Skipped,
}

impl<'ast> ImplTarget<'ast> {
    pub fn describe(item: &'ast ItemImpl) -> ImplTarget<'ast> {
        match (&item.trait_, &*item.self_ty) {
            (None, Type::Path(path)) => path
                .path
                .segments
                .last()
                .map_or(ImplTarget::Skipped, ImplTarget::Inherent),
            _ => ImplTarget::Skipped,
        }
    }
}
