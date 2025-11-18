# mender-api

`mender-api` is a Rust library and CLI application for interacting with the Mender API for device management and
software deployments.

## Features

- API client for Mender endpoints
- Management of devices, groups, releases, deployments, and tags
- Authentication and session handling
- CLI for common operations
- Configuration via file and arguments

## Project Structure

- `api/` — Rust library for the Mender API
- `cfg/` — Configuration module
- `cli/` — Command-line application

## Installation

```bash
git clone https://github.com/PaulmannLighting/mender-api.git
cd mender-api
cargo build --release
```

## Usage

### CLI

```bash
cargo run --bin mender-api-cli -- <commands> [options]
```

### Library

Add to your `Cargo.toml`:

```toml
mender-api = { path = "./api" }
```

Import and use the library in your Rust project:

```rust
use mender_api::client::Client;
```

## Configuration

Configuration files can be managed in the `cfg/` module. See examples in `cfg/src/config_file.rs`.

## Tests

Run tests with:

```bash
cargo test
```

## License

See `LICENSE` for license information.

## Contributing

Pull requests and issues are welcome!