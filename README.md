# Rust Libraries 🦀

This repository is a collection of useful Rust libraries created by me.

## Adding libraries

To add a new library crate to the project, run the following command:

```bash
cargo new --lib <library_name>
```

## Testing the libraries

Unit tests are expected to be located inside the `src/lib.rs` file of each library crate. Integration tests can be placed in the `tests` directory at the root of each library crate.

## Using the libraries

To use a library crate (i.e. `lib-a`) in your project, add it as a dependency in your `Cargo.toml` file:

```toml
[dependencies]
lib-a = { git = "https://github.com/estanislaocrivos/rust-libraries", package = "lib-a" }
```
