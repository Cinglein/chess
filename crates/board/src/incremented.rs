pub trait Incremented {
    #[must_use]
    fn incremented(self) -> Self;
}
