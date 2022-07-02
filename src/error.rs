use crate::parser::{ParseError, SourcePos};
use crate::sass::{ArgsError, Name};
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
    /// An illegal value for a specific parameter.
    BadArgument(Name, String),
    /// The pos here is the function declaration.
    /// This error will be wrapped in a BadCall, giving the pos of the call.
    BadArguments(ArgsError, SourcePos),
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

/// Something invalid.
///
/// Should be combined with a position to get an [Error].
#[derive(Debug)]
pub enum Invalid {
    /// An undefined variable was used at source pos.
    UndefinedVariable,
    /// Undefined function.
    UndefinedFunction,
    /// Attemt to use an undefined module.
    UndefModule(String),
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
    /// Functions may not be declared in control directives
    FunctionInControl,
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
            Invalid::UndefinedFunction => "Undefined function.".fmt(out),
            Invalid::UndefModule(name) => {
                write!(
                    out,
                    "There is no module with the namespace {:?}.",
                    name
                )
            }
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
            Invalid::AtError(msg) => msg.fmt(out),
        }
    }
}
