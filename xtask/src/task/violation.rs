use std::fmt;

use crate::task::site::Site;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Violation {
    site: Site,
    message: String,
}

impl Violation {
    pub fn new(site: Site, message: impl Into<String>) -> Violation {
        Violation {
            site,
            message: message.into(),
        }
    }
}

impl fmt::Display for Violation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.site, self.message)
    }
}
