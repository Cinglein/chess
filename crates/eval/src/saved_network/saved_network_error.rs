use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Error)]
pub enum SavedNetworkError {
    #[error("a saved network is {expected_bytes} bytes, found {found_bytes}")]
    Length {
        expected_bytes: usize,
        found_bytes: usize,
    },
}
