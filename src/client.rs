use crate::database::Database;
use crate::Error;
use reqwest::Url;
use std::str::FromStr;
use std::sync::Arc;
use url::ParseError;

/// An asynchronous CouchDB client
pub struct Client {
    url: Url,
    http_client: Arc<reqwest::Client>,
}

impl Client {
    /// Create a new asynchronous client.
    ///
    /// # Example
    /// ```
    /// use chesterfield::Client;
    ///
    /// let client = Client::from_url_str("http://localhost:5984").unwrap();
    /// ```
    ///
    /// # Errors
    /// This method fails if the TLS backend fails to initialise
    pub fn new(url: Url) -> Result<Self, Error> {
        let http_client = Arc::new(reqwest::ClientBuilder::new().build()?);

        Ok(Client { url, http_client })
    }

    /// Create a new asynchronous client from a URL string
    ///
    /// # Example
    /// ```
    /// use chesterfield::Client;
    ///
    /// let client = Client::from_url_str("http://localhost:5984").unwrap();
    /// ```
    ///
    /// # Errors
    /// This method fails if the TLS backend fails to initialise or if the URL string cannot be parsed
    pub fn from_url_str(url: impl AsRef<str>) -> Result<Self, Error> {
        let url = Url::parse(url.as_ref())?;
        Client::new(url)
    }

    pub(crate) fn join(&self, name: impl AsRef<str>) -> Result<Self, ParseError> {
        let url = self.url.join(&format!("{}/", name.as_ref()))?;
        let http_client = Arc::clone(&self.http_client);

        Ok(Client { url, http_client })
    }

    /// Create an interface to a CouchDB database.
    ///
    /// # Example
    /// ```
    /// use chesterfield::Client;
    ///
    /// let client = Client::from_url_str("http://localhost:5984").unwrap();
    ///
    /// let database = client.database("some_collection").unwrap();
    /// ```
    pub fn database(&self, name: impl AsRef<str>) -> Result<Database, ParseError> {
        let client = self.join(name)?;
        Ok(Database::new(client))
    }

    pub(crate) fn get(&self) -> reqwest::RequestBuilder {
        self.http_client.get(self.url.clone())
    }

    pub(crate) fn post(&self) -> reqwest::RequestBuilder {
        self.http_client.post(self.url.clone())
    }

    pub(crate) fn put(&self) -> reqwest::RequestBuilder {
        self.http_client.put(self.url.clone())
    }

    pub(crate) fn delete(&self) -> reqwest::RequestBuilder {
        self.http_client.delete(self.url.clone())
    }

    pub(crate) fn head(&self) -> reqwest::RequestBuilder {
        self.http_client.head(self.url.clone())
    }
}

impl From<&Client> for Client {
    fn from(client: &Client) -> Client {
        let url = client.url.clone();
        let http_client = Arc::clone(&client.http_client);

        Client { url, http_client }
    }
}

impl FromStr for Client {
    type Err = Error;

    fn from_str(url: &str) -> Result<Self, Self::Err> {
        Client::from_url_str(url)
    }
}
