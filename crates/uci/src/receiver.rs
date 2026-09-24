use crate::command::{EngineOption, GoLimits, Position};

pub trait Receiver<'line>: Sized {
    #[must_use]
    fn identify(self) -> Self;

    #[must_use]
    fn confirm_ready(self) -> Self;

    #[must_use]
    fn configure(self, option: EngineOption<'line>) -> Self;

    #[must_use]
    fn reset_game(self) -> Self;

    #[must_use]
    fn place(self, position: Position<'line>) -> Self;

    #[must_use]
    fn start_search(self, limits: GoLimits) -> Self;

    #[must_use]
    fn halt(self) -> Self;

    #[must_use]
    fn shut_down(self) -> Self;
}
