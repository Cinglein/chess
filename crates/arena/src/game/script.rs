use std::cell::RefCell;
use std::collections::VecDeque;
use std::str::FromStr;

use board::LongAlgebraic;
use uci::{GoLimits, Position};

use crate::arena_error::ArenaError;
use crate::chosen_move::ChosenMove;
use crate::opponent::Opponent;

pub(super) struct Script {
    moves: RefCell<VecDeque<LongAlgebraic>>,
}

impl FromStr for Script {
    type Err = <LongAlgebraic as FromStr>::Err;

    fn from_str(text: &str) -> Result<Script, Self::Err> {
        text.split_whitespace()
            .map(str::parse)
            .collect::<Result<VecDeque<LongAlgebraic>, Self::Err>>()
            .map(|moves| Script {
                moves: RefCell::new(moves),
            })
    }
}

impl Opponent for &Script {
    fn begin_game(&mut self) -> Result<(), ArenaError> {
        Ok(())
    }

    fn choose_move(&mut self, _: Position<'_>, _: GoLimits) -> Result<ChosenMove, ArenaError> {
        self.moves
            .borrow_mut()
            .pop_front()
            .map(|notation| ChosenMove::new(notation, None))
            .ok_or(ArenaError::NoMove)
    }
}
