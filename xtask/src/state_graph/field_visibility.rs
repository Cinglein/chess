use quote::ToTokens;
use syn::spanned::Spanned;
use syn::visit::Visit;
use syn::{ItemStruct, Visibility};

use crate::site::Site;
use crate::source_file::SourceFile;
use crate::violation::Violation;

pub struct FieldVisibility<'scan> {
    path: &'scan str,
    violations: Vec<Violation>,
}

impl<'scan> FieldVisibility<'scan> {
    pub fn violations(file: &'scan SourceFile) -> Vec<Violation> {
        let mut fields = FieldVisibility {
            path: file.path(),
            violations: Vec::new(),
        };
        fields.visit_file(file.syntax());
        fields.violations
    }
}

impl<'ast> Visit<'ast> for FieldVisibility<'_> {
    fn visit_item_struct(&mut self, item: &'ast ItemStruct) {
        let exposed = item
            .fields
            .iter()
            .enumerate()
            .filter(|(_, field)| !matches!(field.vis, Visibility::Inherited))
            .map(|(index, field)| {
                Violation::new(
                    Site::Line(self.path.to_owned(), field.span().start().line),
                    format!(
                        "field {} of {} is {}; struct fields are private",
                        field
                            .ident
                            .as_ref()
                            .map_or_else(|| index.to_string(), ToString::to_string),
                        item.ident,
                        field.vis.to_token_stream().to_string().replace(' ', "")
                    ),
                )
            });
        self.violations.extend(exposed);
    }
}
