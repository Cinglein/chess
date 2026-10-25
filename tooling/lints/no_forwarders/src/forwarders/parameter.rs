use syn::{FnArg, Pat, Path};

use super::struct_fields::StructFields;

pub struct Parameter {
    name: String,
    type_name: Option<String>,
}

impl Parameter {
    pub fn describe(input: &FnArg) -> Option<Parameter> {
        match input {
            FnArg::Receiver(_) => Some(Parameter {
                name: String::from("self"),
                type_name: None,
            }),
            FnArg::Typed(typed) => match &*typed.pat {
                Pat::Ident(ident) => Some(Parameter {
                    name: ident.ident.to_string(),
                    type_name: StructFields::type_name(&typed.ty),
                }),
                _ => None,
            },
        }
    }

    pub fn named_by<'params>(
        params: &'params [Parameter],
        path: &Path,
    ) -> Option<&'params Parameter> {
        let ident = path.get_ident()?.to_string();
        params.iter().find(|parameter| parameter.name == ident)
    }

    pub fn type_name(&self) -> Option<&str> {
        self.type_name.as_deref()
    }
}
