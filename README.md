[![Build Status](https://travis-ci.org/danieleades/chesterfield.svg?branch=master)](https://travis-ci.org/danieleades/chesterfield)

# chesterfield

An ergonomic, strongly typed CouchDB client in Rust.

This library includes both synchronous and asynchronous APIs for the programmer who wants it all

### Synchronous

```rust
use chesterfield::{Error, sync::Client, GetResponse};
use serde::Deserialize;

// use your own concrete types
#[derive(Deserialize)]
struct MyConcreteStruct {}

let client = Client::new("https://localhost:5984").unwrap();
let database =  client.database("items").unwrap();
let doc_id = "some_unique_id";

match database
    .get(doc_id)
    .send()
    .map(GetResponse::<MyConcreteStruct>::into_inner) {
    Ok(my_struct) => (), // do something with struct
    Err(e) => println!("{}", e),
}

```

### Asynchronous
```rust
use chesterfield::{Error, r#async::Client, GetResponse};
use serde::Deserialize;
use tokio::prelude::Future;

#[derive(Deserialize)]
struct MyConcreteStruct {}

let client = Client::new("https://localhost:5984").unwrap();
let database =  client.database("items").unwrap();
let doc_id = "some_unique_id".to_string();

let fut = database
    .get(doc_id)
    .send()
    .map(GetResponse::into_inner)
    .map(| document: MyConcreteStruct | {
        // do something with your struct
        })
    .map_err(|e| println!("{}", e));

tokio::run(fut);

```

## Building
```bash
cargo build
```
>*"oh my gosh so easy!"*

## Running Tests
```bash
cargo test
```
>*"sham-wow!"*

## Contributing
```bash
cargo contribute
```
>*"bullseye!"*

---

Current version: 0.1.0

License: apache-2.0