pub use reqwest::UrlError;

#[derive(Debug)]
/// A catch-all error type for everything that can (and does, currently)
/// go wrong with this library
pub enum ChesterfieldError {
    /// An error reported by the underlying reqwest library.
    Reqwest(reqwest::Error),

    /// An error related to the parsing of a URL.
    Url(reqwest::UrlError),
}

impl From<reqwest::Error> for ChesterfieldError {
    fn from(e: reqwest::Error) -> Self {
        ChesterfieldError::Reqwest(e)
    }
}

impl From<reqwest::UrlError> for ChesterfieldError {
    fn from(e: reqwest::UrlError) -> Self {
        ChesterfieldError::Url(e)
    }
}

impl std::error::Error for ChesterfieldError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ChesterfieldError::Reqwest(e) => Some(e),
            ChesterfieldError::Url(e) => Some(e),
        }
    }
}

impl std::fmt::Display for ChesterfieldError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ChesterfieldError::Reqwest(e) => write!(f, "reqwest error: {}", e),
            ChesterfieldError::Url(e) => write!(f, "url error: {}", e),
        }
    }
}
