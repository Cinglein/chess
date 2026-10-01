mod bound_kind;
mod conclusion;
mod root_distance;
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
        self.slot_index(hash)
            .and_then(|index| self.entries.get(index))
            .copied()
            .filter(|entry| entry.hash() == hash)
    }

    pub(crate) fn store(&mut self, entry: TableEntry) {
        let Some(index) = self.slot_index(entry.hash()) else {
            return;
        };
        if let Some(slot) = self.entries.get_mut(index)
            && (slot.hash() != entry.hash() || slot.depth() <= entry.depth())
        {
            *slot = entry;
        }
    }

    fn slot_index(&self, hash: Zobrist) -> Option<usize> {
        let count = u64::try_from(self.entries.len()).ok()?;
        usize::try_from(hash.bits().checked_rem(count)?).ok()
    }
}

#[cfg(test)]
mod tests {
    use board::{Board, Zobrist};
    use eval::Score;

    use super::{BoundKind, Conclusion, RootDistance, TableEntry, TranspositionTable};
    use crate::search::depth::Depth;

    const DEEP: Depth = Depth::new(2);
    const SHALLOW: Depth = Depth::new(1);

    impl TableEntry {
        fn exact(hash: Zobrist, depth: Depth) -> TableEntry {
            let conclusion = Conclusion::new(None, Score::DRAW, BoundKind::Exact);
            TableEntry::remember(hash, depth, RootDistance::ROOT, conclusion)
        }
    }

    #[test]
    fn a_slot_keeps_the_deeper_entry_for_its_position_and_yields_to_any_other_position() {
        let start = Board::START.hash();
        let moved = Board::START
            .make_move(Board::START.legal_moves()[0])
            .unwrap()
            .hash();
        let mut store = [TableEntry::EMPTY];
        let mut table = TranspositionTable::new(&mut store);
        table.store(TableEntry::exact(start, DEEP));
        table.store(TableEntry::exact(start, SHALLOW));
        assert_eq!(table.probe(start).map(|entry| entry.depth()), Some(DEEP));
        table.store(TableEntry::exact(moved, Depth::ZERO));
        assert_eq!(
            (
                table.probe(start),
                table.probe(moved).map(|entry| entry.depth())
            ),
            (None, Some(Depth::ZERO))
        );
    }
}
