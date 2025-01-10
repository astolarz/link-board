use core::fmt;

pub struct Error {
    err: Box<ErrorImpl>
}

struct ErrorImpl {
    kind: Kind
}

enum Kind {
    ClientError(reqwest::Error),
    JsonParseError(serde_json::Error),
    TripParseError(TripParseErr),
}

#[derive(Debug)]
pub enum TripParseErr {
    Destination,
    Id,
    NextStop,
    ClosestStopTimeOffset,
}

impl Error {
    pub fn client_error(req_err: reqwest::Error) -> Self {
        Self {
            err: Box::new(ErrorImpl {
                kind: Kind::ClientError(req_err),
            })
        }
    }

    pub fn json_error(serde_err: serde_json::Error) -> Self {
        Self {
            err: Box::new(ErrorImpl {
                kind: Kind::JsonParseError(serde_err),
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
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::client_error(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::json_error(value)
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
            Kind::ClientError(e) => write!(f, "error retrieving data: {e}"),
            Kind::JsonParseError(e) => write!(f, "error parsing JSON: {e}"),
            Kind::TripParseError(trip_err) => write!(f, "failed to find {trip_err:?} for trip"),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("link_board::Error").field("err", &self.err.to_string()).finish()
    }
}