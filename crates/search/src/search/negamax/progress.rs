#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Progress {
    Running,
    Aborted,
}
