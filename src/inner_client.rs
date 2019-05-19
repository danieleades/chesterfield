pub(crate) mod sync {

    use crate::{Error, UrlError};
    use reqwest::Url;
    use std::sync::Arc;

    pub struct InnerClient {
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

        pub fn put(&self) -> reqwest::RequestBuilder {
            self.http_client.put(self.url.clone())
        }

        pub fn delete(&self) -> reqwest::RequestBuilder {
            self.http_client.delete(self.url.clone())
        }

        pub fn head(&self) -> reqwest::RequestBuilder {
            self.http_client.head(self.url.clone())
        }

        pub fn url(&self) -> &Url {
            &self.url
        }
    }
}

pub(crate) mod r#async {

    use crate::{Error, UrlError};
    use reqwest::Url;
    use std::sync::Arc;

    pub(crate) struct InnerClient {
        url: Url,
        http_client: Arc<reqwest::r#async::Client>,
    }

    impl InnerClient {
        pub fn new(url: impl AsRef<str>) -> Result<Self, Error> {
            let url = Url::parse(url.as_ref())?;
            let client = reqwest::r#async::ClientBuilder::new().build()?;
            let http_client = Arc::new(client);

            Ok(InnerClient { url, http_client })
        }

        pub fn join(&self, name: impl AsRef<str>) -> Result<Self, UrlError> {
            let url = self.url.join(name.as_ref())?;
            let http_client = Arc::clone(&self.http_client);

            Ok(InnerClient { url, http_client })
        }

        pub fn get(&self) -> reqwest::r#async::RequestBuilder {
            self.http_client.get(self.url.clone())
        }

        pub fn post(&self) -> reqwest::r#async::RequestBuilder {
            self.http_client.post(self.url.clone())
        }

        pub fn put(&self) -> reqwest::r#async::RequestBuilder {
            self.http_client.put(self.url.clone())
        }

        pub fn delete(&self) -> reqwest::r#async::RequestBuilder {
            self.http_client.delete(self.url.clone())
        }

        pub fn head(&self) -> reqwest::r#async::RequestBuilder {
            self.http_client.head(self.url.clone())
        }

        pub fn duplicate(&self) -> InnerClient {
            InnerClient {
                url: self.url.clone(),
                http_client: Arc::clone(&self.http_client),
            }
        }
    }
}
