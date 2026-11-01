use std::ffi::{OsStr, OsString};

#[derive(Clone, Copy)]
pub(super) enum Note {
    Failing,
    Green,
}

impl Note {
    pub(super) fn comment_on(self, run_url: &OsStr) -> OsString {
        let lead = match self {
            Self::Failing => "Still failing: ",
            Self::Green => "Green again: ",
        };
        [OsStr::new(lead), run_url].into_iter().collect()
    }
}
