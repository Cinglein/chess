mod bound_kind;
mod conclusion;
mod remembered;
mod root_distance;
mod table_entry;

pub use bound_kind::BoundKind;
pub(crate) use conclusion::Conclusion;
pub(super) use remembered::Remembered;
pub(crate) use root_distance::RootDistance;
pub use table_entry::TableEntry;

use board::Zobrist;

pub struct TranspositionTable<'store> {
    entries: &'store mut [TableEntry],
}

impl<'store> TranspositionTable<'store> {
    #[must_use]
    pub fn new(entries: &'store mut [TableEntry]) -> TranspositionTable<'store> {
        TranspositionTable { entries }
    }

    pub(crate) fn probe(&self, hash: Zobrist) -> Option<TableEntry> {
        self.slot_index(hash)
            .and_then(|index| self.entries.get(index))
            .copied()
            .filter(|entry| Zobrist::from(*entry) == hash)
    }

    pub(crate) fn store(&mut self, entry: TableEntry) {
        self.slot_index(Zobrist::from(entry))
            .and_then(|index| self.entries.get_mut(index))
            .filter(|slot| {
                Zobrist::from(**slot) != Zobrist::from(entry) || slot.depth() <= entry.depth()
            })
            .into_iter()
            .for_each(|slot| *slot = entry);
    }

    fn slot_index(&self, hash: Zobrist) -> Option<usize> {
        let count = u64::try_from(self.entries.len()).ok()?;
        usize::try_from(hash.bits().checked_rem(count)?).ok()
    }
}
