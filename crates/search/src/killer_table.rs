use crate::killers::Killers;

#[derive(Clone, Copy, Debug)]
pub(crate) struct KillerTable([Killers; KillerTable::MAX_PLY]);

impl KillerTable {
    pub(crate) const MAX_PLY: usize = 128;

    pub(crate) const fn new() -> KillerTable {
        KillerTable([Killers::NONE; Self::MAX_PLY])
    }

    pub(crate) fn at_ply(&self, ply: u8) -> Killers {
        self.0
            .get(usize::from(ply))
            .copied()
            .unwrap_or(Killers::NONE)
    }

    pub(crate) fn remember(&mut self, ply: u8, chess_move: board::ChessMove) {
        if let Some(killers) = self.0.get_mut(usize::from(ply)) {
            *killers = killers.remembering(chess_move);
        }
    }
}
