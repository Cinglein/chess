mod line_reader;

use std::io::Write;
use std::iter;
use std::ops::ControlFlow;
use std::process::{Child, ChildStdin, Command as ProcessCommand, Stdio};
use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant};

use board::Color;
use line_reader::LineReader;
use uci::{Command, EngineOption, GoLimits, Position, Response};

use crate::arena_error::ArenaError;
use crate::chosen_move::ChosenMove;
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

    pub fn spawn(
        mut program: ProcessCommand,
        options: &[EngineOption<'_>],
    ) -> Result<UciProcess, ArenaError> {
        let mut child = program
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
    ) -> Result<ChosenMove, ArenaError> {
        self.send(Command::Position(position))?;
        self.send(Command::Go(limits))?;
        self.await_response(Self::allowance(&limits), None, |score, response| {
            let score = response.info().map(|info| info.score()).or(score);
            if response.is_best_move() {
                ControlFlow::Break(ChosenMove::from_best_move(response.best_move(), score))
            } else {
                ControlFlow::Continue(score)
            }
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
    use std::env;
    use std::io::{self, BufRead};
    use std::ops::ControlFlow;
    use std::process::Command as ProcessCommand;
    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;
    use std::thread;
    use std::time::{Duration, Instant};

    use board::Board;
    use engine::{Engine, Sink};
    use uci::{Command, EngineOption, GoLimits, Position, Response, Switch};

    use super::{Opponent, UciProcess};

    const PROGRAM: &str = "stockfish";
    const MATE_IN_ONE: &str = "6k1/5ppp/8/8/8/8/5PPP/R5K1 w - - 0 1";
    const MATING_MOVE: &str = "a1a8";
    const START: Position<'static> = Position::played(Board::START, &[]);
    const THINK: GoLimits = GoLimits::MoveTime(Duration::from_millis(50));
    const LIMITED: [EngineOption<'static>; 1] = [EngineOption::LimitStrength(Switch::True)];

    struct EngineProcess;

    impl EngineProcess {
        const ENTRY: &str = "ARENA_ENGINE_PROCESS";
        const ENTRY_TEST: &str =
            "uci_process::tests::the_engine_process_serves_uci_over_its_standard_streams";
        const SETUP: Duration = Duration::from_millis(20);

        fn command() -> ProcessCommand {
            let mut command = ProcessCommand::new(env::current_exe().unwrap());
            command
                .args(["--ignored", "--exact", Self::ENTRY_TEST, "--nocapture"])
                .env(Self::ENTRY, "1");
            command
        }

        fn serve() {
            let engine = Engine::new(Arc::new(AtomicBool::new(false)), EngineProcess);
            let _finished = io::stdin().lock().lines().map_while(Result::ok).try_fold(
                engine,
                |engine, line| {
                    let engine = match Command::try_from(line.as_str()) {
                        Ok(command) => command.deliver_to(engine),
                        Err(_) => engine,
                    };
                    if engine.is_ending() {
                        ControlFlow::Break(engine)
                    } else {
                        ControlFlow::Continue(engine)
                    }
                },
            );
        }
    }

    impl Sink for EngineProcess {
        fn emit(&mut self, response: Response<'_>) {
            if response == Response::ReadyOk {
                thread::sleep(Self::SETUP);
            }
            let acted_on = response.is_best_move()
                || response.info().is_some()
                || response == Response::UciOk
                || response == Response::ReadyOk;
            if acted_on {
                println!("{response}");
            }
        }
    }

    #[test]
    #[ignore = "the engine behind the handshake test, which spawns it with ARENA_ENGINE_PROCESS set"]
    fn the_engine_process_serves_uci_over_its_standard_streams() {
        if env::var_os(EngineProcess::ENTRY).is_some() {
            EngineProcess::serve();
        }
    }

    #[test]
    fn a_spawned_engine_is_ready_before_the_game_starts_and_answers_a_timed_go_with_the_mate() {
        let mut engine = UciProcess::spawn(EngineProcess::command(), &LIMITED).unwrap();
        let started = Instant::now();
        engine.begin_game().unwrap();
        assert!(started.elapsed() >= EngineProcess::SETUP);
        let position = Position::played(MATE_IN_ONE.parse().unwrap(), &[]);
        let chosen = engine.choose_move(position, THINK).unwrap();
        assert_eq!(chosen.notation().to_string(), MATING_MOVE);
    }

    #[test]
    #[ignore = "needs stockfish on the path"]
    fn stockfish_completes_the_handshake_and_answers_with_a_legal_move() {
        let mut stockfish = UciProcess::spawn(ProcessCommand::new(PROGRAM), &LIMITED).unwrap();
        stockfish.begin_game().unwrap();
        let chosen = stockfish.choose_move(START, THINK).unwrap();
        assert!(Board::START.resolve_move(chosen.notation()).is_some());
    }
}
