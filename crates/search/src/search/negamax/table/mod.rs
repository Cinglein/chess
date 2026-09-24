mod bound_kind;
mod conclusion;
mod root_distance;
mod stored_score;
mod table_entry;

pub use bound_kind::BoundKind;
pub(crate) use conclusion::Conclusion;
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
        self.slot(hash)
            .and_then(|index| self.entries.get(index))
            .copied()
            .filter(|entry| entry.hash() == hash)
    }

    pub(crate) fn store(&mut self, entry: TableEntry) {
        let Some(index) = self.slot(entry.hash()) else {
            return;
        };
        if let Some(slot) = self.entries.get_mut(index)
            && (slot.hash() != entry.hash() || slot.depth() <= entry.depth())
        {
            *slot = entry;
        }
    }

    fn slot(&self, hash: Zobrist) -> Option<usize> {
        let count = u64::try_from(self.entries.len()).ok()?;
        usize::try_from(hash.bits().checked_rem(count)?).ok()
    }
}
