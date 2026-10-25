use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct ShardCount(usize);

impl ShardCount {
    pub const fn new(shards: usize) -> ShardCount {
        ShardCount(shards)
    }
}
