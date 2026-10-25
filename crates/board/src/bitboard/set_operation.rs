#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SetOperation {
    Union,
    DisjointUnion,
    Intersection,
    Difference,
}
