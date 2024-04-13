use eventsource_client as es;

/// Error type returned from this library's functions
#[derive(Debug)]
pub enum Error {
    /// The request timed out.
    TimedOut,
    /// The stream was closed.
    StreamClosed,
    /// An invalid request parameter.
    InvalidParameter(Box<dyn std::error::Error + Send + Sync + 'static>),
    /// An error when creating the SSE stream.
    SseStreamCreation(Box<dyn std::error::Error + Send + Sync + 'static>),
    /// The HTTP response stream ended.
    Eof,
    /// The HTTP response stream ended unexpectedly.
    UnexpectedEof,
    /// Encountered a line not conforming to the SSE protocol.
    InvalidLine(String),
    /// Encountered an invalid SSE event.
    InvalidEvent,
    /// Encountered an unknown error.
    Unknown(Box<dyn std::error::Error + Send + Sync + 'static>),
    /// An Error returned by the API
    ApiError(String),
    /// An Error not related to the API
    RequestError(String),
    /// An Error occurred when serializing an object.
    SerializeError(serde_json::error::Error),
    /// An Error occurred when deserializing an object.
    DeserializeError(serde_json::error::Error),
    /// An Error occurred when deserializing an object from JSON.
    DeserializeIntoJson(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::{
            ApiError, DeserializeError, DeserializeIntoJson, Eof, InvalidEvent, InvalidLine,
            InvalidParameter, RequestError, SerializeError, SseStreamCreation, StreamClosed,
            TimedOut, UnexpectedEof, Unknown,
        };

        match self {
            TimedOut => write!(f, "timed out"),
            StreamClosed => write!(f, "stream closed"),
            SseStreamCreation(err) => write!(f, "sse stream creation error: {err}"),
            InvalidParameter(err) => write!(f, "invalid parameter: {err}"),
            Eof => write!(f, "eof"),
            UnexpectedEof => write!(f, "unexpected eof"),
            InvalidLine(line) => write!(f, "invalid line: {line}"),
            InvalidEvent => write!(f, "invalid event"),
            Unknown(err) => write!(f, "sse stream error: {err}"),
            ApiError(s) => write!(f, "API Error: {s}"),
            RequestError(s) => write!(f, "Request Error: {s}"),
            SerializeError(err) => write!(f, "serialize error: {err}"),
            DeserializeError(err) => write!(f, "deserialize error: {err}"),
            DeserializeIntoJson(err) => write!(f, "deserialize into error: {err}"),
        }
    }
}

impl From<es::Error> for Error {
    fn from(error: es::Error) -> Self {
        match error {
            es::Error::TimedOut => Self::TimedOut,
            es::Error::StreamClosed => Self::StreamClosed,
            es::Error::InvalidParameter(error) => Self::InvalidParameter(error),
            es::Error::Eof => Self::Eof,
            es::Error::UnexpectedEof => Self::UnexpectedEof,
            es::Error::InvalidLine(line) => Self::InvalidLine(line),
            es::Error::InvalidEvent => Self::InvalidEvent,
            _ => Self::Unknown(Box::new(error)),
        }
    }
}

impl std::error::Error for Error {}
