# anthropic-rs

[![Crates.io](https://img.shields.io/crates/v/anthropic-rs.svg)](https://crates.io/crates/anthropic-rs)
[![Docs.rs](https://docs.rs/anthropic-rs/badge.svg)](https://docs.rs/anthropic-rs)

`anthropic-rs` is a Rust library that simplifies the process of interacting with Anthropic's Claude API. It provides an easy-to-use interface for sending chat messages and receiving responses, with support for streaming responses.

## Features

- Send chat messages to the Claude API
- Receive responses from the API
- Support for streaming responses
- Configurable authentication using API keys
- Error handling and detailed error messages

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
anthropic-rs = "0.1.0"
```

## Usage

### Authentication

To use the library, you need to provide your Anthropic API key. You can create an `Auth` instance using the `Auth::new` method:

```rust
use anthropic_rs::Auth;

let auth = Auth::new("your_api_key_here");
```

Alternatively, you can load the API key from an environment variable using `Auth::from_env`:

```rust
use anthropic_rs::Auth;

let auth = Auth::from_env().expect("Failed to load API key from environment");
```

### Creating an Anthropic Instance

Once you have an `Auth` instance, you can create an `Anthropic` instance using the `Anthropic::new` method:

```rust
use anthropic_rs::{Anthropic, Auth};

let auth = Auth::new("your_api_key_here");
let api_url = "https://api.anthropic.com";
let anthropic = Anthropic::new(auth, api_url);
```

### Sending Chat Messages

To send a chat message and receive a response, you can use the `post` method:

```rust
use anthropic_rs::{Anthropic, Auth, Json};

let auth = Auth::new("your_api_key_here");
let anthropic = Anthropic::new(auth, "https://api.anthropic.com");

let body = Json::from_str(r#"{"prompt": "Hello, Claude!"}"#).unwrap();
let response = anthropic.post("/v1/chat", body).unwrap();
println!("Response: {}", response);
```

### Streaming Responses

To receive streaming responses, you can use the `stream` method:

```rust
use anthropic_rs::{Anthropic, Auth, Json};
use futures::StreamExt;

let auth = Auth::new("your_api_key_here");
let anthropic = Anthropic::new(auth, "https://api.anthropic.com");

let body = Json::from_str(r#"{"prompt": "Tell me a story.", "stream": true}"#).unwrap();
let mut stream = anthropic.stream("/v1/chat", body).unwrap();

while let Some(event) = stream.next().await {
    match event {
        Ok(event) => println!("Received event: {:?}", event),
        Err(err) => eprintln!("Error: {}", err),
    }
}
```

## Error Handling

The library defines a custom `Error` enum that represents various error conditions that can occur when interacting with the API. The `Error` type implements the `std::error::Error` trait, allowing it to be used with the `?` operator for easy error propagation.

## Examples

For more examples and detailed usage instructions, please refer to the [documentation](https://docs.rs/anthropic-rs).

## License

This library is licensed under the [MIT License](LICENSE).
