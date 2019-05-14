use crate::inner_client::InnerClient;

mod get;
use self::get::GetRequest;

pub struct Database {
    client: InnerClient,
}

impl Database {
    pub(crate) fn new(client: InnerClient) -> Self {
        Database { client }
    }

    pub fn get(&self, id: impl Into<String>) -> GetRequest {
        GetRequest::new(&self.client, id)
    }
}
