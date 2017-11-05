use css::Value;
use nom;
use std::{fmt, io};
use std::convert::From;
use std::path::PathBuf;
use std::string::FromUtf8Error;

/// Most functions in rsass that returns a Result uses this Error type.
#[derive(Debug)]
pub enum Error {
    Input(PathBuf, io::Error),
    IoError(io::Error),
    Encoding(FromUtf8Error),
    BadValue(String),
    BadArguments(String),
    ParseError(nom::ErrorKind),
    S(String),
}

impl Error {
    pub fn bad_value(expected: &str, actual: &Value) -> Self {
        Error::BadValue(format!("expected {}, got {} = {}",
                                expected,
                                actual.type_name(),
                                actual))
    }

    /// Wrong kind of argument to a sass function.
    /// `expected` is a string describing what the parameter should
    /// have been, `actual` is the argument.
    pub fn badarg(expected: &str, actual: &Value) -> Error {
        Error::BadArguments(format!("expected {}, got {} = {}",
                                    expected,
                                    actual.type_name(),
                                    actual))
    }

    /// Multiple-argument variant of `badarg`.
    pub fn badargs(expected: &[&str], actual: &[&Value]) -> Error {
        // TODO Better message!
        Error::BadArguments(format!("expected {:?}, got {:?}",
                                    expected,
                                    actual))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::S(ref s) => write!(out, "{}", s),
            Error::Input(ref p, ref e) => {
                write!(out, "Failed to read {:?}: {}", p, e)
            }
            // fallback
            ref x => write!(out, "{:?}", x),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IoError(e)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(e: FromUtf8Error) -> Self {
        Error::Encoding(e)
    }
}

impl From<nom::ErrorKind> for Error {
    fn from(e: nom::ErrorKind) -> Self {
        Error::ParseError(e)
    }
}
impl<'a> From<nom::Err<&'a [u8]>> for Error {
    fn from(e: nom::Err<&'a [u8]>) -> Self {
        Error::S(format!("Parse error: {}", e))
    }
}
