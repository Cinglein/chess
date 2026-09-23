use std::fmt;

use quote::ToTokens;
use syn::{GenericArgument, Ident, PathArguments, Type};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TypeName(String);

impl TypeName {
    const WRAPPERS: [&str; 2] = ["Option", "Result"];

    pub fn from_type(ty: &Type) -> TypeName {
        TypeName(
            Self::unwrapped(ty)
                .to_token_stream()
                .to_string()
                .replace(' ', ""),
        )
    }

    pub fn named(ident: &Ident) -> TypeName {
        TypeName(ident.to_string())
    }

    pub fn or_self(self, self_type: &TypeName) -> TypeName {
        if self.0 == "Self" {
            self_type.clone()
        } else {
            self
        }
    }

    fn unwrapped(ty: &Type) -> &Type {
        let Type::Path(path) = ty else { return ty };
        let Some(segment) = path.path.segments.last() else {
            return ty;
        };
        if !Self::WRAPPERS.contains(&segment.ident.to_string().as_str()) {
            return ty;
        }
        let PathArguments::AngleBracketed(arguments) = &segment.arguments else {
            return ty;
        };
        match arguments.args.first() {
            Some(GenericArgument::Type(inner)) => inner,
            _ => ty,
        }
    }
}

impl fmt::Display for TypeName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}
