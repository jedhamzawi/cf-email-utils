use std::{fmt, io};

#[derive(Debug)]
pub(crate) enum Error {
    Generic(String),
    IO(io::Error),
    Reqwest(reqwest::Error),
}

impl std::error::Error for Error {}

impl Error {
    pub(crate) fn new(msg: impl Into<String>) -> Self {
        Self::Generic(msg.into())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Generic(msg) => write!(f, "{msg}"),
            Error::IO(err) => write!(f, "IO Error: {err}"),
            Error::Reqwest(err) => write!(f, "Reqwest Client Error: {err}"),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::IO(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::Reqwest(err)
    }
}
