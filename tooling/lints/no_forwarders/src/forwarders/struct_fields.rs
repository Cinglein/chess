use std::collections::BTreeMap;

use syn::{
    Field, GenericArgument, GenericParam, ItemStruct, Member, PathArguments, PathSegment, Type,
};

pub struct StructFields {
    generics: Vec<String>,
    fields: BTreeMap<String, String>,
}

impl StructFields {
    pub fn describe(item: &ItemStruct) -> StructFields {
        StructFields {
            generics: item
                .generics
                .params
                .iter()
                .filter_map(|param| match param {
                    GenericParam::Type(parameter) => Some(parameter.ident.to_string()),
                    _ => None,
                })
                .collect(),
            fields: item
                .fields
                .iter()
                .enumerate()
                .filter_map(|(index, field)| {
                    let key = Self::field_key(index, field);
                    Self::type_name(&field.ty).map(|name| (key, name))
                })
                .collect(),
        }
    }

    pub fn field_type(&self, member: &Member, concrete: &[String]) -> Option<String> {
        let key = match member {
            Member::Named(ident) => ident.to_string(),
            Member::Unnamed(index) => index.index.to_string(),
        };
        let declared = self.fields.get(&key)?;
        match self.generics.iter().position(|generic| generic == declared) {
            Some(position) => concrete.get(position).cloned(),
            None => Some(declared.clone()),
        }
    }

    pub fn type_name(ty: &Type) -> Option<String> {
        match ty {
            Type::Reference(reference) => Self::type_name(&reference.elem),
            Type::Path(path) => path
                .path
                .segments
                .last()
                .map(|segment| segment.ident.to_string()),
            _ => None,
        }
    }

    pub fn type_arguments(segment: &PathSegment) -> Vec<String> {
        match &segment.arguments {
            PathArguments::AngleBracketed(arguments) => arguments
                .args
                .iter()
                .filter_map(Self::argument_type)
                .collect(),
            _ => Vec::new(),
        }
    }

    fn argument_type(argument: &GenericArgument) -> Option<String> {
        match argument {
            GenericArgument::Type(ty) => Self::type_name(ty),
            _ => None,
        }
    }

    fn field_key(index: usize, field: &Field) -> String {
        field
            .ident
            .as_ref()
            .map_or_else(|| index.to_string(), ToString::to_string)
    }
}
