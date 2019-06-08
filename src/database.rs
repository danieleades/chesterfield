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
    use crate::{inner_client::sync::InnerClient, Error};
    use serde::Serialize;

    /// A client to a database instance within a CouchDB node.
    ///
    /// A Database is created from a parent Client object.
    ///
    /// # Example
    /// ```
    /// use chesterfield::sync::Client;
    ///
    /// let client = Client::new("http://localhost:5984").unwrap();
    ///
    /// let items = client.database("items").unwrap();
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
        /// use chesterfield::sync::Client;
        ///
        /// # {
        /// let client = Client::new("http://localhost:5984").unwrap();
        /// # }
        /// #
        /// # use chesterfield::CouchDbContainer;
        /// # let couchdb = CouchDbContainer::default();
        /// # let url = format!("http://localhost:{}", couchdb.port());
        /// # let client = Client::new(url).unwrap();
        /// # use std::{thread, time};
        /// # thread::sleep(time::Duration::from_millis(10000));
        ///
        /// let database = client.database("items").unwrap();
        ///
        /// database.create().unwrap();
        ///
        /// # couchdb.delete();
        /// ```
        pub fn create(&self) -> Result<(), Error> {
            self.client.put().send().map(|_| ()).map_err(Error::from)
        }

        /// Check whether the database exists
        ///
        /// # Example
        /// ```
        /// use chesterfield::sync::Client;
        ///
        /// # {
        /// let client = Client::new("http://localhost:5984").unwrap();
        /// # }
        /// #
        /// # use chesterfield::CouchDbContainer;
        /// # let couchdb = CouchDbContainer::default();
        /// # let url = format!("http://localhost:{}", couchdb.port());
        /// # let client = Client::new(url).unwrap();
        /// # use std::{thread, time};
        /// # thread::sleep(time::Duration::from_millis(10000));
        ///
        /// let database = client.database("items").unwrap();
        ///
        /// database.create().unwrap();
        ///
        /// assert!(database.exists().unwrap());
        ///
        /// # couchdb.delete();
        /// ```
        pub fn exists(&self) -> Result<bool, Error> {
            let request = self.client.head();
            println!("{:#?}", request);

            request
                .send()
                .map(|response| match response.status().as_u16() {
                    200 => true,
                    404 => false,
                    _ => unreachable!(),
                })
                .map_err(Error::from)
        }

        /// Retrieve a document from the database by ID.
        ///
        /// The returned GetRequest is lazy, and won't do a goddamn thing until
        /// you 'send' it. see [GetReqest](GetRequest) for details.
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
        /// ```
        /// use chesterfield::sync::Client;
        /// use serde::Serialize;
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
        /// let client = Client::new("http://localhost:5984").unwrap();
        /// # }
        /// #
        /// # use chesterfield::CouchDbContainer;
        /// # let couchdb = CouchDbContainer::default();
        /// # let url = format!("http://localhost:{}", couchdb.port());
        /// # let client = Client::new(url).unwrap();
        /// # use std::{thread, time};
        /// # thread::sleep(time::Duration::from_millis(10000));
        ///
        /// let database = client.database("items").unwrap();
        ///
        /// database.create().unwrap();
        ///
        /// let response = database.insert(&doc, None)
        ///     .send()
        ///     .unwrap();
        ///
        /// assert!(response.ok);
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

        /// Update an existing document.
        ///
        /// You'll need to know the ID and current revision of the document you wish to update
        ///
        /// # Example
        /// ```
        /// use chesterfield::sync::Client;
        /// use serde::Serialize;
        ///
        /// #[derive(Serialize, Clone)]
        /// struct MyCoolStruct {
        ///     field1: String,
        ///     field2: u32,
        /// }
        ///
        /// let mut doc = MyCoolStruct {
        ///     field1: String::from("some string"),
        ///     field2: 42,
        /// };
        ///
        /// # {
        /// let client = Client::new("http://localhost:5984").unwrap();
        /// # }
        /// #
        /// # use chesterfield::CouchDbContainer;
        /// # let couchdb = CouchDbContainer::default();
        /// # let url = format!("http://localhost:{}", couchdb.port());
        /// # let client = Client::new(url).unwrap();
        /// # use std::{thread, time};
        /// # thread::sleep(time::Duration::from_millis(10000));
        ///
        /// let database = client.database("items").unwrap();
        /// # database.create().unwrap();
        ///
        /// let response = database.insert(&doc, None)
        ///     .send()
        ///     .unwrap();
        ///
        /// assert!(response.ok);
        ///
        /// let id = response.id;
        /// let rev = response.rev;
        ///
        /// // modify the document
        /// doc.field2 = 100;
        ///
        /// // update it
        /// database.update(&doc, id, rev);
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

        /// Delete an existing document.
        ///
        /// You'll need to know the ID and current revision of the document you wish to delete
        ///
        /// # Example
        /// ```
        /// use chesterfield::sync::Client;
        /// use serde::{Serialize, Deserialize};
        ///
        /// #[derive(Serialize, Deserialize, Clone, Debug)]
        /// struct MyCoolStruct {
        ///     field1: String,
        ///     field2: u32,
        /// }
        ///
        /// let mut doc = MyCoolStruct {
        ///     field1: String::from("some string"),
        ///     field2: 42,
        /// };
        ///
        /// # {
        /// let client = Client::new("http://localhost:5984").unwrap();
        /// # }
        /// #
        /// # use chesterfield::CouchDbContainer;
        /// # let couchdb = CouchDbContainer::default();
        /// # let url = format!("http://localhost:{}", couchdb.port());
        /// # let client = Client::new(url).unwrap();
        /// # use std::{thread, time};
        /// # thread::sleep(time::Duration::from_millis(10000));
        ///
        /// let database = client.database("items").unwrap();
        /// # database.create().unwrap();
        ///
        /// let response = database.insert(&doc, None)
        ///     .send()
        ///     .unwrap();
        ///
        /// assert!(response.ok);
        ///
        /// let id = response.id;
        /// let rev = response.rev;
        ///
        /// // Check that the document exists
        /// assert!(
        ///     database.get(id.clone()).send::<MyCoolStruct>().is_ok()
        /// );
        ///
        /// // delete it
        /// database.delete(id.clone(), rev).send().unwrap();
        ///
        /// // Check that it's been deleted
        /// assert!(
        ///     database.get(id.clone()).send::<MyCoolStruct>().is_err()
        /// );
        ///
        /// # couchdb.delete();
        /// ```
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
    use crate::{inner_client::r#async::InnerClient, Error};
    use serde::Serialize;
    use tokio::prelude::Future;

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
        /// use chesterfield::Client;
        /// use tokio::prelude::Future;
        ///
        /// # {
        /// let client = Client::new("http://localhost:5984").unwrap();
        /// # }
        /// #
        /// # use chesterfield::CouchDbContainer;
        /// # let couchdb = CouchDbContainer::default();
        /// # let url = format!("http://localhost:{}", couchdb.port());
        /// # let client = Client::new(url).unwrap();
        ///
        /// let database = client.database("items").unwrap();
        ///
        /// tokio::run(
        ///     database.create()
        ///     .map_err(|e| {
        ///         # panic!();
        ///         println!("{}", e);
        ///     })
        /// );
        ///
        /// # couchdb.delete();
        /// ```
        pub fn create(&self) -> impl Future<Item = (), Error = Error> {
            self.client.put().send().map(|_| ()).map_err(Error::from)
        }

        /// Check whether the database exists
        pub fn exists(&self) -> impl Future<Item = bool, Error = Error> {
            let request = self.client.head();
            println!("{:#?}", request);

            request
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
        /// ```
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
    }
}
