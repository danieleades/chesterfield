mod delete;
mod get;
mod insert;
mod update;
pub use self::{
    delete::DeleteResponse, get::GetResponse, insert::InsertResponse, update::UpdateResponse,
};

pub mod sync {
    pub use super::{
        delete::sync::DeleteRequest, get::sync::GetRequest, insert::sync::InsertRequest,
        update::sync::UpdateRequest,
    };
    use crate::inner_client::sync::InnerClient;
    use crate::Error;
    use serde::Serialize;

    pub struct Database {
        client: InnerClient,
    }

    impl Database {
        pub(crate) fn new(client: InnerClient) -> Self {
            Database { client }
        }

        /// Check whether the database exists
        pub fn exists(&self) -> Result<bool, Error> {
            self.client
                .head()
                .send()
                .map(|response| match response.status().as_u16() {
                    200 => true,
                    404 => false,
                    _ => unreachable!(),
                })
                .map_err(Error::from)
        }

        pub fn get(&self, id: impl Into<String>) -> GetRequest {
            GetRequest::new(&self.client, id)
        }

        pub fn insert<T: Serialize>(
            &self,
            document: T,
            id: impl Into<Option<String>>,
        ) -> InsertRequest<T> {
            InsertRequest::new(&self.client, document, id)
        }

        pub fn update<T: Serialize>(
            &self,
            document: T,
            id: impl Into<String>,
            rev: impl Into<String>,
        ) -> UpdateRequest<T> {
            UpdateRequest::new(&self.client, document, id, rev)
        }

        pub fn delete(&self, id: impl Into<String>, rev: impl Into<String>) -> DeleteRequest {
            DeleteRequest::new(&self.client, id, rev)
        }
    }
}

pub mod r#async {
    pub use super::{
        delete::r#async::DeleteRequest, get::r#async::GetRequest, insert::r#async::InsertRequest,
        update::r#async::UpdateRequest,
    };
    use crate::inner_client::r#async::InnerClient;
    use crate::Error;
    use serde::Serialize;
    use tokio::prelude::Future;

    /// Interface for interacting with a specific CouchDB database within a CouchDB node.
    ///
    /// # Example
    /// ```
    /// use chesterfield::r#async::Client;
    ///
    /// let couchdb_url = "https://localhost:5984";
    /// let db = "collection";
    ///
    /// let client = Client::new(couchdb_url).unwrap();
    /// let database = client.database(db).unwrap();
    /// ```
    pub struct Database {
        client: InnerClient,
    }

    impl Database {
        pub(crate) fn new(client: InnerClient) -> Self {
            Database { client }
        }

        /// Check whether the database exists
        pub fn exists(&self) -> impl Future<Item = bool, Error = Error> {
            self.client
                .head()
                .send()
                .map(|response| match response.status().as_u16() {
                    200 => true,
                    404 => false,
                    _ => unreachable!(),
                })
                .map_err(Error::from)
        }

        /// Retrieve a document from a database.
        ///
        /// # Example
        /// ```
        /// use chesterfield::r#async::Client;
        ///
        /// let couchdb_url = "https://localhost:5984";
        /// let db = "collection";
        /// let document_id = "some-unique-id";
        ///
        /// let client = Client::new(couchdb_url).unwrap();
        /// let database = client.database(db).unwrap();
        ///
        /// let get_request = database.get(document_id);
        ///
        /// ```
        pub fn get(&self, id: impl Into<String>) -> GetRequest {
            GetRequest::new(&self.client, id)
        }

        pub fn insert<T: Serialize>(
            &self,
            document: T,
            id: impl Into<Option<String>>,
        ) -> InsertRequest<T> {
            InsertRequest::new(&self.client, document, id)
        }

        pub fn update<T: Serialize>(
            &self,
            document: T,
            id: impl Into<String>,
            rev: impl Into<String>,
        ) -> UpdateRequest<T> {
            UpdateRequest::new(&self.client, document, id, rev)
        }

        pub fn delete(&self, id: impl Into<String>, rev: impl Into<String>) -> DeleteRequest {
            DeleteRequest::new(&self.client, id, rev)
        }
    }
}
