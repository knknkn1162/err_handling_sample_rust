use std::{io, fmt};
use std;
use std::error::{Error as StdError};


#[derive(Debug)]
pub enum Error {
    EmptyVec,
    Parse(std::num::ParseIntError),
    Io(io::Error),
}

pub type LibResult<T> = Result<T, Error>;

// self define ParseIntError or io:Error into Error
impl From<io::Error> for Error {
    fn from(err: io::Error)-> Self {Error::Io(err)}
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError)-> Self {Error::Parse(err)}
}

// Generation of an error is completely separate from how it is displayed.
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter)-> fmt::Result {
        match *self {
            //user-defined
            Error::EmptyVec => write!(f, "please use a vector with at least one element"),
            // simply wrapped
            Error::Parse(ref err) => err.fmt(f),
            Error::Io(ref err) => err.fmt(f),
        }
    }
}

// This is important for other errors to wrap this one.
impl StdError for Error {
    fn description(&self)-> &str {
        match *self {
            Error::EmptyVec => "empty vectors not allowed",
            Error::Parse(ref err) => err.description(),
            Error::Io(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::EmptyVec => None,
            Error::Parse(ref err) => Some(err),
            Error::Io(ref err) => Some(err),
        }
    }
}