use crate::css;
use crate::error::Error;
use crate::sass::{Name, Value};
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
        args: css::CallArgs,
    ) -> Result<ScopeRef, ArgsError> {
        let mut args = args;
        let argscope = ScopeRef::sub(scope);
        let n = self.0.len();
        let m = args.len();
        if !self.is_varargs() {
            if m > n {
                let n_p = args.positional.len();
                if n_p != m && n_p > n {
                    return Err(ArgsError::TooManyPos(n, n_p));
                } else {
                    return Err(ArgsError::TooMany(n, n_p));
                }
            }
            let (by_pos, by_name) = self.0.split_at(args.positional.len());
            for ((name, _default), value) in
                by_pos.iter().zip(&args.positional)
            {
                argscope.define(name.clone(), &value);
            }
            for (name, default) in by_name {
                if let Some(v) = args.named.remove(name) {
                    argscope.define(name.clone(), &v);
                } else if let Some(default) = default {
                    argscope.define(
                        name.clone(),
                        &default
                            .do_evaluate(argscope.clone(), true)
                            .map_err(ArgsError::Eval)?,
                    );
                } else {
                    return Err(ArgsError::Missing(name.clone()));
                }
            }
            if let Some((extra, _)) = args.named.iter().next() {
                return Err(ArgsError::Unexpected(extra.clone()));
            }
        } else {
            // Va args must have at least one!
            // TODO: use va: Option<Name> to remove flag and condtion?
            let (va, normal) = self.0.split_last().unwrap();

            let positional = args.take_positional(normal.len());
            for ((name, _default), value) in normal.iter().zip(&positional) {
                argscope.define(name.clone(), value);
            }
            if normal.len() > positional.len() {
                for (name, default) in &normal[positional.len()..] {
                    if let Some(v) = args.named.remove(name) {
                        argscope.define(name.clone(), &v);
                    } else if let Some(default) = default {
                        argscope.define(
                            name.clone(),
                            &default
                                .do_evaluate(argscope.clone(), true)
                                .map_err(ArgsError::Eval)?,
                        );
                    } else {
                        return Err(ArgsError::Missing(name.clone()));
                    }
                }
            }

            argscope.define(
                va.0.clone(),
                &if let Some(v) = args.only_named(&va.0) {
                    v
                } else {
                    args.into()
                },
            );
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
    /// Got the first number of positional arguments, but only the second number allowed.
    TooManyPos(usize, usize),
    /// A required argument is missing
    Missing(Name),
    /// Got unexpected named argumet
    Unexpected(Name),
    /// An error evaluating one of the arguments.
    Eval(Error),
}

impl fmt::Display for ArgsError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ArgsError::TooManyPos(n, m) => write!(
                out,
                "Error: Only {} positional argument{} allowed, but {} {} passed.",
                n,
                if *n != 1 { "s" } else { "" },
                m,
                if *m != 1 { "were" } else { "was" },
            ),
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
            ArgsError::Unexpected(name) => {
                write!(out, "Error: No argument named ${}.", name)
            }
            ArgsError::Eval(e) => e.fmt(out),
        }
    }
}
