use crate::css::Value;
#[cfg(feature = "clap")]
use clap;
use nom;
use std::convert::From;
use std::path::PathBuf;
use std::string::FromUtf8Error;
use std::{fmt, io};

/// Most functions in rsass that returns a Result uses this Error type.
#[derive(Debug)]
pub enum Error {
    Input(PathBuf, io::Error),
    IoError(io::Error),
    Encoding(FromUtf8Error),
    BadValue(String),
    BadArguments(String),
    ParseError {
        file: String,
        pos: ErrPos,
        kind: Option<nom::error::ErrorKind>,
    },
    S(String),
    UndefinedVariable(String),
}

impl std::error::Error for Error {}

impl Error {
    pub fn bad_value(expected: &str, actual: &Value) -> Self {
        Error::BadValue(format!(
            "expected {}, got {} = {}",
            expected,
            actual.type_name(),
            actual
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
            actual
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
            Error::ParseError {
                ref file,
                ref pos,
                ref kind,
            } => write!(
                out,
                "{}:{}: Parse error, expected {}",
                file,
                pos,
                kind.as_ref()
                    .map(|k| k.description())
                    .unwrap_or("something else"),
            ),
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

impl<'a> From<nom::Err<(&[u8], nom::error::ErrorKind)>> for Error {
    fn from(e: nom::Err<(&[u8], nom::error::ErrorKind)>) -> Self {
        Error::S(format!("Parse error: {:?}", e))
    }
}

#[cfg(feature = "clap")]
impl<'a> From<clap::Error> for Error {
    fn from(e: clap::Error) -> Self {
        Error::S(format!("{}", e))
    }
}

/// Position data for a parse error.
///
/// To be usefull for reporting the error to an end user, this
/// contains the position in two forms, both the byte index (an
/// index of the parsed byte slice) and the line number and character
/// position in that line.
#[derive(Debug)]
pub struct ErrPos {
    pub index: usize,
    pub line: usize,
    pub pos: usize,
}

impl ErrPos {
    pub fn pos_of(index: usize, buffer: &[u8]) -> Self {
        let before = &buffer[0..index];
        ErrPos {
            index,
            line: 1 + bytecount::count(before, b'\n'),
            pos: bytecount::num_chars(
                before.rsplit(|c| *c == b'\n').next().unwrap(),
            ),
        }
    }
}
impl fmt::Display for ErrPos {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        self.line.fmt(out)?;
        out.write_str(":")?;
        self.pos.fmt(out)
    }
}
