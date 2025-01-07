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
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt::Display::fmt( &*self.err, f)
    }
}

impl fmt::Display for ErrorImpl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            Kind::ClientError(e) => write!(f, "error retrieving data: {e}"),
            Kind::JsonParseError(e) => write!(f, "error parsing JSON: {e}"),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("link_board::Error").field("err", &self.err.to_string()).finish()
    }
}