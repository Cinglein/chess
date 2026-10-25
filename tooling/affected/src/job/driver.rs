#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Driver {
    Cargo,
    CargoMutants,
}
