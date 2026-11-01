mod line_reader;

use std::io::Write;
use std::iter;
use std::ops::ControlFlow;
use std::path::Path;
use std::process::{Child, ChildStdin, Command as ProcessCommand, Stdio};
use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant};

use board::Color;
use line_reader::LineReader;
use uci::{Command, EngineOption, GoLimits, Position, Remaining, Response};

use game::ArenaError;
use game::ChosenMove;
use game::Opponent;

#[must_use]
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
        process.await_response(Self::HANDSHAKE, (), |(), response| {
            if response == Response::UciOk {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
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
        self.await_response(Self::HANDSHAKE, (), |(), response| {
            if response == Response::ReadyOk {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        })
    }

    fn await_response<T, S>(
        &self,
        within: Duration,
        seed: S,
        mut step: impl FnMut(S, Response<'_>) -> ControlFlow<T, S>,
    ) -> Result<T, ArenaError> {
        let deadline = Instant::now() + within;
        iter::repeat_with(|| {
            self.lines
                .recv_timeout(deadline.saturating_duration_since(Instant::now()))
        })
        .try_fold(seed, |state, line| {
            match line.as_deref().map(Response::try_from) {
                Err(_) => ControlFlow::Break(Err(ArenaError::Unresponsive)),
                Ok(Err(_)) => ControlFlow::Continue(state),
                Ok(Ok(response)) => step(state, response).map_break(Ok),
            }
        })
        .break_value()
        .unwrap_or(Err(ArenaError::Unresponsive))
    }

    fn allowance(limits: &GoLimits) -> Duration {
        limits
            .move_time()
            .or_else(|| {
                limits.clock().map(|clock| {
                    clock
                        .time::<Remaining>(Color::White)
                        .max(clock.time::<Remaining>(Color::Black))
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
    ) -> Result<ChosenMove, ArenaError> {
        self.send(Command::Position(position))?;
        self.send(Command::Go(limits))?;
        self.await_response(Self::allowance(&limits), None, |score, response| {
            let score = response.info().map(|info| *info.as_ref()).or(score);
            response
                .best_move()
                .map_or(ControlFlow::Continue(score), |notation| {
                    ControlFlow::Break(ChosenMove::from_best_move(notation, score))
                })
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
