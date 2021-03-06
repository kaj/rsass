use crate::css;
use crate::error::Error;
use crate::parser::SourcePos;
use crate::sass::{Name, Value};
use crate::value::ListSeparator;
use crate::ScopeRef;
use std::fmt;

/// The declared arguments of a mixin or function declaration.
///
/// The arguments are ordered (so they have a position).
/// Each argument also has a name and may have a default value.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct FormalArgs(Vec<(Name, Option<Value>)>, bool, SourcePos);

impl FormalArgs {
    /// Create a new FormalArgs.
    ///
    /// The given arg-pairs each have a name and a default value.
    /// It the default value is [`Null`](Value::Null), the argument
    /// does not have a default.
    ///
    /// If `is_varargs` is true, all extra call arguments are bundled
    /// as a List value for the last named argument.
    pub fn new(
        a: Vec<(Name, Option<Value>)>,
        is_varargs: bool,
        pos: SourcePos,
    ) -> Self {
        FormalArgs(a, is_varargs, pos)
    }

    /// Evaluate a set of call arguments for these formal arguments.
    ///
    /// Returns a Scope that is a sub-scope to the given `scope`.
    pub fn eval(
        &self,
        scope: ScopeRef,
        args: &css::CallArgs,
    ) -> Result<ScopeRef, ArgsError> {
        let argscope = ScopeRef::sub(scope);
        let n = self.0.len();
        let m = args.len();
        if !self.1 && m > n {
            return Err(ArgsError::TooMany(n, m));
        }
        for (i, &(ref name, ref default)) in self.0.iter().enumerate() {
            if let Some(value) = args
                .iter()
                .find(|&&(ref k, ref _v)| k.as_deref() == Some(name.as_ref()))
                .map(|&(ref _k, ref v)| v)
            {
                argscope.define(name.clone(), value);
            } else if self.1 && i + 1 == n && args.len() > n {
                if self.1 {
                    let args = args
                        .iter()
                        .skip(i)
                        .map(|&(_, ref v)| v.clone())
                        .collect();
                    argscope.define(
                        name.clone(),
                        &css::Value::List(args, ListSeparator::Comma, false),
                    );
                } else {
                    return Err(ArgsError::TooMany(n, args.len()));
                }
            } else {
                match args.get(i) {
                    Some(&(None, ref v)) => argscope.define(name.clone(), v),
                    _ => {
                        if let Some(default) = default {
                            let v = default
                                .do_evaluate(argscope.clone(), true)
                                .map_err(ArgsError::Eval)?;
                            argscope.define(name.clone(), &v)
                        } else if i + 1 == self.0.len() && self.1 {
                            // Should be an empty list?
                            argscope.define(name.clone(), &css::Value::Null)
                        } else {
                            return Err(ArgsError::Missing(name.clone()));
                        }
                    }
                };
            }
        }
        Ok(argscope)
    }
    /// Get the position of declaration for the function with these arguments.
    pub fn decl_pos(&self) -> &SourcePos {
        &self.2
    }
}

/// Error evaluating arguments
pub enum ArgsError {
    /// Got the first number of arguments, but only the second number allowed.
    TooMany(usize, usize),
    /// A required argument is missing
    Missing(Name),
    /// An error evaluating one of the arguments.
    Eval(Error),
}

impl fmt::Display for ArgsError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ArgsError::TooMany(n, m) => write!(
                out,
                "Error: Only {} argument{} allowed, but {} {} passed.",
                n,
                if *n != 1 { "s" } else { "" },
                m,
                if *m != 1 { "were" } else { "was" },
            ),
            ArgsError::Missing(name) => {
                write!(out, "Error: Missing argument ${}.", name)
            }
            ArgsError::Eval(e) => e.fmt(out),
        }
    }
}
