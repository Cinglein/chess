mod bound_kind;
mod root_distance;
mod table_entry;

pub use bound_kind::BoundKind;
pub(crate) use root_distance::RootDistance;
pub use table_entry::TableEntry;

use board::{ChessMove, Zobrist};
use eval::Score;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Conclusion {
    best_move: Option<ChessMove>,
    score: Score,
    kind: BoundKind,
}

impl Conclusion {
    pub(crate) const fn new(
        best_move: Option<ChessMove>,
        score: Score,
        kind: BoundKind,
    ) -> Conclusion {
        Conclusion {
            best_move,
            score,
            kind,
        }
    }

    pub(crate) const fn score(&self) -> Score {
        self.score
    }
}

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
