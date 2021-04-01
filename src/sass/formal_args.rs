use crate::css;
use crate::error::Error;
use crate::sass::{Name, Value};
use crate::value::ListSeparator;
use crate::ScopeRef;
use std::fmt;

/// The declared arguments of a mixin or function declaration.
///
/// The arguments are ordered (so they have a position).
/// Each argument also has a name and may have a default value.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct FormalArgs(Vec<(Name, Option<Value>)>, bool);

impl FormalArgs {
    /// Create a new FormalArgs.
    ///
    /// The given arg-pairs each have a name and an optional default value.
    pub fn new(args: Vec<(Name, Option<Value>)>) -> FormalArgs {
        FormalArgs(args, false)
    }
    /// Create a new set of varargs arguments
    pub fn new_va(args: Vec<(Name, Option<Value>)>) -> FormalArgs {
        FormalArgs(args, true)
    }
    /// Create an empty set of arguments.
    pub fn none() -> FormalArgs {
        FormalArgs(vec![], false)
    }

    /// Return true if this formalarg is varargs.
    pub fn is_varargs(&self) -> bool {
        self.1
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
        if !self.is_varargs() && m > n {
            return Err(ArgsError::TooMany(n, m));
        }
        for (i, &(ref name, ref default)) in self.0.iter().enumerate() {
            if let Some(value) = args
                .iter()
                .find(|&&(ref k, ref _v)| k.as_deref() == Some(name.as_ref()))
                .map(|&(ref _k, ref v)| v)
            {
                argscope.define(name.clone(), value);
            } else if self.is_varargs() && i + 1 == n && args.len() > n {
                if self.is_varargs() {
                    let args = args
                        .iter()
                        .skip(i)
                        .map(|&(_, ref v)| v.clone())
                        .collect();
                    argscope.define(
                        name.clone(),
                        &css::Value::List(
                            args,
                            Some(ListSeparator::Comma),
                            false,
                        ),
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
                        } else if i + 1 == self.0.len() && self.is_varargs() {
                            argscope.define(
                                name.clone(),
                                &css::Value::List(vec![], None, false),
                            )
                        } else {
                            return Err(ArgsError::Missing(name.clone()));
                        }
                    }
                };
            }
        }
        Ok(argscope)
    }
}

impl fmt::Display for FormalArgs {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        out.write_str("(")?;
        if let Some((first, rest)) = self.0.split_first() {
            write!(out, "${}", first.0)?;
            if let Some(default) = &first.1 {
                out.write_str(": ")?;
                default.inspect(out)?;
            }
            for (name, default) in rest {
                write!(out, ", ${}", name)?;
                if let Some(default) = default {
                    out.write_str(": ")?;
                    default.inspect(out)?;
                }
            }
        }
        if self.1 {
            out.write_str("...")?;
        }
        out.write_str(")")
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
