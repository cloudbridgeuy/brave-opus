# Brave Search Engine API Wrapper

This is a lightweight Rust library that provides a convenient wrapper around the Brave Search Engine API. It allows you to easily integrate Brave search functionality into your Rust applications.

## Features

- Perform web searches using the Brave Search Engine API
- Retrieve search results, including web pages, images, videos, and more
- Customize search parameters such as query, country, language, and safe search settings
- Handle pagination of search results
- Deserialize search responses into structured Rust types

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
brave_search = "0.1.0"
```

## Usage

First, obtain a Brave Search API subscription token. You can sign up for an API key on the [Brave Search API website](https://api.search.brave.com/).

Then, create an instance of the `Brave` client with your subscription token:

```rust
use brave::{Brave, Auth};

let key = std::env::var("BRAVE_SUBSCRIPTION_TOKEN").unwrap();
let client = Brave::new(Auth::new(&key), "https://api.search.brave.com/res/v1");
```

You can now perform searches using the `search` method:

```rust
use brave::{apis::web_search::WebSearchApi, WebSearchParams};

let params = WebSearchParams::new("capital of the United States");
let response = client.search(&params);

println!("{:#?}", response);
```

The `WebSearchParams` struct allows you to customize various search parameters, such as:

- `q`: The search query (required)
- `country`: The country code for localized results
- `search_lang`: The language code for search results
- `ui_lang`: The preferred UI language
- `count`: The number of search results per page
- `offset`: The offset for pagination
- `safesearch`: The safe search filter setting
- `freshness`: The time range for search results
- `text_decorations`: Whether to include text decoration markers
- `spellcheck`: Whether to enable spell checking
- `result_filter`: A comma-separated list of result types to include
- `goggles_id`: The ID of a custom re-ranking Goggle
- `units`: The preferred measurement units (metric or imperial)
- `extra_snippets`: Whether to include additional result snippets

The `search` method returns a `WebSearchApiResponse` struct containing the search results, which can be accessed using the various fields such as `web`, `images`, `videos`, etc.

For more detailed information on the available search parameters and response fields, please refer to the [Brave Search API documentation](https://api.search.brave.com/app/documentation/web-search/get-started).

## Error Handling

The library defines a custom `Error` enum for handling different types of errors that may occur during API requests. You can use pattern matching to handle specific error cases:

```rust
match client.search(&params) {
    Ok(response) => {
        // Handle successful response
    }
    Err(brave::Error::ApiError(msg)) => {
        // Handle API error
    }
    Err(brave::Error::RequestError(msg)) => {
        // Handle request error
    }
    // Handle other error cases
}
```

## Contributing

Contributions to this library are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request on the [GitHub repository](https://github.com/your-username/brave-search-rs).

## License

This library is licensed under the [MIT License](LICENSE).
