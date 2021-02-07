use crate::css::Value;
use crate::parser::ParseError;
use crate::value::RangeError;
use std::convert::From;
use std::string::FromUtf8Error;
use std::{fmt, io};

/// Most functions in rsass that returns a Result uses this Error type.
#[derive(Debug)]
pub enum Error {
    Input(String, io::Error),
    IoError(io::Error),
    Encoding(FromUtf8Error),
    BadValue(String),
    BadArguments(String),
    BadRange(RangeError),
    ParseError(ParseError),
    S(String),
    UndefinedVariable(String),
}

impl std::error::Error for Error {}

impl Error {
    pub fn bad_value(expected: &str, actual: &Value) -> Self {
        Error::BadValue(format!(
            "Error: {} is not {}.",
            actual.format(Default::default()),
            expected,
        ))
    }

    /// Wrong kind of argument to a sass function.
    /// `expected` is a string describing what the parameter should
    /// have been, `actual` is the argument.
    pub fn badarg(expected: &str, actual: &Value) -> Error {
        Error::BadArguments(format!(
            "expected {}, got {} = {}",
            expected,
            actual.type_name(),
            actual.format(Default::default())
        ))
    }

    /// Multiple-argument variant of `badarg`.
    pub fn badargs(expected: &[&str], actual: &[&Value]) -> Error {
        // TODO Better message!
        Error::BadArguments(format!(
            "expected {:?}, got {:?}",
            expected, actual
        ))
    }

    pub fn undefined_variable(name: &str) -> Self {
        Error::UndefinedVariable(name.to_string())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::S(ref s) => write!(out, "{}", s),
            Error::Input(ref p, ref e) => {
                write!(out, "Failed to read {:?}: {}", p, e)
            }
            Error::UndefinedVariable(ref name) => {
                write!(out, "Undefined variable: \"${}\"", name)
            }
            Error::ParseError(ref err) => err.fmt(out),
            Error::BadRange(ref err) => err.fmt(out),
            Error::BadValue(ref err) => err.fmt(out),
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

impl From<ParseError> for Error {
    fn from(e: ParseError) -> Self {
        Error::ParseError(e)
    }
}
impl From<RangeError> for Error {
    fn from(e: RangeError) -> Self {
        Error::BadRange(e)
    }
}
