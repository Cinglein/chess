use strum::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display)]
#[repr(u8)]
pub enum MutantsWord {
    #[strum(serialize = "--jobs")]
    Jobs,
    #[strum(serialize = "--in-diff")]
    InDiff,
    #[strum(serialize = "--in-place")]
    InPlace,
    #[strum(serialize = "--output")]
    Output,
    #[strum(serialize = "--shard")]
    Shard,
    #[strum(serialize = "--list")]
    List,
    #[strum(serialize = "--jobserver-tasks")]
    JobserverTasks,
}
