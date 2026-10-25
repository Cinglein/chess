use syn::ItemImpl;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Walk {
    Entered,
    Skipped,
}

impl Walk {
    #[must_use]
    pub fn inherent_impls(item: &ItemImpl) -> Walk {
        match item.trait_ {
            None => Walk::Entered,
            Some(_) => Walk::Skipped,
        }
    }
}
