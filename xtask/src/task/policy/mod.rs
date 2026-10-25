mod floor;
mod interval;
mod least;
mod limit;
mod memory_share;
mod millis;
mod most;
mod percent;
mod share;

pub use floor::Floor;
pub use interval::Interval;
pub use limit::Limit;
pub use memory_share::MemoryShare;
pub use share::Share;

use enum_map::EnumMap;
use least::Least;
use millis::Millis;
use most::Most;
use percent::Percent;
use serde::{Deserialize, Serialize};
use settings::EmbeddedDefaults;

use crate::task::available_gib::AvailableGib;
use crate::task::crate_name::CrateName;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Policy {
    smoke_crate: CrateName,
    vague: Vec<String>,
    limits: EnumMap<Limit, Most>,
    floors: EnumMap<Floor, Least>,
    shares: EnumMap<Share, Percent>,
    memory_gib: EnumMap<MemoryShare, AvailableGib>,
    millis: EnumMap<Interval, Millis>,
}

impl EmbeddedDefaults for Policy {
    const DEFAULTS: &str = include_str!("../../../policy.toml");
}

impl Policy {
    pub fn limit(&self, limit: Limit) -> Most {
        self.limits[limit]
    }

    pub fn floor(&self, floor: Floor) -> Least {
        self.floors[floor]
    }

    pub fn share(&self, share: Share) -> Percent {
        self.shares[share]
    }

    pub fn memory(&self, share: MemoryShare) -> AvailableGib {
        self.memory_gib[share]
    }

    pub fn interval(&self, interval: Interval) -> Millis {
        self.millis[interval]
    }

    pub fn smoke_crate(&self) -> &CrateName {
        &self.smoke_crate
    }

    pub fn is_vague(&self, name: &str) -> bool {
        !self.floor(Floor::FnName).allows(name.len())
            || self.vague.iter().any(|vague| vague == name)
    }
}

#[cfg(test)]
mod tests {
    use settings::EmbeddedDefaults;

    use super::{Limit, Policy};

    const CHECK: &str = "check";
    const LONG_ENOUGH: &str = "report";

    #[test]
    fn the_checked_in_policy_round_trips_and_knows_a_vague_name_from_a_said_one() {
        let written = toml::to_string_pretty(&Policy::defaults()).unwrap();
        let read: Policy = toml::from_str(&written).unwrap();
        assert_eq!(read, Policy::defaults());
        assert!(read.is_vague(CHECK) && !read.is_vague(LONG_ENOUGH));
        assert!(
            read.limit(Limit::Parameters)
                .allows(read.limit(Limit::Parameters).count())
        );
    }
}
