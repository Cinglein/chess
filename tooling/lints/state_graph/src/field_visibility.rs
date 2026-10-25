use quote::ToTokens;
use std::path::Path;
use syn::spanned::Spanned;
use syn::visit::Visit;

use syn::{ItemStruct, Visibility};

use task::LineNumber;
use task::Site;
use task::SourceFile;
use task::Violation;

pub struct FieldVisibility<'scan> {
    path: &'scan Path,
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
                    Site::Line(self.path.into(), LineNumber::from(field.span())),
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
