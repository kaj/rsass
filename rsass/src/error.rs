use crate::css::InvalidCss;
use crate::input::{LoadError, SourcePos};
use crate::parser::ParseError;
use crate::value::RangeError;
use crate::ScopeError;
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
            Self::S(ref s) => write!(out, "{s}"),
            Self::Input(ref load) => load.fmt(out),
            Self::ParseError(ref err) => fmt::Display::fmt(err, out),
            Self::ImportLoop(ref module, ref pos, ref oldpos) => {
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
            Self::BadCall(ref msg, ref callpos, ref declpos) => {
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
            Self::Invalid(ref what, ref pos) => {
                writeln!(out, "{what}")?;
                pos.show(out)
            }
            Self::BadRange(ref err) => err.fmt(out),
            Self::IoError(ref err) => err.fmt(out),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}
impl From<fmt::Error> for Error {
    fn from(e: fmt::Error) -> Self {
        Self::IoError(io::Error::new(io::ErrorKind::Other, e))
    }
}

impl From<ParseError> for Error {
    fn from(e: ParseError) -> Self {
        Self::ParseError(e)
    }
}
impl From<RangeError> for Error {
    fn from(e: RangeError) -> Self {
        Self::BadRange(e)
    }
}

impl From<LoadError> for Error {
    fn from(err: LoadError) -> Self {
        Self::Input(err)
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
            Self::FunctionName => "Invalid function name.".fmt(out),
            Self::AtRule => "This at-rule is not allowed here.".fmt(out),
            Self::MixinInMixin => {
                "Mixins may not contain mixin declarations.".fmt(out)
            }
            Self::MixinInControl => {
                "Mixins may not be declared in control directives.".fmt(out)
            }
            Self::FunctionInMixin => {
                "Mixins may not contain function declarations.".fmt(out)
            }
            Self::FunctionInControl => {
                "Functions may not be declared in control directives."
                    .fmt(out)
            }
            Self::DuplicateArgument => "Duplicate argument.".fmt(out),
            Self::PositionalArgAfterNamed => {
                "Positional arguments must come before keyword arguments."
                    .fmt(out)
            }
            Self::DeclarationOutsideRule => {
                "Declarations may only be used within style rules.".fmt(out)
            }
            Self::InNsRule => {
                "Only properties are valid inside namespace rules.".fmt(out)
            }
            Self::GlobalCustomProperty => {
                "Global custom property not allowed.".fmt(out)
            }
            Self::GlobalNsProperty => {
                "Global namespaced property not allowed.".fmt(out)
            }
            Self::ConfigBuiltin => {
                "Built-in modules can\'t be configured.".fmt(out)
            }
            Self::InScope(err) => err.fmt(out),
            Self::AtError(msg) => msg.fmt(out),
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
