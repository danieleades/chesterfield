pub use reqwest::UrlError;

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Url(reqwest::UrlError),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl From<reqwest::UrlError> for Error {
    fn from(e: reqwest::UrlError) -> Self {
        Error::Url(e)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Reqwest(e) => Some(e),
            Error::Url(e) => Some(e),
        }
    }
}

impl std::fmt::Display for Error {
    // TODO
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "chesterfield error")
    }
}
