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
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::{ApiError, Eof, InvalidEvent, InvalidLine, InvalidParameter, RequestError, SseStreamCreation, StreamClosed, TimedOut, UnexpectedEof, Unknown};

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
