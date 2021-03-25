
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Error {
    pub kind: Arc<ErrorKind>,
}

impl Error {
    pub fn new(kind: ErrorKind) -> Self {
        Error {
            kind: Arc::new(kind),
        }
    }

    pub fn impl_err(msg: String) -> Self {
        Error::new(ErrorKind::Impl(msg))
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self.kind {
            ErrorKind::IO(ref err) => Some(err),
            ErrorKind::Impl(_) => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.kind.fmt(f)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error{kind: Arc::new(ErrorKind::IO(err))}
    }
}

pub enum ErrorKind {
    IO(std::io::Error),
    Impl(String),
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::IO(err) => write!(f, "{}", err),
            ErrorKind::Impl(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::fmt::Debug for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}", self))
    }
}

/*
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("I/O error")]
    IOError(#[from] std::io::Error),

    #[error("KeyValue Impl error")]
    Impl(String),

    #[error("Unknown KeyValue error")]
    Unknown,
} 
*/