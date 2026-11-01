use std::str::FromStr;

#[derive(Clone, Copy)]
pub(super) enum Branch {
    Main,
    Other,
}

impl Branch {
    pub(super) fn on_main<T>(self, work: impl FnOnce() -> T) -> Option<T> {
        match self {
            Self::Main => Some(work()),
            Self::Other => None,
        }
    }
}

impl FromStr for Branch {
    type Err = ();

    fn from_str(text: &str) -> Result<Self, ()> {
        match text {
            "true" => Ok(Self::Main),
            "false" => Ok(Self::Other),
            _ => Err(()),
        }
    }
}
