//! > The code found in this library has been adapted from the one used to create
//! > the [OpenAI API library](https://github.com/openai-rs/openai-api).
//!
//! Pull parser for [CommonMark](https://commonmark.org). This crate provides a
//! [Parser](struct.Parser.html) struct which is an iterator over
//! [Event](enum.Event.html)s. This iterator can be used dierectly, or to
//! output HTML using the [HTML module](html/index.html).
//!
//! By default, only `CommonMark` features are enabled. To use extensions like
//! tables, footnotes or task lists, enable them by setting the corresponding
//! flags in the [Options](struct.Options.html) struct.

pub mod apis;
use std::fmt::{self, Display, Formatter};

pub use apis::*;
pub mod anthropic;
pub use anthropic::*;
mod error;
mod requests;

use log as _;

pub type Json = serde_json::Value;
pub type ApiResult<T> = Result<T, Error>;



#[derive(Debug)]
pub enum Error {
    /// An Error returned by the API
    ApiError(String),
    /// An Error not related to the API
    RequestError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::ApiError(s) => write!(f, "API Error: {s}"),
            Self::RequestError(s) => write!(f, "Request Error: {s}"),
        }
    }
}
