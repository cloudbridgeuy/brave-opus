pub mod apis;

pub use apis::*;
pub mod brave;
pub use brave::*;

pub mod error;
pub mod search;
pub mod summarize;

use log as _;

pub type Json = serde_json::Value;
pub type ApiResult<T> = std::result::Result<T, error::Error>;
