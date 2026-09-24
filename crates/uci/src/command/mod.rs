mod go_limits;
mod position;
mod uci_error;

pub use go_limits::{Clock, GoLimits};
pub use position::Position;
pub use uci_error::UciError;

use crate::receiver::Receiver;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Command<'line> {
    Uci,
    IsReady,
    UciNewGame,
    Position(Position<'line>),
    Go(GoLimits),
    Stop,
    Quit,
}

impl<'line> Command<'line> {
    pub fn deliver_to<R: Receiver<'line>>(self, receiver: R) -> R {
        match self {
            Command::Uci => receiver.identify(),
            Command::IsReady => receiver.confirm_ready(),
            Command::UciNewGame => receiver.reset_game(),
            Command::Position(position) => receiver.place(position),
            Command::Go(limits) => receiver.start_search(limits),
            Command::Stop => receiver.halt(),
            Command::Quit => receiver.shut_down(),
        }
    }
}

impl<'line> TryFrom<&'line str> for Command<'line> {
    type Error = UciError;

    fn try_from(line: &'line str) -> Result<Command<'line>, UciError> {
        let trimmed = line.trim();
        let (head, rest) = trimmed
            .split_once(char::is_whitespace)
            .unwrap_or((trimmed, ""));
        match head {
            "uci" => Ok(Command::Uci),
            "isready" => Ok(Command::IsReady),
            "ucinewgame" => Ok(Command::UciNewGame),
            "position" => Position::try_from(rest).map(Command::Position),
            "go" => Ok(Command::Go(GoLimits::from(rest))),
            "stop" => Ok(Command::Stop),
            "quit" => Ok(Command::Quit),
            _ => Err(UciError::UnknownCommand),
        }
    }
}

#[cfg(test)]
mod tests {
    use core::time::Duration;

    use board::Color;

    use super::Command;
    use crate::command::{GoLimits, Position};
    use crate::receiver::Receiver;

    const GO: &str = "go wtime 60 btime 40 winc 1 binc 1";
    const POSITION: &str = "position startpos moves e2e4 e7e5";

    #[derive(Debug, PartialEq, Eq)]
    struct Trace(&'static str);

    impl Receiver<'_> for Trace {
        fn identify(self) -> Self {
            Trace("identify")
        }

        fn confirm_ready(self) -> Self {
            Trace("confirm_ready")
        }

        fn reset_game(self) -> Self {
            Trace("reset_game")
        }

        fn place(self, _: Position<'_>) -> Self {
            Trace("place")
        }

        fn start_search(self, _: GoLimits) -> Self {
            Trace("start_search")
        }

        fn halt(self) -> Self {
            Trace("halt")
        }

        fn shut_down(self) -> Self {
            Trace("shut_down")
        }
    }

    #[test]
    fn a_clock_go_command_parses_both_sides_and_is_delivered_as_a_search() {
        let command = Command::try_from(GO).unwrap();
        let Command::Go(limits) = command else {
            unreachable!();
        };
        let clock = limits.clock().unwrap();
        assert_eq!(clock.remaining(Color::Black), Duration::from_millis(40));
        assert_eq!(command.deliver_to(Trace("halt")), Trace("start_search"));
    }

    #[test]
    fn a_position_command_carries_its_moves_and_unknown_words_are_rejected() {
        let Command::Position(position) = Command::try_from(POSITION).unwrap() else {
            panic!("{POSITION}");
        };
        assert_eq!((position.fen(), position.moves().count()), (None, 2));
        assert!(Command::try_from("dance").is_err());
    }
}
