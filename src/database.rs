mod delete;
mod get;
mod insert;
mod update;
//mod replication;

pub use self::{
    delete::DeleteRequest, get::GetRequest, insert::InsertRequest, update::UpdateRequest,
};
use crate::{inner_client::InnerClient, Error};
use futures::compat::Future01CompatExt;
use serde::Serialize;

/// Interface for interacting with a specific CouchDB database within a CouchDB node.
///
/// # Example
/// ```
/// use chesterfield::Client;
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

    /// Create the database, if it doesn't exist.
    ///
    /// Creating the database object itself is lazy- no check is performed
    /// that the endpoint exists. Call this method if you need to create the endpoint
    /// (or if you're not sure).
    ///
    /// # Example
    /// ```
    /// # #![feature(async_await)]
    /// use chesterfield::Client;
    /// use futures::future::{FutureExt, TryFutureExt};
    /// use tokio;
    /// # use couchdb_container::CouchDbContainer;
    /// 
    /// let future03 = async {
    /// # {
    ///     // Create the CouchDB client
    ///     let client = Client::new("http://localhost:5984").unwrap();
    /// # }
    /// #
    /// # let couchdb = CouchDbContainer::new().await;
    /// # let url = format!("http://localhost:{}", couchdb.port());
    /// # let client = Client::new(url).unwrap();
    /// 
    ///     // create a client for a specific database
    ///     let database = client.database("items").unwrap();
    /// 
    ///     // create the database in the remote CouchDB instance
    ///     database.create().await.expect("unable to create database!");
    /// #
    /// # couchdb.delete().await;
    /// };
    /// 
    /// // Currently, we must convert 0.3 future to 0.1 future to run on tokio executor
    /// let future01 = future03.unit_error().boxed().compat();
    /// 
    /// tokio::run(future01);
    /// ```
    pub async fn create(&self) -> Result<(), Error> {
        self.client
            .put()
            .send()
            .compat()
            .await
            .map(|_| ())
            .map_err(Error::from)
    }

    /// Check whether the database exists
    pub async fn exists(&self) -> Result<bool, Error> {
        self.client
            .head()
            .send()
            .compat()
            .await
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
    /// ```rust,ignore
    /// use chesterfield::Client;
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

    /// Insert pretty much anything into the database.
    ///
    /// Provided that is, that it implements [Serialize](serde::Serialize).
    ///
    /// You can optionally provide an id. If you don't, CouchDB will assign one for you
    /// (but you might not like it). The response will contain the ID and the revision.
    ///
    /// # Example
    /// ```rust,ignore
    /// # use chesterfield::Client;
    /// # use serde::Serialize;
    /// # use tokio::prelude::Future;
    ///
    /// #[derive(Serialize)]
    /// struct MyCoolStruct {
    ///     field1: String,
    ///     field2: u32,
    /// }
    ///
    /// let doc = MyCoolStruct {
    ///     field1: String::from("some string"),
    ///     field2: 42,
    /// };
    ///
    /// # {
    /// #     let client = Client::new("http://localhost:5984").unwrap();
    /// # }
    ///
    /// # use chesterfield::CouchDbContainer;
    /// # let couchdb = CouchDbContainer::default();
    /// # let url = format!("http://localhost:{}", couchdb.port());
    /// # let client = Client::new(url).unwrap();
    ///     
    /// # // Create the database client
    /// # let database = client.database("items").unwrap();
    /// #     
    /// # tokio::run(
    /// #     // ensure the database exists in the remote
    /// #     database.create().map_err(|e| panic!("{}", e)),
    /// # );
    ///     
    /// tokio::run(
    ///     database
    ///         // insert document into database
    ///         .insert(&doc, None)
    ///         .send()
    ///         // do something with the response
    ///         .map(|response| assert!(response.ok))
    ///         // handle any errors
    ///         .map_err(|e| panic!("{}", e)),
    /// );
    ///     
    /// # couchdb.delete();
    /// ```
    pub fn insert<'a, T: Serialize>(
        &self,
        document: &'a T,
        id: impl Into<Option<String>>,
    ) -> InsertRequest<'a, T> {
        InsertRequest::new(&self.client, document, id)
    }

    /// Update an existing document in the database.
    ///
    /// You'll need to know the id and current revision of the document
    /// (you can get the current revision with a 'get' request).
    ///
    /// you can *patch* an existing document by providing a subset of the
    /// document fields. Easiest way to do this is probably with a [Value](serde_json::Value).
    ///
    /// # Example
    /// ```rust,ignore
    /// # use chesterfield::Client;
    /// # use serde::Serialize;
    /// # use tokio::prelude::Future;
    ///
    /// # #[derive(Serialize)]
    /// # struct MyCoolStruct {
    /// #     field1: String,
    /// #     field2: u32,
    /// # }
    ///
    /// let doc = MyCoolStruct {
    ///     field1: String::from("some string"),
    ///     field2: 42,
    /// };
    ///
    /// # {
    /// #     let client = Client::new("http://localhost:5984").unwrap();
    /// # }
    ///
    /// # use chesterfield::CouchDbContainer;
    /// # let couchdb = CouchDbContainer::default();
    /// # let url = format!("http://localhost:{}", couchdb.port());
    /// # let client = Client::new(url).unwrap();
    ///     
    /// # // Create the database client
    /// # let database = client.database("items").unwrap();
    /// #     
    /// # tokio::run(
    /// #     // ensure the database exists in the remote
    /// #     database.create().map_err(|e| panic!("{}", e)),
    /// # );
    ///     
    /// tokio::run(
    ///     database
    ///         // insert document into database
    ///         .insert(&doc, None)
    ///         .send()
    ///         // do something with the response
    ///         .map(|response| assert!(response.ok))
    ///         // handle any errors
    ///         .map_err(|e| panic!("{}", e)),
    /// );
    ///     
    /// # couchdb.delete();
    /// ```
    pub fn update<'a, T: Serialize>(
        &self,
        document: &'a T,
        id: impl Into<String>,
        rev: impl Into<String>,
    ) -> UpdateRequest<'a, T> {
        UpdateRequest::new(&self.client, document, id, rev)
    }

    pub fn delete(&self, id: impl Into<String>, rev: impl Into<String>) -> DeleteRequest {
        DeleteRequest::new(&self.client, id, rev)
    }

    pub fn replicate_to<S: Into<String>>(url: S) -> Result<Replication, Error> {
        unimplemented!()
    }

    pub fn replicate_from<S: Into<String>>(url: S) -> Result<Replication, Error> {
        unimplemented!()
    }

    pub fn replicate_sync<S: Into<String>>(url: S) -> Result<Replication, Error> {
        unimplemented!()
    }
}

pub struct Replication {}
