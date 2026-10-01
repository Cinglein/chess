use std::process::ExitStatus;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct MutantsExit(i32);

impl MutantsExit {
    const CLEAN: MutantsExit = MutantsExit(0);
    const TIMEOUTS_ONLY: MutantsExit = MutantsExit(3);

    pub(super) fn accepts(status: ExitStatus) -> bool {
        status
            .code()
            .map(MutantsExit)
            .is_some_and(|exit| exit == Self::CLEAN || exit == Self::TIMEOUTS_ONLY)
    }
}
