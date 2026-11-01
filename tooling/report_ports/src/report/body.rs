use std::ffi::OsString;
use std::fmt;

use super::missed::Missed;

pub(super) struct Body {
    run_url: OsString,
    missed: Missed,
}

impl Body {
    pub(super) const fn new(run_url: OsString, missed: Missed) -> Self {
        Self { run_url, missed }
    }
}

impl fmt::Display for Body {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            out,
            "Run: {}\n\n```\n{}```\n",
            self.run_url.display(),
            self.missed
        )
    }
}
