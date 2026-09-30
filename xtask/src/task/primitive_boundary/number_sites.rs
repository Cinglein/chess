use syn::visit::Visit;
use syn::{
    Block, Field, Fields, Ident, ImplItemConst, ItemConst, ItemMod, ItemStruct, ItemType,
    Signature, TraitItemConst, TypePath,
};

use crate::task::site::Site;
use crate::task::source_file::SourceFile;
use crate::task::violation::Violation;

#[derive(Default)]
pub struct NumberSites {
    newtypes: Vec<Ident>,
    found: Vec<Ident>,
}

impl NumberSites {
    const BIT_BOUNDARY_FILES: [&str; 7] = [
        "crates/board/src/square.rs",
        "crates/board/src/direction.rs",
        "crates/board/src/slider/magic.rs",
        "crates/board/src/slider/magics.rs",
        "crates/board/src/slider/attack_table.rs",
        "crates/board/src/zobrist_keys/split_mix.rs",
        "xtask/src/task/magics/",
    ];
    const PRIMITIVES: [&str; 12] = [
        "u8", "u16", "u32", "u64", "u128", "i8", "i16", "i32", "i64", "i128", "f32", "f64",
    ];

    pub fn violations(file: &SourceFile) -> Vec<Violation> {
        if Self::BIT_BOUNDARY_FILES
            .iter()
            .any(|boundary| file.path().starts_with(boundary))
        {
            return Vec::new();
        }
        let mut sites = NumberSites::default();
        sites.visit_file(file.syntax());
        if !sites.newtypes.is_empty() {
            return Vec::new();
        }
        sites
            .found
            .into_iter()
            .map(|ident| {
                Violation::new(
                    Site::Line(file.path().to_owned(), ident.span().start().line),
                    format!("bare {ident}; name what it counts with a newtype"),
                )
            })
            .collect()
    }

    fn is_primitive(path: &TypePath) -> bool {
        path.qself.is_none()
            && path
                .path
                .get_ident()
                .is_some_and(|ident| Self::PRIMITIVES.contains(&ident.to_string().as_str()))
    }

    fn wraps_primitive(item: &ItemStruct) -> bool {
        match &item.fields {
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                fields.unnamed.first().is_some_and(
                    |field| matches!(&field.ty, syn::Type::Path(path) if Self::is_primitive(path)),
                )
            }
            _ => false,
        }
    }
}

impl<'ast> Visit<'ast> for NumberSites {
    fn visit_item_mod(&mut self, module: &'ast ItemMod) {
        if !SourceFile::is_test_module(module) {
            syn::visit::visit_item_mod(self, module);
        }
    }

    fn visit_item_struct(&mut self, item: &'ast ItemStruct) {
        if Self::wraps_primitive(item) {
            self.newtypes.push(item.ident.clone());
        }
        syn::visit::visit_item_struct(self, item);
    }

    fn visit_block(&mut self, _: &'ast Block) {}

    fn visit_field(&mut self, field: &'ast Field) {
        syn::visit::visit_type(self, &field.ty);
    }

    fn visit_item_const(&mut self, item: &'ast ItemConst) {
        syn::visit::visit_type(self, &item.ty);
    }

    fn visit_impl_item_const(&mut self, item: &'ast ImplItemConst) {
        syn::visit::visit_type(self, &item.ty);
    }

    fn visit_trait_item_const(&mut self, item: &'ast TraitItemConst) {
        syn::visit::visit_type(self, &item.ty);
    }

    fn visit_item_type(&mut self, item: &'ast ItemType) {
        syn::visit::visit_type(self, &item.ty);
    }

    fn visit_signature(&mut self, signature: &'ast Signature) {
        syn::visit::visit_signature(self, signature);
    }

    fn visit_type_path(&mut self, path: &'ast TypePath) {
        if Self::is_primitive(path) {
            self.found.extend(path.path.get_ident().cloned());
        }
        syn::visit::visit_type_path(self, path);
    }
}
