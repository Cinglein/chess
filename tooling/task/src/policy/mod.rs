mod floor;
mod interval;
mod least;
mod limit;
mod memory_share;
mod millis;
mod most;
mod mutant_millis;
mod percent;
mod share;
mod vague;
mod vocabulary;
mod wire_type;

pub use floor::Floor;
pub use interval::Interval;
pub use limit::Limit;
pub use memory_share::MemoryShare;
pub use millis::Millis;
pub use mutant_millis::MutantMillis;
pub use share::Share;
pub use wire_type::WireType;

use enum_map::EnumMap;
use least::Least;
use most::Most;
use percent::Percent;
use serde::{Deserialize, Serialize};
use settings::EmbeddedDefaults;
use vague::Vague;
use vocabulary::Vocabulary;

use crate::available_gib::AvailableGib;
use crate::name::Name;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Policy {
    vague: Vec<String>,
    wire_types: Vec<String>,
    limits: EnumMap<Limit, Most>,
    floors: EnumMap<Floor, Least>,
    shares: EnumMap<Share, Percent>,
    memory_gib: EnumMap<MemoryShare, AvailableGib>,
    millis: EnumMap<Interval, Millis>,
    mutant_millis: MutantMillis,
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
    pub fn mutant_millis(&self) -> &MutantMillis {
        &self.mutant_millis
    }

    #[must_use]
    pub fn is_vague(&self, name: &Name) -> bool {
        !self.floor(Floor::FnName).allows(name.length()) || self.lists::<Vague>(name)
    }

    #[must_use]
    pub fn lists<V: Vocabulary>(&self, word: &impl PartialEq<String>) -> bool {
        V::entries(self).iter().any(|entry| word == entry)
    }
}

#[cfg(test)]
mod tests {
    use super::{Limit, Name, Policy};
    use settings::EmbeddedDefaults;
    const CHECK: &str = "check";
    const LONG_ENOUGH: &str = "report";

    #[test]
    fn the_checked_in_policy_round_trips_and_knows_a_vague_name_from_a_said_one() {
        let written = toml::to_string_pretty(&Policy::defaults()).unwrap();
        let read: Policy = toml::from_str(&written).unwrap();
        assert_eq!(read, Policy::defaults());
        let costs = read.mutant_millis();
        assert!(
            read.is_vague(&Name::from(CHECK))
                && !read.is_vague(&Name::from(LONG_ENOUGH))
                && costs.for_crate(&"engine".parse().unwrap())
                    != costs.for_crate(&"unlisted".parse().unwrap())
        );
        assert!(
            read.limit(Limit::Parameters)
                .allows(read.limit(Limit::Parameters).count())
        );
    }
}
