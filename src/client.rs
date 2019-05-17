pub mod sync {

    use crate::database::sync::Database;
    use crate::inner_client::sync::InnerClient;
    use crate::{Error, UrlError};

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
        /// let client = Client::new("localhost:5984").unwrap();
        /// ```
        pub fn new(url: impl AsRef<str>) -> Result<Self, Error> {
            let client = InnerClient::new(url)?;

            Ok(Client { client })
        }

        pub fn database(&self, name: impl AsRef<str>) -> Result<Database, UrlError> {
            let client = self.client.join(name)?;
            Ok(Database::new(client))
        }
    }
}

pub mod r#async {

    use crate::database::r#async::Database;
    use crate::inner_client::r#async::InnerClient;
    use crate::{Error, UrlError};

    pub struct Client {
        client: InnerClient,
    }

    impl Client {
        pub fn new(url: impl AsRef<str>) -> Result<Self, Error> {
            let client = InnerClient::new(url)?;

            Ok(Client { client })
        }

        pub fn database(&self, name: impl AsRef<str>) -> Result<Database, UrlError> {
            let client = self.client.join(name)?;
            Ok(Database::new(client))
        }
    }

}
