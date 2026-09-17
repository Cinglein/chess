use quote::ToTokens;
use syn::spanned::Spanned;
use syn::visit::Visit;
use syn::{ItemStruct, Visibility};

use crate::source_file::SourceFile;

pub struct FieldVisibility<'a> {
    path: &'a str,
    violations: Vec<String>,
}

impl<'a> FieldVisibility<'a> {
    pub fn violations(file: &'a SourceFile) -> Vec<String> {
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
        for (index, field) in item.fields.iter().enumerate() {
            if matches!(field.vis, Visibility::Inherited) {
                continue;
            }
            let name = field
                .ident
                .as_ref()
                .map_or_else(|| index.to_string(), ToString::to_string);
            self.violations.push(format!(
                "{}:{}: field {name} of {} is {}; struct fields are private",
                self.path,
                field.span().start().line,
                item.ident,
                field.vis.to_token_stream().to_string().replace(' ', "")
            ));
        }
    }
}
