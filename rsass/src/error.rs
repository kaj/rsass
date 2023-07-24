use crate::css::InvalidCss;
use crate::input::{LoadError, SourcePos};
use crate::parser::ParseError;
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
    ///
    /// Note: This variant should probably be removed in the future.
    S(String),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(out, "Error: {self:?}")
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::S(ref s) => write!(out, "{s}"),
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
                    SourcePos::show_two(
                        out,
                        pos,
                        "new load",
                        oldpos,
                        "original load",
                    )
                } else {
                    pos.show(out)
                }
            }
            Error::BadCall(ref msg, ref callpos, ref declpos) => {
                writeln!(out, "{msg}")?;
                if let Some(declpos) = declpos {
                    SourcePos::show_two(
                        out,
                        callpos,
                        "invocation",
                        declpos,
                        "declaration",
                    )
                } else {
                    callpos.show(out)
                }
            }
            Error::Invalid(ref what, ref pos) => {
                writeln!(out, "{what}")?;
                pos.show(out)
            }
            Error::BadRange(ref err) => err.fmt(out),
            Error::IoError(ref err) => err.fmt(out),
        }
    }
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
    /// Declarations may only be used within style rules.
    DeclarationOutsideRule,
    /// Only properties are valid inside namespace rules.
    InNsRule,
    /// Global custom property not allowed.
    GlobalCustomProperty,
    /// Global namespaced property not allowed.
    GlobalNsProperty,
    /// Built-in modules can't be configured.
    ConfigBuiltin,
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
            Invalid::DeclarationOutsideRule => {
                "Declarations may only be used within style rules.".fmt(out)
            }
            Invalid::InNsRule => {
                "Only properties are valid inside namespace rules.".fmt(out)
            }
            Invalid::GlobalCustomProperty => {
                "Global custom property not allowed.".fmt(out)
            }
            Invalid::GlobalNsProperty => {
                "Global namespaced property not allowed.".fmt(out)
            }
            Invalid::ConfigBuiltin => {
                "Built-in modules can\'t be configured.".fmt(out)
            }
            Invalid::InScope(err) => err.fmt(out),
            Invalid::AtError(msg) => msg.fmt(out),
        }
    }
}

pub(crate) trait ResultPos<T> {
    /// Someting bad happened at a given source position.
    fn at(self, pos: &SourcePos) -> Result<T, Error>;
    /// Something bad happened, but we don't know the position.
    /// This is probably room for improvement in rsass error handling.
    fn no_pos(self) -> Result<T, Error>;
}
impl<T> ResultPos<T> for Result<T, Invalid> {
    fn at(self, pos: &SourcePos) -> Result<T, Error> {
        self.map_err(|e| e.at(pos.clone()))
    }
    fn no_pos(self) -> Result<T, Error> {
        self.map_err(|e| Error::S(e.to_string()))
    }
}
impl<T> ResultPos<T> for Result<T, InvalidCss> {
    fn at(self, pos: &SourcePos) -> Result<T, Error> {
        self.map_err(|e| Invalid::AtError(e.to_string()).at(pos.clone()))
    }
    fn no_pos(self) -> Result<T, Error> {
        self.map_err(|e| Error::S(e.to_string()))
    }
}
