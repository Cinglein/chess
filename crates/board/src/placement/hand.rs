use core::fmt::Debug;
use core::hash::Hash;

pub trait Hand: Copy + Debug + Eq + Hash {}
