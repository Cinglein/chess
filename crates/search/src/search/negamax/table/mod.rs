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
        self.slot_index(entry.hash())
            .and_then(|index| self.entries.get_mut(index))
            .filter(|slot| slot.hash() != entry.hash() || slot.depth() <= entry.depth())
            .into_iter()
            .for_each(|slot| *slot = entry);
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
    const SLOTS: usize = 2;

    impl TableEntry {
        fn exact(hash: Zobrist, depth: Depth) -> TableEntry {
            let conclusion = Conclusion::new(None, Score::DRAW, BoundKind::Exact);
            TableEntry::remember(hash, depth, RootDistance::ROOT, conclusion)
        }

        fn child_hash_sharing_the_slot(shared: bool) -> Zobrist {
            let slot_of = |hash: Zobrist| usize::try_from(hash.bits()).unwrap_or_default() % SLOTS;
            Board::START
                .legal_moves()
                .into_iter()
                .map(|chess_move| Board::START.make_move(chess_move).unwrap().hash())
                .find(|hash| (slot_of(*hash) == slot_of(Board::START.hash())) == shared)
                .unwrap()
        }
    }

    #[test]
    fn a_slot_keeps_the_deeper_entry_for_its_position_and_yields_only_to_a_position_sharing_it() {
        let start = Board::START.hash();
        let (elsewhere, sharing) = (
            TableEntry::child_hash_sharing_the_slot(false),
            TableEntry::child_hash_sharing_the_slot(true),
        );
        let mut store = [TableEntry::EMPTY; SLOTS];
        let mut table = TranspositionTable::new(&mut store);
        table.store(TableEntry::exact(start, DEEP));
        table.store(TableEntry::exact(start, SHALLOW));
        table.store(TableEntry::exact(elsewhere, Depth::ZERO));
        let depth_of = |hash| table.probe(hash).map(|entry| entry.depth());
        assert_eq!(
            (depth_of(start), depth_of(elsewhere)),
            (Some(DEEP), Some(Depth::ZERO))
        );
        table.store(TableEntry::exact(sharing, Depth::ZERO));
        assert_eq!(
            (table.probe(start), table.probe(sharing).is_some()),
            (None, true)
        );
    }
}
