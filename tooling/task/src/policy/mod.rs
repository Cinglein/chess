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

use crate::available_gib::AvailableGib;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Policy {
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
    #[must_use]
    pub fn limit(&self, limit: Limit) -> Most {
        self.limits[limit]
    }

    #[must_use]
    pub fn floor(&self, floor: Floor) -> Least {
        self.floors[floor]
    }

    #[must_use]
    pub fn share(&self, share: Share) -> Percent {
        self.shares[share]
    }

    #[must_use]
    pub fn memory(&self, share: MemoryShare) -> AvailableGib {
        self.memory_gib[share]
    }

    #[must_use]
    pub fn interval(&self, interval: Interval) -> Millis {
        self.millis[interval]
    }

    #[must_use]
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
