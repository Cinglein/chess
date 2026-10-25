use std::path::PathBuf;

use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::visit::Visit;
use syn::{Attribute, Fields, ItemStruct, Path, Token, TypePath};

use super::keys::Keys;
use task::LineNumber;

#[derive(Debug)]
pub struct DeserializedStruct {
    path: PathBuf,
    line: LineNumber,
    name: String,
    field_types: Vec<String>,
    keys: Keys,
}

impl DeserializedStruct {
    const KEY_SHAPES: &[&str] = &["deny_unknown_fields", "transparent", "try_from", "from ="];

    pub fn from_item(path: &std::path::Path, item: &ItemStruct) -> Option<DeserializedStruct> {
        (matches!(item.fields, Fields::Named(_)) && Self::derives_deserialize(&item.attrs)).then(
            || {
                let mut types = FieldTypes::default();
                types.visit_fields(&item.fields);
                DeserializedStruct {
                    path: path.to_owned(),
                    line: LineNumber::from(item.ident.span()),
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

    pub fn path(&self) -> &std::path::Path {
        &self.path
    }

    pub fn line(&self) -> LineNumber {
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
