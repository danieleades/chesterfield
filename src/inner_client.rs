use crate::error::{Error, UrlError};
use reqwest::Url;
use std::sync::Arc;

pub(crate) struct InnerClient {
    url: Url,
    http_client: Arc<reqwest::Client>,
}

impl InnerClient {
    pub fn new(url: impl AsRef<str>) -> Result<Self, Error> {
        let url = Url::parse(url.as_ref())?;
        let client = reqwest::ClientBuilder::new().build()?;
        let http_client = Arc::new(client);

        Ok(InnerClient { url, http_client })
    }

    pub fn join(&self, name: impl AsRef<str>) -> Result<Self, UrlError> {
        let url = self.url.join(name.as_ref())?;
        let http_client = Arc::clone(&self.http_client);

        Ok(InnerClient { url, http_client })
    }

    pub fn get(&self) -> reqwest::RequestBuilder {
        self.http_client.get(self.url.clone())
    }

    pub fn post(&self) -> reqwest::RequestBuilder {
        self.http_client.post(self.url.clone())
    }
}
