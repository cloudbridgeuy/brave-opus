# anthropic-rs

A lightweight Rust library for interacting with Anthropic's Claude, a powerful Large Language Model, through their Chat API. This library supports streaming and multimedia, making it easy to integrate Claude into your Rust applications.

## Features

- Simple and intuitive API for sending messages to Claude and receiving responses
- Support for streaming responses, allowing for real-time interaction
- Ability to send and receive multimedia content
- Lightweight and easy to integrate into existing Rust projects

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
anthropic-rs = "0.1.0"
```

## Usage

First, make sure you have an API key from Anthropic. You can set this as an environment variable named `ANTHROPIC_API_KEY`.

Here's a basic example of how to use the library to send a message to Claude and receive a response:

```rust
use anthropic::{
    apis::{
        messages::{MessageApi, MessageBody},
        Message, Role,
    },
    Anthropic,
};
use futures::stream::TryStreamExt;
use std::io::Write;

#[tokio::main]
async fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    env_logger::init();

    let key = std::env::var("ANTHROPIC_API_KEY")?;

    let client = Anthropic::new(anthropic::Auth::new(&key), "https://api.anthropic.com/v1/");

    let messages = vec![Message {
        role: Role::User,
        content: "What is the capital of the United States?".to_string(),
    }];

    let body = MessageBody::with_stream("claude-3-opus-20240229", messages, 100);

    let mut stream = client.message_delta_stream(&body)?;

    while let Ok(Some(text)) = stream.try_next().await {
        print!("{text}");
        std::io::stdout().flush()?;
    }

    Ok(())
}
```

This will send the message "What is the capital of the United States?" to Claude and stream the response back, printing it to the console as it arrives.

## API

The main entry point to the library is the `Anthropic` struct. This is created with your API key and the base URL for the API.

The `MessageApi` trait provides methods for sending messages and receiving responses:

- `message_create`: Sends a message and returns the complete response.
- `message_stream`: Sends a message and returns a stream of `MessageEvent`s, allowing you to process the response as it arrives.
- `message_delta_stream`: Similar to `message_stream`, but returns a stream of strings representing the delta between the current response and the previous one.

Messages are represented by the `Message` struct, which has a `role` (either `Role::User` or `Role::Assistant`) and `content` (the text of the message).

## Error Handling

The library defines a custom `Error` enum for errors that can occur when interacting with the API. These include network errors, serialization/deserialization errors, and errors returned by the API itself.

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue if you have any suggestions or find any bugs.

## License

This library is open-source and available under the MIT license. See the `LICENSE` file for more details.
