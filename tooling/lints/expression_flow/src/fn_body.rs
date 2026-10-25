use syn::{Block, Signature};

pub enum FnBody<'ast> {
    Scanned(&'ast Block),
    Skipped,
}

impl<'ast> FnBody<'ast> {
    pub fn scanned_unless_const(signature: &Signature, block: Option<&'ast Block>) -> FnBody<'ast> {
        match (signature.constness, block) {
            (None, Some(block)) => FnBody::Scanned(block),
            _ => FnBody::Skipped,
        }
    }
}
