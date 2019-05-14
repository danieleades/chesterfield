use crate::database::Database;
use crate::error::{Error, UrlError};
use crate::inner_client::InnerClient;

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
