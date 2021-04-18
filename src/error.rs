use crate::css::Value;
use crate::output::Format;
use crate::parser::{ParseError, SourcePos};
use crate::sass::Name;
use crate::value::RangeError;
use std::convert::From;
use std::{fmt, io};

/// Most functions in rsass that returns a Result uses this Error type.
#[derive(Debug)]
pub enum Error {
    /// An IO error encoundered on a specific path
    Input(String, io::Error),
    /// An IO error without specifying a path.
    ///
    /// This is (probably) an error writing output.
    IoError(io::Error),
    /// A bad call to a builtin function, with call- and optionally
    /// declaration position.
    BadCall(String, SourcePos, Option<SourcePos>),
    InvalidFunctionName(SourcePos),
    BadValue(String),
    BadArgument(Name, String),
    /// The pos here is the function declaration.
    /// This error will be wrapped in a BadCall, giving the pos of the call.
    BadArguments(String, SourcePos),
    /// A range error
    BadRange(RangeError),
    /// Error parsing sass data.
    ParseError(ParseError),
    UndefinedVariable(String),
    /// Fallback error type.
    ///
    /// This just contains a string with some message.
    S(String),
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
    pub fn bad_arg(
        name: Name,
        actual: &Value,
        problem: &'static str,
    ) -> Error {
        Error::BadArgument(
            name,
            format!("{} {}", actual.format(Format::introspect()), problem),
        )
    }

    pub fn undefined_variable(name: &str) -> Self {
        Error::UndefinedVariable(name.to_string())
    }

    pub fn error<T: AsRef<str>>(msg: T) -> Self {
        Error::S(format!("Error: {}.", msg.as_ref()))
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
            Error::BadArgument(ref name, ref problem) => {
                write!(out, "Error: ${}: {}.", name, problem)
            }
            Error::ParseError(ref err) => err.fmt(out),
            Error::BadCall(ref msg, ref callpos, ref declpos) => {
                msg.fmt(out)?;
                writeln!(out)?;
                if let Some(declpos) = declpos {
                    callpos.show_detail(out, '^', " invocation")?;
                    writeln!(out)?;
                    declpos.show_detail(out, '=', " declaration")?;
                    callpos.show_files(out)
                } else {
                    callpos.show(out)
                }
            }
            Error::InvalidFunctionName(ref pos) => {
                writeln!(out, "Error: Invalid function name.")?;
                pos.show(out)
            }
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
