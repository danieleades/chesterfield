pub mod sync {

    use crate::database::sync::Database;
    use crate::inner_client::sync::InnerClient;
    use crate::{Error, UrlError};

    /// A synchronous CouchDB client.
    ///
    /// The synchronous API is a tad easier to work with and reason about
    /// than the asynchronous one, with the downside that the operations will
    /// block the current thread until complete.
    pub struct Client {
        client: InnerClient,
    }

    impl Client {
        /// Create a new synchronous client.
        ///
        /// # Example
        /// ```
        /// use chesterfield::sync::Client;
        ///
        /// let client = Client::new("http://localhost:5984").unwrap();
        /// ```
        pub fn new(url: impl AsRef<str>) -> Result<Self, Error> {
            let client = InnerClient::new(url)?;

            Ok(Client { client })
        }

        /// Create an interface to a CouchDB database.
        ///
        /// # Example
        /// ```
        /// use chesterfield::sync::Client;
        ///
        /// let client = Client::new("http://localhost:5984").unwrap();
        ///
        /// let database = client.database("some_collection").unwrap();
        /// ```
        pub fn database(&self, name: impl AsRef<str>) -> Result<Database, UrlError> {
            let client = self.client.join(format!("{}/", name.as_ref()))?;
            Ok(Database::new(client))
        }
    }
}

pub mod r#async {

    use crate::database::r#async::Database;
    use crate::inner_client::r#async::InnerClient;
    use crate::{Error, UrlError};

    /// An asynchronous CouchDB client
    pub struct Client {
        client: InnerClient,
    }

    impl Client {
        /// Create a new asynchronous client.
        ///
        /// # Example
        /// ```
        /// use chesterfield::Client;
        ///
        /// let client = Client::new("http://localhost:5984").unwrap();
        /// ```
        pub fn new(url: impl AsRef<str>) -> Result<Self, Error> {
            let client = InnerClient::new(url)?;

            Ok(Client { client })
        }

        /// Create an interface to a CouchDB database.
        ///
        /// # Example
        /// ```
        /// use chesterfield::Client;
        ///
        /// let client = Client::new("http://localhost:5984").unwrap();
        ///
        /// let database = client.database("some_collection").unwrap();
        /// ```
        pub fn database(&self, name: impl AsRef<str>) -> Result<Database, UrlError> {
            let client = self.client.join(format!("{}/", name.as_ref()))?;
            Ok(Database::new(client))
        }
    }

}
