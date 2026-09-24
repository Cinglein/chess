use board::Zobrist;

use crate::table_entry::TableEntry;

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
