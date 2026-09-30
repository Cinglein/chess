use syn::visit::Visit;
use syn::{
    Block, ConstParam, Field, Fields, ImplItemConst, ItemConst, ItemEnum, ItemImpl, ItemMod,
    ItemStruct, ItemType, Signature, TraitItemConst, TypePath,
};

use crate::task::site::Site;
use crate::task::source_file::SourceFile;
use crate::task::violation::Violation;

pub struct NumberSites {
    path: String,
    context: String,
    newtypes: Vec<String>,
    numbers: Vec<Violation>,
    sizes: Vec<Violation>,
}

impl NumberSites {
    const PRIMITIVES: [&str; 12] = [
        "u8", "u16", "u32", "u64", "u128", "i8", "i16", "i32", "i64", "i128", "f32", "f64",
    ];
    const SIZES: [&str; 2] = ["usize", "isize"];
    const SIZE_WORDS: [&str; 6] = ["length", "len", "capacity", "size", "bytes", "index"];
    const BIT_BOUNDARY_FILES: [&str; 7] = [
        "crates/board/src/square.rs",
        "crates/board/src/direction.rs",
        "crates/board/src/slider/magic.rs",
        "crates/board/src/slider/magics.rs",
        "crates/board/src/slider/attack_table.rs",
        "crates/board/src/zobrist_keys/split_mix.rs",
        "xtask/src/task/magics/",
    ];
    const SIZE_DOMAIN: &str = "crates/";

    pub fn violations(file: &SourceFile) -> Vec<Violation> {
        if Self::BIT_BOUNDARY_FILES
            .iter()
            .any(|boundary| file.path().starts_with(boundary))
        {
            return Vec::new();
        }
        let mut sites = NumberSites {
            path: file.path().to_owned(),
            context: String::new(),
            newtypes: Vec::new(),
            numbers: Vec::new(),
            sizes: Vec::new(),
        };
        sites.visit_file(file.syntax());
        if !sites.newtypes.is_empty() {
            return Vec::new();
        }
        if file.path().starts_with(Self::SIZE_DOMAIN) {
            sites.numbers.extend(sites.sizes);
        }
        sites.numbers
    }

    fn is_named(path: &TypePath, names: &[&str]) -> bool {
        path.qself.is_none()
            && path
                .path
                .get_ident()
                .is_some_and(|ident| names.contains(&ident.to_string().as_str()))
    }

    fn wraps_primitive(item: &ItemStruct) -> bool {
        match &item.fields {
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => fields
                .unnamed
                .first()
                .is_some_and(|field| matches!(&field.ty, syn::Type::Path(path) if Self::is_named(path, &Self::PRIMITIVES) || Self::is_named(path, &Self::SIZES))),
            _ => false,
        }
    }

    fn names_a_size(&self) -> bool {
        let lowered = self.context.to_lowercase();
        Self::SIZE_WORDS.iter().any(|word| lowered.contains(word))
    }

    fn site(&self, path: &TypePath) -> Site {
        Site::Line(
            self.path.clone(),
            path.path.segments[0].ident.span().start().line,
        )
    }
}

impl<'ast> Visit<'ast> for NumberSites {
    fn visit_item_mod(&mut self, module: &'ast ItemMod) {
        if !SourceFile::is_test_module(module) {
            syn::visit::visit_item_mod(self, module);
        }
    }

    fn visit_item_impl(&mut self, item: &'ast ItemImpl) {
        if item.trait_.is_none() {
            syn::visit::visit_item_impl(self, item);
        }
    }

    fn visit_item_struct(&mut self, item: &'ast ItemStruct) {
        if Self::wraps_primitive(item) {
            self.newtypes.push(item.ident.to_string());
        }
        self.context = item.ident.to_string();
        syn::visit::visit_item_struct(self, item);
    }

    fn visit_item_enum(&mut self, item: &'ast ItemEnum) {
        self.context = item.ident.to_string();
        syn::visit::visit_item_enum(self, item);
    }

    fn visit_const_param(&mut self, _: &'ast ConstParam) {}

    fn visit_block(&mut self, _: &'ast Block) {}

    fn visit_field(&mut self, field: &'ast Field) {
        if let Some(ident) = &field.ident {
            self.context = ident.to_string();
        }
        syn::visit::visit_type(self, &field.ty);
    }

    fn visit_item_const(&mut self, item: &'ast ItemConst) {
        self.context = item.ident.to_string();
        syn::visit::visit_type(self, &item.ty);
    }

    fn visit_impl_item_const(&mut self, item: &'ast ImplItemConst) {
        self.context = item.ident.to_string();
        syn::visit::visit_type(self, &item.ty);
    }

    fn visit_trait_item_const(&mut self, item: &'ast TraitItemConst) {
        self.context = item.ident.to_string();
        syn::visit::visit_type(self, &item.ty);
    }

    fn visit_item_type(&mut self, item: &'ast ItemType) {
        self.context = item.ident.to_string();
        syn::visit::visit_type(self, &item.ty);
    }

    fn visit_signature(&mut self, signature: &'ast Signature) {
        self.context = signature.ident.to_string();
        syn::visit::visit_signature(self, signature);
    }

    fn visit_type_path(&mut self, path: &'ast TypePath) {
        if Self::is_named(path, &Self::PRIMITIVES) {
            self.numbers.push(Violation::new(
                self.site(path),
                format!(
                    "bare {} in {}; name what it counts with a newtype",
                    path.path.segments[0].ident, self.context
                ),
            ));
        } else if Self::is_named(path, &Self::SIZES) && !self.names_a_size() {
            self.sizes.push(Violation::new(
                self.site(path),
                format!("usize in {} is neither a length nor an index; a count gets a newtype, a size says so in its name", self.context),
            ));
        }
        syn::visit::visit_type_path(self, path);
    }
}
