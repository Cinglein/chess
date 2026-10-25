use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::visit::Visit;
use syn::{Attribute, Fields, ItemStruct, Path, Token};

use crate::task::report::Report;
use crate::task::site::Site;
use crate::task::source_file::SourceFile;
use crate::task::violation::Violation;

pub struct SettingsShape;

impl SettingsShape {
    pub fn report(files: &[SourceFile]) -> Report {
        Report::new(
            "a settings struct refuses keys it does not know",
            files
                .iter()
                .flat_map(|file| {
                    let mut structs = DeserializedStructs::default();
                    structs.visit_file(file.syntax());
                    structs.0.into_iter().map(move |line| {
                        Violation::new(
                            Site::Line(file.path().to_owned(), line),
                            "struct derives Deserialize with named fields but lacks #[serde(deny_unknown_fields)]; a misspelt key must be an error, not a silent default",
                        )
                    })
                })
                .collect(),
        )
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

    const KEY_SHAPES: [&str; 4] = ["deny_unknown_fields", "transparent", "try_from", "from ="];
}

#[derive(Default)]
struct DeserializedStructs(Vec<usize>);

impl<'ast> Visit<'ast> for DeserializedStructs {
    fn visit_item_struct(&mut self, item: &'ast ItemStruct) {
        if matches!(item.fields, Fields::Named(_))
            && SettingsShape::derives_deserialize(&item.attrs)
            && !item.attrs.iter().any(SettingsShape::shapes_its_keys)
        {
            self.0.push(item.ident.span().start().line);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SettingsShape;
    use crate::task::source_file::SourceFile;

    const SOURCE: &str = "
        #[derive(Deserialize)]
        pub struct Loose { rounds: usize }
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        pub struct Strict { rounds: usize }
        #[derive(Deserialize)]
        #[serde(transparent)]
        pub struct Count(usize);
        #[derive(Deserialize)]
        #[serde(try_from = \"String\")]
        pub struct Clock { base: u64 }
    ";

    const PATH: &str = "crates/a/src/x.rs";
    const REPORTED: &str = ":3:";
    const CLEAN: [&str; 2] = [":6:", ":9:"];

    #[test]
    fn flags_a_named_field_struct_that_deserializes_without_denying_unknown_keys() {
        let file = SourceFile::parse(PATH.to_owned(), SOURCE.to_owned()).expect("valid rust");
        let report = SettingsShape::report(&[file]).to_string();
        assert!(
            report.contains(REPORTED) && CLEAN.iter().all(|line| !report.contains(line)),
            "{report}"
        );
    }
}
