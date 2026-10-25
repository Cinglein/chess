#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ToolScope {
    ProductOnly,
    PlusTheTool,
    SmokeCrate,
}
