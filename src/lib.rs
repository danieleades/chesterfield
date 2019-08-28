//! An ergonomic, strongly typed, asynchronous CouchDB client in Rust.
//!
//! [see the documentation](https://docs.rs/chesterfield/)
//!
//! *(note that the docs.rs documentation might be lagging behind master)*
//!
//! Couldn't find a decent, maintained CouchDB client in Rust. Also I wanted async. So i rolled my own.
//!
//! This is still in active development, in the sense that I add things when I need them, and fix bugs when they affect
//! me directly.
//!
//! **Pull requests, feedback, and other contributions very welcome!**
//! Would be thrilled to have a couple more sets of eyes and keyboards chipping away at this. climb aboard.

#![allow(unknown_lints)]
#![warn(clippy::all)]
#![warn(missing_docs)]

mod client;
mod database;
mod error;

pub use crate::client::Client;
pub use crate::database::{Database, GetRequest, InsertRequest, UpdateRequest};

pub use crate::error::ChesterfieldError as Error;
pub use reqwest::Url;
pub use reqwest::UrlError;
