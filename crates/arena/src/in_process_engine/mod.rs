mod captured_reply;

use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use captured_reply::CapturedReply;
use engine::Engine;
use uci::{Command, GoLimits, Position};

use crate::arena_error::ArenaError;
use crate::chosen_move::ChosenMove;
use crate::opponent::Opponent;

pub struct InProcessEngine {
    engine: Option<Engine<CapturedReply>>,
}

impl InProcessEngine {
    fn deliver(&mut self, command: Command<'_>) {
        self.engine = self.engine.take().map(|engine| command.deliver_to(engine));
    }
}

impl Default for InProcessEngine {
    fn default() -> InProcessEngine {
        InProcessEngine {
            engine: Some(Engine::new(
                Arc::new(AtomicBool::new(false)),
                CapturedReply::default(),
            )),
        }
    }
}

impl Opponent for InProcessEngine {
    fn begin_game(&mut self) -> Result<(), ArenaError> {
        self.deliver(Command::UciNewGame);
        Ok(())
    }

    fn choose_move(
        &mut self,
        position: Position<'_>,
        limits: GoLimits,
    ) -> Result<ChosenMove, ArenaError> {
        self.deliver(Command::Position(position));
        self.deliver(Command::Go(limits));
        self.engine
            .as_ref()
            .and_then(|engine| {
                ChosenMove::from_best_move(engine.sink().best_move(), engine.sink().score())
            })
            .ok_or(ArenaError::NoMove)
    }
}
