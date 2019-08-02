[![Build Status](https://travis-ci.org/danieleades/chesterfield.svg?branch=master)](https://travis-ci.org/danieleades/chesterfield)
[![Latest Docs](https://docs.rs/chesterfield/badge.svg)](https://docs.rs/chesterfield/)

# chesterfield

**This project is in a holding pattern pending the tokio and reqwest libraries moving to futures 0.3**

An ergonomic, strongly typed CouchDB client in Rust.

This library includes both synchronous and asynchronous APIs for the programmer who wants to have it all.

[see the documentation](https://docs.rs/chesterfield/0.0.1/chesterfield/)
*(note that the docs.rs documentation might be lagging behind master)*

Couldn't find a decent, maintained CouchDB client in Rust. Also I wanted async. So i rolled my own.

This is still in active development, in the sense that I add things when I need them, and fix bugs when they affect
me directly.

Would be thrilled to have a couple more sets of eyes and keyboards chipping away at this. climb aboard.

## Building
```bash
cargo build
```
>*"oh my gosh so easy!"*

## Running Tests
```bash
cargo test --features container
```
>*"sham-wow!"*

## Viewing Documentation
```bash
cargo doc --open
```
>*"kersplash!"*

## Contributing
```bash
cargo contribute
```
>*"bullseye!"*

---

Current version: 0.0.2

License: Apache-2.0
