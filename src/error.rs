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
