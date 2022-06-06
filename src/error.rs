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
    /// Some kind of illegal value.
    BadValue(String),
    /// An illegal value for a specific parameter.
    BadArgument(Name, String),
    /// The pos here is the function declaration.
    /// This error will be wrapped in a BadCall, giving the pos of the call.
    BadArguments(String, SourcePos),
    /// Tried to import file at pos while already importing it at pos.
    ImportLoop(SourcePos, SourcePos),
    /// A range error
    BadRange(RangeError),
    /// Error parsing sass data.
    ParseError(ParseError),
    /// Something bad at a specific position.
    Invalid(Invalid, SourcePos),
    /// Fallback error type.
    ///
    /// This just contains a string with some message.
    S(String),
}

impl std::error::Error for Error {}

impl Error {
    /// A bad value with an "(actual) is not (expected)" message.
    pub fn bad_value(expected: &str, actual: &Value) -> Self {
        Error::BadValue(format!(
            "Error: {} is not {}.",
            actual.format(Format::introspect()),
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

    /// A generic error message.
    pub fn error<T: AsRef<str>>(msg: T) -> Self {
        Error::S(format!("Error: {}", msg.as_ref()))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::S(ref s) => write!(out, "{}", s),
            Error::Input(ref p, ref e) => {
                write!(out, "Failed to read {:?}: {}", p, e)
            }
            Error::BadArgument(ref name, ref problem) => {
                write!(out, "Error: ${}: {}", name, problem)
            }
            Error::ParseError(ref err) => err.fmt(out),
            Error::ImportLoop(ref pos, ref oldpos) => {
                writeln!(out, "Error: This file is already being loaded.")?;
                pos.show_detail(out, '^', " new load")?;
                writeln!(out)?;
                oldpos.show_detail(out, '=', " original load")?;
                pos.show_files(out)
            }
            Error::BadCall(ref msg, ref callpos, ref declpos) => {
                msg.fmt(out)?;
                writeln!(out)?;
                if let Some(declpos) = declpos {
                    if callpos.same_file_as(declpos) {
                        show_in_file(
                            out,
                            callpos,
                            " invocation",
                            declpos,
                            " declaration",
                        )?;
                    } else {
                        callpos.show_detail(out, '^', " invocation")?;
                        writeln!(out)?;
                        declpos.show_detail(out, '=', " declaration")?;
                    }
                    callpos.show_files(out)
                } else {
                    callpos.show(out)
                }
            }
            Error::Invalid(ref what, ref pos) => {
                writeln!(out, "Error: {}", what)?;
                pos.show(out)
            }
            Error::BadRange(ref err) => err.fmt(out),
            Error::BadValue(ref err) => err.fmt(out),
            // fallback
            ref x => write!(out, "{:?}", x),
        }
    }
}

/// Output multiple positions from the same file.
fn show_in_file(
    out: &mut fmt::Formatter,
    one: &SourcePos,
    one_name: &str,
    other: &SourcePos,
    other_name: &str,
) -> fmt::Result {
    if one < other {
        show_in_file2(out, one, one_name, other, other_name)
    } else {
        show_in_file2(out, other, other_name, one, one_name)
    }
}

fn show_in_file2(
    out: &mut fmt::Formatter,
    first: &SourcePos,
    first_name: &str,
    second: &SourcePos,
    second_name: &str,
) -> fmt::Result {
    write!(out, "    ,")?;
    first.show_inner(out, 3, '=', first_name)?;
    write!(out, "... |")?;
    second.show_inner(out, 3, '^', second_name)?;
    write!(out, "    '")?;
    Ok(())
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

/// Something invalid.
///
/// Should be combined with a position to get an [Error].
#[derive(Debug)]
#[allow(unused)] // fixme
pub enum Invalid {
    /// An undefined variable was used at source pos.
    UndefinedVariable,
    /// Attemt to use an undefined module.
    UndefModule(String),
    /// Tried to declare a function with a forbidden name.
    FunctionName,
    /// This at rule is not allowed here.
    AtRule,
    /// An `@error` reached.
    AtError(String),
}
impl Invalid {
    /// Combine this with a position to get a proper error.
    pub fn at(self, pos: SourcePos) -> Error {
        Error::Invalid(self, pos)
    }
}

impl fmt::Display for Invalid {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Invalid::UndefinedVariable => "Undefined variable.".fmt(out),
            Invalid::UndefModule(name) => {
                write!(
                    out,
                    "There is no module with the namespace {:?}.",
                    name
                )
            }
            Invalid::FunctionName => "Invalid function name.".fmt(out),
            Invalid::AtRule => "This at-rule is not allowed here.".fmt(out),
            Invalid::AtError(msg) => msg.fmt(out),
        }
    }
}
