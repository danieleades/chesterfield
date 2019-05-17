//! An ergonomic, strongly typed CouchDB client in Rust.
//!
//! This library includes both synchronous and asynchronous APIs for the programmer who wants it all
//!
//! ## Synchronous
//!
//! ```rust
//! use chesterfield::{Error, sync::Client, GetResponse};
//! use serde::Deserialize;
//!
//! // use your own concrete types
//! #[derive(Deserialize)]
//! struct MyConcreteStruct {}
//!
//! let client = Client::new("https://localhost:5984").unwrap();
//! let database =  client.database("items").unwrap();
//! let doc_id = "some_unique_id";
//!
//! match database
//!     .get(doc_id)
//!     .send()
//!     .map(GetResponse::<MyConcreteStruct>::into_inner) {
//!     Ok(my_struct) => (), // do something with struct
//!     Err(e) => println!("{}", e),
//! }
//!
//! ```
//!
//! ## Asynchronous
//! ```rust
//! use chesterfield::{Error, r#async::Client, GetResponse};
//! use serde::Deserialize;
//! use tokio::prelude::Future;
//!
//! #[derive(Deserialize)]
//! struct MyConcreteStruct {}
//!
//! let client = Client::new("https://localhost:5984").unwrap();
//! let database =  client.database("items").unwrap();
//! let doc_id = "some_unique_id".to_string();
//!
//! let fut = database
//!     .get(doc_id)
//!     .send()
//!     .map(GetResponse::into_inner)
//!     .map(| document: MyConcreteStruct | {
//!         // do something with your struct
//!         })
//!     .map_err(|e| println!("{}", e));
//!
//! tokio::run(fut);
//!
//! ```

#![allow(unknown_lints)]
#![warn(clippy::all)]

mod client;
mod database;
mod error;
mod inner_client;

// asynchronous components
pub mod r#async {
    pub use crate::client::r#async::Client;
    pub use crate::database::r#async::{Database, GetRequest};
}

// synchronous components
pub mod sync {
    pub use crate::client::sync::Client;
    pub use crate::database::sync::{Database, GetRequest};
}

// common components
pub use crate::{
    database::{GetResponse, InsertResponse},
    error::Error,
};
pub use reqwest::Url;
pub use reqwest::UrlError;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
