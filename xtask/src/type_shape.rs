use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::visit::Visit;
use syn::{
    FnArg, GenericArgument, ItemImpl, ItemMod, ItemStruct, PathArguments, Signature, Type,
    TypePath, TypeTuple,
};

use crate::report::Report;
use crate::site::Site;
use crate::source_file::SourceFile;
use crate::violation::Violation;

pub struct TypeShape;

impl TypeShape {
    pub fn report(files: &[SourceFile]) -> Report {
        Report::new(
            "types carry their meaning: no bool fields, no tuples in signatures, no String errors",
            files.iter().flat_map(Self::violations).collect(),
        )
    }

    fn violations(file: &SourceFile) -> Vec<Violation> {
        let mut shapes = TypeShapes {
            path: file.path(),
            violations: Vec::new(),
        };
        shapes.visit_file(file.syntax());
        shapes.violations
    }

    fn findings(ty: &Type) -> Vec<&'static str> {
        let mut findings = TypeFindings::default();
        findings.visit_type(ty);
        findings.0
    }
}

struct TypeShapes<'scan> {
    path: &'scan str,
    violations: Vec<Violation>,
}

impl TypeShapes<'_> {
    fn report(&mut self, span: Span, message: &str) {
        self.violations.push(Violation::new(
            Site::Line(self.path.to_owned(), span.start().line),
            message,
        ));
    }

    fn check_type(&mut self, ty: &Type) {
        for finding in TypeShape::findings(ty) {
            self.report(ty.span(), finding);
        }
    }
}

impl<'ast> Visit<'ast> for TypeShapes<'_> {
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
        for field in &item.fields {
            if matches!(&field.ty, Type::Path(path) if path.path.is_ident("bool")) {
                self.report(
                    field.span(),
                    "bool field; a stored flag is a stored mode, make it an enum or a type parameter",
                );
            }
            self.check_type(&field.ty);
        }
    }

    fn visit_signature(&mut self, signature: &'ast Signature) {
        for argument in &signature.inputs {
            if let FnArg::Typed(typed) = argument {
                self.check_type(&typed.ty);
            }
        }
        if let syn::ReturnType::Type(_, ty) = &signature.output {
            self.check_type(ty);
        }
    }
}

#[derive(Default)]
struct TypeFindings(Vec<&'static str>);

impl<'ast> Visit<'ast> for TypeFindings {
    fn visit_type_tuple(&mut self, tuple: &'ast TypeTuple) {
        if !tuple.elems.is_empty() {
            self.0
                .push("tuple in a signature or field; values that travel together are a struct");
        }
        syn::visit::visit_type_tuple(self, tuple);
    }

    fn visit_type_path(&mut self, path: &'ast TypePath) {
        if let Some(segment) = path.path.segments.last()
            && segment.ident == "Result"
            && let PathArguments::AngleBracketed(arguments) = &segment.arguments
            && let Some(GenericArgument::Type(Type::Path(error))) = arguments.args.iter().nth(1)
            && error.path.is_ident("String")
        {
            self.0
                .push("Result with a String error; errors are an enum with thiserror");
        }
        syn::visit::visit_type_path(self, path);
    }
}

#[cfg(test)]
mod tests {
    use super::TypeShape;
    use crate::source_file::SourceFile;

    const SOURCE: &str = "
struct Flags { ready: bool, pair: (u8, u8) }
impl Flags {
    fn split(&self) -> (u8, u8) { self.pair }
    fn load(&self) -> Result<u8, String> { Ok(0) }
    fn unit(&self) -> () {}
}
impl Iterator for Flags { type Item = u8; fn size_hint(&self) -> (usize, Option<usize>) { (0, None) } }
";
    const REPORTED: [&str; 4] = [
        ":2: bool field",
        ":2: tuple in a signature",
        ":4: tuple in a signature",
        ":5: Result with a String error",
    ];

    #[test]
    fn flags_bool_fields_tuples_and_string_errors_outside_trait_impls() {
        let file = SourceFile::parse("flags.rs".to_owned(), SOURCE.to_owned()).expect("valid rust");
        let report = TypeShape::report(core::slice::from_ref(&file)).to_string();
        assert!(
            REPORTED.iter().all(|line| report.contains(line)) && !report.contains(":8:"),
            "{report}"
        );
    }
}
