#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BoundKind {
    Exact,
    AtMost,
    AtLeast,
}
