use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::visit::Visit;
use syn::{Attribute, Fields, ItemStruct, Path, Token, TypePath};

use super::keys::Keys;

#[derive(Debug)]
pub struct DeserializedStruct {
    path: String,
    line: usize,
    name: String,
    field_types: Vec<String>,
    keys: Keys,
}

impl DeserializedStruct {
    const KEY_SHAPES: &[&str] = &["deny_unknown_fields", "transparent", "try_from", "from ="];

    pub fn from_item(path: &str, item: &ItemStruct) -> Option<DeserializedStruct> {
        (matches!(item.fields, Fields::Named(_)) && Self::derives_deserialize(&item.attrs)).then(
            || {
                let mut types = FieldTypes::default();
                types.visit_fields(&item.fields);
                DeserializedStruct {
                    path: path.to_owned(),
                    line: item.ident.span().start().line,
                    name: item.ident.to_string(),
                    field_types: types.0,
                    keys: if item.attrs.iter().any(Self::shapes_its_keys) {
                        Keys::Denied
                    } else {
                        Keys::Open
                    },
                }
            },
        )
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn field_types(&self) -> &[String] {
        &self.field_types
    }

    pub fn keys(&self) -> Keys {
        self.keys
    }

    fn derives_deserialize(attributes: &[Attribute]) -> bool {
        attributes.iter().any(|attribute| {
            attribute.path().is_ident("derive")
                && attribute
                    .parse_args_with(Punctuated::<Path, Token![,]>::parse_terminated)
                    .is_ok_and(|paths| paths.iter().any(Self::names_deserialize))
        })
    }

    fn names_deserialize(path: &Path) -> bool {
        path.segments
            .last()
            .is_some_and(|segment| segment.ident == "Deserialize")
    }

    fn shapes_its_keys(attribute: &Attribute) -> bool {
        attribute.path().is_ident("serde")
            && Self::KEY_SHAPES
                .iter()
                .any(|shape| attribute.meta.to_token_stream().to_string().contains(shape))
    }
}

#[derive(Default)]
struct FieldTypes(Vec<String>);

impl<'ast> Visit<'ast> for FieldTypes {
    fn visit_type_path(&mut self, path: &'ast TypePath) {
        self.0.extend(
            path.path
                .segments
                .last()
                .map(|segment| segment.ident.to_string()),
        );
        syn::visit::visit_type_path(self, path);
    }
}
