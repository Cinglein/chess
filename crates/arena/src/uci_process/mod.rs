mod line_reader;

use std::io::Write;
use std::path::Path;
use std::process::{Child, ChildStdin, Command as ProcessCommand, Stdio};
use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant};

use board::{Color, LongAlgebraic};
use line_reader::LineReader;
use uci::{Command, EngineOption, GoLimits, Position, Response};

use crate::arena_error::ArenaError;
use crate::opponent::Opponent;

pub struct UciProcess {
    child: Child,
    stdin: ChildStdin,
    lines: Receiver<String>,
}

impl UciProcess {
    const HANDSHAKE: Duration = Duration::from_secs(10);
    const GRACE: Duration = Duration::from_secs(2);
    const UNBOUNDED: Duration = Duration::from_hours(1);

    pub fn spawn(program: &Path, options: &[EngineOption<'_>]) -> Result<UciProcess, ArenaError> {
        let mut child = ProcessCommand::new(program)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(ArenaError::Spawn)?;
        let stdin = child.stdin.take().ok_or(ArenaError::Disconnected)?;
        let stdout = child.stdout.take().ok_or(ArenaError::Disconnected)?;
        let mut process = UciProcess {
            child,
            stdin,
            lines: LineReader::spawn(stdout),
        };
        process.send(Command::Uci)?;
        process.await_response(Self::HANDSHAKE, |response| {
            (response == Response::UciOk).then_some(())
        })?;
        options
            .iter()
            .try_for_each(|option| process.send(Command::SetOption(*option)))?;
        process.synchronise()?;
        Ok(process)
    }

    fn send(&mut self, command: Command<'_>) -> Result<(), ArenaError> {
        writeln!(self.stdin, "{command}").map_err(ArenaError::Write)
    }

    fn synchronise(&mut self) -> Result<(), ArenaError> {
        self.send(Command::IsReady)?;
        self.await_response(Self::HANDSHAKE, |response| {
            (response == Response::ReadyOk).then_some(())
        })
    }

    fn await_response<T>(
        &self,
        within: Duration,
        pick: impl Fn(Response<'_>) -> Option<T>,
    ) -> Result<T, ArenaError> {
        let deadline = Instant::now() + within;
        loop {
            let line = self
                .lines
                .recv_timeout(deadline.saturating_duration_since(Instant::now()))
                .map_err(|_| ArenaError::Unresponsive)?;
            if let Some(picked) = Response::try_from(line.as_str()).ok().and_then(&pick) {
                return Ok(picked);
            }
        }
    }

    fn allowance(limits: &GoLimits) -> Duration {
        limits
            .move_time()
            .or_else(|| {
                limits.clock().map(|clock| {
                    clock
                        .remaining(Color::White)
                        .max(clock.remaining(Color::Black))
                })
            })
            .map_or(Self::UNBOUNDED, |budget| budget + Self::GRACE)
    }
}

impl Opponent for UciProcess {
    fn begin_game(&mut self) -> Result<(), ArenaError> {
        self.send(Command::UciNewGame)?;
        self.synchronise()
    }

    fn choose_move(
        &mut self,
        position: Position<'_>,
        limits: GoLimits,
    ) -> Result<LongAlgebraic, ArenaError> {
        self.send(Command::Position(position))?;
        self.send(Command::Go(limits))?;
        self.await_response(Self::allowance(&limits), |response| {
            response.is_best_move().then(|| response.best_move())
        })?
        .ok_or(ArenaError::NoMove)
    }
}

impl Drop for UciProcess {
    fn drop(&mut self) {
        let _ = self.send(Command::Quit);
        let _ = self.child.wait();
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::time::Duration;

    use board::Board;
    use uci::{EngineOption, GoLimits, Position};

    use super::{Opponent, UciProcess};

    const PROGRAM: &str = "stockfish";
    const START: Position<'static> = Position::new(None, "");
    const THINK: GoLimits = GoLimits::MoveTime(Duration::from_millis(50));
    const LIMITED: [EngineOption<'static>; 2] = [
        EngineOption::new("UCI_LimitStrength", "true"),
        EngineOption::new("UCI_Elo", "1320"),
    ];

    #[test]
    #[ignore = "needs stockfish on the path"]
    fn stockfish_completes_the_handshake_and_answers_with_a_legal_move() {
        let mut stockfish = UciProcess::spawn(Path::new(PROGRAM), &LIMITED).unwrap();
        stockfish.begin_game().unwrap();
        let chosen = stockfish.choose_move(START, THINK).unwrap();
        assert!(Board::START.resolve_move(chosen).is_some());
    }
}
