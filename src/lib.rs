//! An ergonomic, strongly typed CouchDB client in Rust.
//!
//! This library includes both synchronous and asynchronous APIs for the programmer who wants to have it all.
//! 
//! [see the documentation](https://docs.rs/chesterfield/0.0.1/chesterfield/)
//! *(note that the docs.rs documentation might be lagging behind master)*
//!
//! Couldn't find a decent, maintained CouchDB client in Rust. Also I wanted async. So i rolled my own.
//!
//! This is still in active development, in the sense that I add things when I need them, and fix bugs when they affect
//! me directly.
//!
//! Would be thrilled to have a couple more sets of eyes and keyboards chipping away at this. climb aboard.

#![allow(unknown_lints)]
#![warn(clippy::all)]
#![warn(missing_docs)]

mod client;
mod database;
mod error;
mod inner_client;

/// the async module contains all the types which are specific
/// to the *asynchronous* (non-blocking) API.
pub use crate::client::r#async::Client;
pub use crate::database::r#async::{Database, GetRequest, InsertRequest, UpdateRequest};

/// The sync module contains all the types which are specific
/// to the *synchronous* (blocking) API.
pub mod sync {
    pub use crate::client::sync::Client;
    pub use crate::database::sync::{Database, GetRequest};
}

// common objects
pub use crate::{
    database::{GetResponse, InsertResponse},
    error::ChesterfieldError as Error,
};
pub use reqwest::Url;
pub use reqwest::UrlError;

#[cfg(feature = "container")]
mod container;
#[cfg(feature = "container")]
pub use container::CouchDbContainer;
