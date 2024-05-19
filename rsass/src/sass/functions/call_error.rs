use super::super::{ArgsError, Name};
use crate::css::{BadSelector, Value};
use crate::input::SourcePos;
use crate::output::Format;
use crate::{Error, Invalid, ScopeError};
use std::fmt;

/// An error in calling a function.
#[derive(Debug)]
pub enum CallError {
    /// Something invalid while executing call.
    Invalid(Invalid),
    /// An illegal value for a specific parameter.
    BadArgument(Name, String),
    /// An error matching arguments.  The position is the declaration.
    Args(ArgsError, SourcePos),
    /// An error happened during execution of the call.
    Wrap(Box<Error>),
}

impl CallError {
    /// A generic error message inside a function.
    ///
    /// This is like using an `@error` statement inside a
    /// sass-implemented function.
    pub fn msg<S: ToString>(msg: S) -> Self {
        Self::Invalid(Invalid::AtError(msg.to_string()))
    }

    /// The values were expected to be compatible, but wasn't.
    pub fn incompatible_values<T: Into<Value>>(a: T, b: T) -> Self {
        Self::msg(format!(
            "{} and {} are incompatible.",
            a.into().format(Format::introspect()),
            b.into().format(Format::introspect()),
        ))
    }

    /// Map this error to a [`crate::Error`].
    pub fn called_from(self, call_pos: &SourcePos, name: &str) -> Error {
        match self {
            Self::Invalid(Invalid::AtError(msg)) => {
                Error::BadCall(msg, call_pos.clone().opt_in_calc(), None)
            }
            Self::Invalid(err) => {
                Error::BadCall(format!("{err:?}"), call_pos.clone(), None)
            }
            Self::BadArgument(name, problem) => Error::BadCall(
                if name.as_ref().is_empty() {
                    problem
                } else {
                    format!("${name}: {problem}")
                },
                call_pos.clone(),
                None,
            ),
            Self::Args(ArgsError::Eval(err), _decl) => *err,
            Self::Args(err, decl) => Error::BadCall(
                err.to_string(),
                if decl.is_builtin() {
                    call_pos.clone()
                } else {
                    call_pos.in_call(name)
                },
                Some(decl),
            ),
            Self::Wrap(err) => *err,
        }
    }
}
impl std::error::Error for CallError {}

impl From<ScopeError> for CallError {
    fn from(err: ScopeError) -> Self {
        Self::Invalid(Invalid::InScope(err))
    }
}
impl From<Invalid> for CallError {
    fn from(err: Invalid) -> Self {
        Self::Invalid(err)
    }
}
impl From<BadSelector> for CallError {
    fn from(e: BadSelector) -> Self {
        Self::msg(e)
    }
}
impl From<Error> for CallError {
    fn from(e: Error) -> Self {
        Self::Wrap(Box::new(e))
    }
}
impl fmt::Display for CallError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {self:?}")
    }
}
