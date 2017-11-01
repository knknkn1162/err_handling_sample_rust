use std::error::{Error as StdError};
use std::{io, fmt};



#[derive(Debug)]
pub enum Error {
    EmptyVec,
    Parse(std::num::ParseIntError),
    Io(io::Error),
}

type LibResult<T> = Result<T, Error>;

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

use std::path::Path;
use std::fs::File;
use std::io::prelude::*; // reexports Read, Write, BufRead, Seek.

fn convert_vec(file: &str) -> LibResult<Vec<String>> {
    let path = Path::new(file);

    let mut s = String::new();
    File::open(&path)?.read_to_string(&mut s)?;

    let v = s.lines().map(|t| t.to_string()).collect::<Vec<String>>();
    v.first().ok_or(Error::EmptyVec)?;
    Ok(v)
}

fn file_sum(file: &str) -> LibResult<i32> {
    let v = convert_vec(file)?;
    let s = v.iter().map(|s| s.parse::<i32>()).sum::<Result<i32, _>>()?;
    Ok(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_vec() {
        let filename = "data/true.txt";
        let res = convert_vec(filename);

        assert_eq!(res.unwrap(),vec!["1", "34", "332", "67", "9980"]);
    }

    #[test]
    fn test_convert_vec_notfile() {
        let filename = "dummy.txt";
        let res = file_sum(filename);

        // check parse error
        assert_eq!(
            format!("{:?}", res.unwrap_err()),
            r#"Io(Error { repr: Os { code: 2, message: "No such file or directory" } })"#
        );
    }

    #[test]
    fn test_convert_emptyfile() {
        let filename = "data/empty.txt";

        let res = file_sum(filename);

        let ans = res.unwrap_err();

        assert_eq!(
            ans.description(),
            "empty vectors not allowed"
        );

        assert_eq!(
            format!("{:?}", ans),
            "EmptyVec"
        )
    }

    #[test]
    fn test_sum_true_file() {
        let filename = "data/true.txt";

        let res = file_sum(filename);

        assert_eq!(res.unwrap(), 10414);
    }

    #[test]
    fn test_sum_fail_file() {
        let filename = "data/fail.txt";
        let res = file_sum(filename);

        // check parse error
        assert_eq!(
            format!("{:?}", res.unwrap_err()),
            "Parse(ParseIntError { kind: InvalidDigit })"
        );
    }

    #[test]
    fn test_sum_notfile() {
        let filename = "dummy.txt";
        let res = file_sum(filename);

        // check parse error
        assert_eq!(
            format!("{:?}", res.unwrap_err()),
            r#"Io(Error { repr: Os { code: 2, message: "No such file or directory" } })"#
        );
    }
}