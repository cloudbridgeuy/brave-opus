/// Error type returned from this library's functions
#[derive(Debug)]
pub enum Error {
    /// Encountered an unknown error.
    Unknown(Box<dyn std::error::Error + Send + Sync + 'static>),
    /// An Error returned by the API
    ApiError(String),
    /// An Error occurred when serializing an object.
    SerializeError(serde_json::error::Error),
    /// An Error occurred when deserializing an object.
    DeserializeError(serde_json::error::Error),
    /// An Error occurred when deserializing an object.
    DeserializeIoError(std::io::Error),
    /// An Error not related to the API
    RequestError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::{
            ApiError, DeserializeError, DeserializeIoError, RequestError, SerializeError, Unknown,
        };

        match self {
            SerializeError(err) => write!(f, "serialize error: {err}"),
            DeserializeError(err) => write!(f, "deserialize error: {err}"),
            ApiError(s) => write!(f, "API Error: {s}"),
            Unknown(err) => write!(f, "sse stream error: {err}"),
            DeserializeIoError(err) => write!(f, "deserialize into error: {err}"),
            RequestError(s) => write!(f, "Request Error: {s}"),
        }
    }
}

impl std::error::Error for Error {}
