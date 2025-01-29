use core::fmt;

use log::SetLoggerError;
use tokio::io;

pub struct Error {
    err: Box<ErrorImpl>
}

struct ErrorImpl {
    kind: Kind
}

enum Kind {
    #[cfg(feature = "native")]
    ClientError(reqwest::Error),
    IoError(io::Error),
    #[cfg(feature="serde")]
    JsonParseError(serde_json::Error),
    LoggerError(SetLoggerError),
    TripParseError(TripParseErr),
    TinyJsonParseError(tinyjson::JsonParseError)
}

#[derive(Debug)]
pub enum TripParseErr {
    Id,
    NextStop,
    ClosestStopTimeOffset,
    NotInProgress,
    BeyondLastStop,
}

impl Error {
    #[cfg(feature = "native")]
    pub fn client_error(req_err: reqwest::Error) -> Self {
        Self {
            err: Box::new(ErrorImpl {
                kind: Kind::ClientError(req_err),
            })
        }
    }

    pub fn io_error(io_err: io::Error) -> Self {
        Self {
            err: Box::new(ErrorImpl {
                kind: Kind::IoError(io_err),
            })
        }
    }

    #[cfg(feature="serde")]
    pub fn json_error(serde_err: serde_json::Error) -> Self {
        Self {
            err: Box::new(ErrorImpl {
                kind: Kind::JsonParseError(serde_err),
            })
        }
    }

    pub fn logging_error(log_err: SetLoggerError) -> Self {
        Self {
            err: Box::new(ErrorImpl {
                kind: Kind::LoggerError(log_err),
            })
        }
    }

    pub fn trip_parse_error(trip_err: TripParseErr) -> Self {
        Self {
            err: Box::new(ErrorImpl {
                kind: Kind::TripParseError(trip_err),
            })
        }
    }

    pub fn tinyjson_parse_error(tinyjson_err: tinyjson::JsonParseError) -> Self {
        Self {
            err: Box::new(ErrorImpl {
                kind: Kind::TinyJsonParseError(tinyjson_err),
            })
        }
    }

    pub fn is_not_in_progress_err(&self) -> bool {
        match self.err.kind {
            Kind::TripParseError(TripParseErr::NotInProgress) => true,
            _ => false
        }
    }
}


#[cfg(feature = "native")]
impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::client_error(value)
    }
}

#[cfg(feature="serde")]
impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::json_error(value)
    }
}

impl From<SetLoggerError> for Error {
    fn from(value: SetLoggerError) -> Self {
        Error::logging_error(value)
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::io_error(value)
    }
}

impl From<tinyjson::JsonParseError> for Error {
    fn from(value: tinyjson::JsonParseError) -> Self {
        Error::tinyjson_parse_error(value)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt::Display::fmt( &*self.err, f)
    }
}

impl fmt::Display for ErrorImpl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            #[cfg(feature = "native")]
            Kind::ClientError(e) => write!(f, "error retrieving data: {e}"),
            Kind::IoError(e) => write!(f, "tokio::io error: {e}"),
            #[cfg(feature="serde")]
            Kind::JsonParseError(e) => write!(f, "error parsing JSON: {e}"),
            Kind::LoggerError(e) => write!(f, "logging error: {e}"),
            Kind::TripParseError(trip_err) => write!(f, "failed to find {trip_err:?} for trip"),
            Kind::TinyJsonParseError(e) => write!(f, "failed to parse json: {e}"),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("link_board::Error").field("err", &self.err.to_string()).finish()
    }
}