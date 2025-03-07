# Safelyx API

[![](https://github.com/safelyx/safelyx-py/workflows/Run%20Tests/badge.svg)](https://github.com/safelyx/safelyx-rust/actions?workflow=Run+Tests) [![crate](https://img.shields.io/crates/v/safelyx.svg)](https://crates.io/crates/safelyx)

> Safelyx API client

Safelyx API client for Rust.

You can find the API documentation at https://safelyx.com/safe-api.

### Some things to note:

1. It's simply making an HTTP request to the Safelyx API.

2. It's using the `reqwest` library to make the HTTP request.

3. If the request to the API fails, it will return a `Result` with an error, so you can handle it using Rust's error handling patterns.

## Usage

It has a method per API endpoint.

### Rust

```rust
use safelyx;

fn main() {
    let check_result = safelyx::check_link("https://example.com", "your-key-code")
        .expect("Failed to check link");

    println!("{}", check_result.result);  // Outputs a safety score between 0 (unsafe) and 10 (safe). -1 if there was an error, -2 if there are no checks remaining.
}
```

### Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
safelyx = "0.1"
```

## Development

Requires [Rust](https://rust-lang.org) 1.70+.

```bash
make install
make format
make test
```

## Publishing

After committing and pushing with a new version in `Cargo.toml`, just run:

```bash
make publish
```
