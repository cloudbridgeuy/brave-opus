use eventsource_client as es;

/// Error type returned from this library's functions
#[derive(Debug)]
pub enum Error {
    SseStreamCreation(Box<dyn std::error::Error + Send + Sync + 'static>),
    SseStreamError(Box<dyn std::error::Error + Send + Sync + 'static>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;

        match self {
            SseStreamCreation(err) => write!(f, "sse stream creation error: {err}"),
            SseStreamError(err) => write!(f, "sse stream error: {err}"),
        }
    }
}

impl From<es::Error> for Error {
    fn from(error: es::Error) -> Self {
        Error::SseStreamError(Box::new(error))
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
