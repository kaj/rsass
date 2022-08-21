use crate::input::LoadError;
use crate::parser::{ParseError, SourcePos};
use crate::value::RangeError;
use crate::ScopeError;
use std::convert::From;
use std::{fmt, io};

/// Many functions in rsass that returns a Result uses this Error type.
///
/// Other errors in rsass, such as [`CallError`][crate::sass::CallError] or
/// [`Invalid`] can be converted to this Error type, often with
/// a method providing some context rather than with just an `Into`
/// implementation.
/// E.g. [`CallError::called_from`][crate::sass::CallError::called_from]
/// also takes a [`SourcePos`] and a function name, defining the call
/// that went wrong.
pub enum Error {
    /// Failed to load a file.
    Input(LoadError),
    /// An IO error without specifying a path.
    ///
    /// This is (probably) an error writing output.
    IoError(io::Error),
    /// A failed function call, with call- and optionally declaration
    /// position.
    BadCall(String, SourcePos, Option<SourcePos>),
    /// Tried to import file at pos while already importing it at pos.
    ///
    /// The bool is true for a used module and false for an import.
    ImportLoop(bool, SourcePos, Option<SourcePos>),
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
    /// A generic error message.
    pub fn error<T: Into<String>>(msg: T) -> Self {
        Error::S(msg.into())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(out, "Error: {:?}", self)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::S(ref s) => write!(out, "{}", s),
            Error::Input(ref load) => load.fmt(out),
            Error::ParseError(ref err) => fmt::Display::fmt(err, out),
            Error::ImportLoop(ref module, ref pos, ref oldpos) => {
                if *module {
                    writeln!(
                        out,
                        "Module loop: this module is already being loaded."
                    )?;
                } else {
                    writeln!(out, "This file is already being loaded.")?;
                }
                if let Some(oldpos) = oldpos {
                    pos.show_detail(out, '^', " new load")?;
                    writeln!(out)?;
                    oldpos.show_detail(out, '=', " original load")?;
                    pos.show_files(out)
                } else {
                    pos.show(out)
                }
            }
            Error::BadCall(ref msg, ref callpos, ref declpos) => {
                writeln!(out, "{}", msg)?;
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
                writeln!(out, "{}", what)?;
                pos.show(out)
            }
            Error::BadRange(ref err) => err.fmt(out),
            Error::IoError(ref err) => err.fmt(out),
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
    let ellipsis = first.line_no() + 1 < second.line_no();
    let lnw = second.line_no().to_string().len();
    let lnw = if ellipsis { std::cmp::max(3, lnw) } else { lnw };
    writeln!(out, "{0:lnw$} ,", "", lnw = lnw)?;
    first.show_inner(out, lnw, '=', first_name)?;
    if ellipsis {
        writeln!(out, "... |")?;
    }
    second.show_inner(out, lnw, '^', second_name)?;
    write!(out, "{0:lnw$} '", "", lnw = lnw)?;
    Ok(())
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IoError(e)
    }
}
impl From<fmt::Error> for Error {
    fn from(e: fmt::Error) -> Self {
        Error::IoError(io::Error::new(io::ErrorKind::Other, e))
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

impl From<LoadError> for Error {
    fn from(err: LoadError) -> Self {
        Error::Input(err)
    }
}

/// Something invalid.
///
/// Should be combined with a position to get an [Error].
#[derive(Debug)]
#[non_exhaustive]
pub enum Invalid {
    /// Tried to declare a function with a forbidden name.
    FunctionName,
    /// This at rule is not allowed here.
    AtRule,
    /// Mixins may not contain mixin declarations.
    MixinInMixin,
    /// Mixins may not be declared in control directives.
    MixinInControl,
    /// Mixins may not contain function declarations.
    FunctionInMixin,
    /// Functions may not be declared in control directives.
    FunctionInControl,
    /// Duplicate argument.
    DuplicateArgument,
    /// Positional arguments must come before keyword arguments.
    PositionalArgAfterNamed,
    /// Some invalid scope operation.
    InScope(ScopeError),
    /// An `@error` reached.
    AtError(String),
}

impl Invalid {
    /// Combine this with a position to get an [`Error`].
    pub fn at(self, pos: SourcePos) -> Error {
        Error::Invalid(self, pos)
    }
}

impl std::error::Error for Invalid {}

impl fmt::Display for Invalid {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Invalid::FunctionName => "Invalid function name.".fmt(out),
            Invalid::AtRule => "This at-rule is not allowed here.".fmt(out),
            Invalid::MixinInMixin => {
                "Mixins may not contain mixin declarations.".fmt(out)
            }
            Invalid::MixinInControl => {
                "Mixins may not be declared in control directives.".fmt(out)
            }
            Invalid::FunctionInMixin => {
                "Mixins may not contain function declarations.".fmt(out)
            }
            Invalid::FunctionInControl => {
                "Functions may not be declared in control directives."
                    .fmt(out)
            }
            Invalid::DuplicateArgument => "Duplicate argument.".fmt(out),
            Invalid::PositionalArgAfterNamed => {
                "Positional arguments must come before keyword arguments."
                    .fmt(out)
            }
            Invalid::InScope(err) => err.fmt(out),
            Invalid::AtError(msg) => msg.fmt(out),
        }
    }
}
