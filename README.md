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

See [ARCHITECTURE.md](ARCHITECTURE.md) for the internal architecture.

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

To remove a device from a group:

```bash
cargo run --bin mender-api-cli -- devices remove-from-group <device-id> <group-name>
```

### Library

Add to your `Cargo.toml`:

```toml
mender-api = { path = "./api" }
```

Import and use the library in your Rust project:

```rust
use mender_api::{Client, Login};

#[tokio::main]
async fn main() {
    let client = Client::new("https://mender.example.com".parse().expect("valid server URL"));
    let session = client.login("username", "password").await.expect("login failed");
    // Use the session to interact with the Mender API.
}
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
