use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Error {
    pub kind: Arc<ErrorKind>,
}

impl Error {
    pub fn unknown(msg: String) -> Self {
        Error {
            kind: Arc::new(ErrorKind::Unknown(msg)),
        }
    }

    pub fn unknown_str(msg: &str) -> Self {
        Error::unknown(msg.to_owned())
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self.kind {
            ErrorKind::IO(ref err) => Some(err),
            ErrorKind::KV(ref err) => Some(err),
            ErrorKind::MsgPackEncode(ref err) => Some(err),
            ErrorKind::MsgPackDecode(ref err) => Some(err),
            ErrorKind::Unknown(_) => None,
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

impl From<keyvalue::Error> for Error {
    fn from(err: keyvalue::Error) -> Error {
        Error{kind: Arc::new(ErrorKind::KV(err))}
    }
}

impl From<rmp_serde::encode::Error> for Error {
    fn from(err: rmp_serde::encode::Error) -> Error {
        Error{kind: Arc::new(ErrorKind::MsgPackEncode(err))}
    }
}

impl From<rmp_serde::decode::Error> for Error {
    fn from(err: rmp_serde::decode::Error) -> Error {
        Error{kind: Arc::new(ErrorKind::MsgPackDecode(err))}
    }
}

pub enum ErrorKind {
    IO(std::io::Error),
    KV(keyvalue::Error),
    MsgPackEncode(rmp_serde::encode::Error),
    MsgPackDecode(rmp_serde::decode::Error),
    Unknown(String),
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::IO(err) => write!(f, "{}", err),
            ErrorKind::KV(err) => write!(f, "{}", err),
            ErrorKind::MsgPackEncode(err) => write!(f, "{}", err),
            ErrorKind::MsgPackDecode(err) => write!(f, "{}", err),
            ErrorKind::Unknown(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::fmt::Debug for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}", self))
    }
}