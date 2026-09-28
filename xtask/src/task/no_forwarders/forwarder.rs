use syn::Ident;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Forwarder {
    name: Ident,
    callee: String,
}

impl Forwarder {
    pub fn new(name: Ident, callee: String) -> Forwarder {
        Forwarder { name, callee }
    }

    pub fn name(&self) -> &Ident {
        &self.name
    }

    pub fn callee(&self) -> &str {
        &self.callee
    }
}
