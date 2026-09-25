mod captured_move;

use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use board::LongAlgebraic;
use captured_move::CapturedMove;
use engine::Engine;
use uci::{Command, GoLimits, Position};

use crate::arena_error::ArenaError;
use crate::opponent::Opponent;

pub struct Native {
    engine: Option<Engine<CapturedMove>>,
}

impl Native {
    fn deliver(&mut self, command: Command<'_>) {
        self.engine = self.engine.take().map(|engine| command.deliver_to(engine));
    }
}

impl Default for Native {
    fn default() -> Native {
        Native {
            engine: Some(Engine::new(
                Arc::new(AtomicBool::new(false)),
                CapturedMove::default(),
            )),
        }
    }
}

impl Opponent for Native {
    fn begin_game(&mut self) -> Result<(), ArenaError> {
        self.deliver(Command::UciNewGame);
        Ok(())
    }

    fn choose_move(
        &mut self,
        position: Position<'_>,
        limits: GoLimits,
    ) -> Result<LongAlgebraic, ArenaError> {
        self.deliver(Command::Position(position));
        self.deliver(Command::Go(limits));
        self.engine
            .as_ref()
            .and_then(|engine| engine.sink().best_move())
            .ok_or(ArenaError::NoMove)
    }
}

#[cfg(test)]
mod tests {
    use search::Depth;
    use uci::{GoLimits, Position};

    use super::{Native, Opponent};

    const MATE_IN_ONE: &str = "6k1/5ppp/8/8/8/8/5PPP/R5K1 w - - 0 1";

    #[test]
    fn the_native_opponent_answers_a_position_with_the_mating_move() {
        let mut native = Native::default();
        native.begin_game().unwrap();
        let chosen = native
            .choose_move(
                Position::new(Some(MATE_IN_ONE), ""),
                GoLimits::Depth(Depth::new(2)),
            )
            .unwrap();
        assert_eq!(chosen.to_string(), "a1a8");
    }
}
