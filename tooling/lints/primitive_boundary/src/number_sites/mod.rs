mod use_aliases;

use std::path::PathBuf;

use use_aliases::UseAliases;

use syn::visit::Visit;
use syn::{
    Block, ConstParam, Field, Fields, ImplItemConst, ItemConst, ItemEnum, ItemImpl, ItemMod,
    ItemStruct, ItemType, Signature, TraitItemConst, TypePath,
};

use task::BitBoundary;
use task::Policy;
use task::Site;
use task::SourceFile;
use task::Violation;
use task::Walk;
use task::WireType;

pub struct NumberSites<'scan> {
    path: PathBuf,
    policy: &'scan Policy,
    aliases: UseAliases,
    context: String,
    newtypes: Vec<String>,
    numbers: Vec<Violation>,
    sizes: Vec<Violation>,
}

impl<'scan> NumberSites<'scan> {
    const PRIMITIVES: [&'static str; 12] = [
        "u8", "u16", "u32", "u64", "u128", "i8", "i16", "i32", "i64", "i128", "f32", "f64",
    ];
    const SIZES: [&'static str; 2] = ["usize", "isize"];
    const SIZE_WORDS: [&'static str; 6] = ["length", "len", "capacity", "size", "bytes", "index"];
    const SIZE_DOMAIN: &'static str = "crates/";

    pub fn violations(file: &SourceFile, policy: &'scan Policy) -> Vec<Violation> {
        if BitBoundary::covers(file.path()) {
            return Vec::new();
        }
        let mut sites = NumberSites {
            path: file.path().to_owned(),
            policy,
            aliases: UseAliases::in_file(file.syntax()),
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

    fn wraps_primitive(&self, item: &ItemStruct) -> bool {
        match &item.fields {
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => fields
                .unnamed
                .first()
                .is_some_and(|field| matches!(&field.ty, syn::Type::Path(path) if Self::is_named(path, &Self::PRIMITIVES) || Self::is_named(path, &Self::SIZES) || self.policy.lists::<WireType>(&self.aliases.resolve(&path.path)))),
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

impl<'ast> Visit<'ast> for NumberSites<'_> {
    fn visit_item_mod(&mut self, module: &'ast ItemMod) {
        match SourceFile::walk(module) {
            Walk::Entered => syn::visit::visit_item_mod(self, module),
            Walk::Skipped => {}
        }
    }

    fn visit_item_impl(&mut self, item: &'ast ItemImpl) {
        match Walk::inherent_impls(item) {
            Walk::Entered => syn::visit::visit_item_impl(self, item),
            Walk::Skipped => {}
        }
    }

    fn visit_item_struct(&mut self, item: &'ast ItemStruct) {
        if self.wraps_primitive(item) {
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
        } else if self
            .policy
            .lists::<WireType>(&self.aliases.resolve(&path.path))
        {
            self.numbers.push(Violation::new(
                self.site(path),
                format!(
                    "{} in {} is a wire form; parse it into a type at the door",
                    self.aliases.resolve(&path.path),
                    self.context
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
