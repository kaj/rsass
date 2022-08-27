use super::{functions::ResolvedArgs, Call, CallError, Name, Value};
use crate::css::CallArgs;
use crate::{Error, ScopeError, ScopeRef, SourcePos};
use std::fmt;

type Result<T> = std::result::Result<T, ArgsError>;

/// The declared arguments of a mixin or function declaration.
///
/// The arguments are ordered (so they have a position).
/// Each argument also has a name and may have a default value.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct FormalArgs(Vec<(Name, Option<Value>)>, Option<Name>);

impl FormalArgs {
    /// Create a new FormalArgs.
    ///
    /// The given arg-pairs each have a name and an optional default value.
    pub fn new(args: Vec<(Name, Option<Value>)>) -> FormalArgs {
        FormalArgs(args, None)
    }
    /// Create a new set of varargs arguments
    pub fn new_va(args: Vec<(Name, Option<Value>)>) -> FormalArgs {
        let mut args = args;
        let va = args.pop().map(|(name, _)| name);
        FormalArgs(args, va)
    }
    /// Create an empty set of arguments.
    pub fn none() -> FormalArgs {
        FormalArgs(vec![], None)
    }

    /// Return true if this formalarg is varargs.
    pub fn is_varargs(&self) -> bool {
        self.1.is_some()
    }

    /// Evaluate a set of call arguments for a given call.
    ///
    /// Returns a Scope that is a sub-scope to the given `scope`.
    pub fn eval_call(
        &self,
        decl: ScopeRef,
        call: Call,
    ) -> Result<ResolvedArgs> {
        Ok(ResolvedArgs::new(self.eval(decl, call.args)?, call.scope))
    }

    /// Evaluate a set of call arguments for these formal arguments.
    ///
    /// Returns a Scope that is a sub-scope to the given `scope`.
    pub fn eval(&self, scope: ScopeRef, args: CallArgs) -> Result<ScopeRef> {
        let mut args = args;
        let argscope = ScopeRef::sub(scope);
        if !self.is_varargs() {
            let n = self.0.len();
            let m = args.len();
            if m > n {
                let n_p = args.positional.len();
                if n_p != m && n_p > n {
                    return Err(ArgsError::TooManyPos(n, n_p));
                } else {
                    return Err(ArgsError::TooMany(n, n_p));
                }
            }
        }
        let positional = args.take_positional(self.0.len());
        for ((name, _default), value) in self.0.iter().zip(&positional) {
            argscope.define(name.clone(), value.clone())?;
        }
        if self.0.len() > positional.len() {
            for (name, default) in &self.0[positional.len()..] {
                if let Some(v) = args.named.remove(name) {
                    argscope.define(name.clone(), v)?;
                } else if let Some(default) = default {
                    argscope.define(
                        name.clone(),
                        default.do_evaluate(argscope.clone(), true)?,
                    )?;
                } else {
                    return Err(ArgsError::Missing(name.clone()));
                }
            }
        }
        if let Some(va_name) = &self.1 {
            argscope.define(
                va_name.clone(),
                args.only_named(va_name).unwrap_or_else(|| args.into()),
            )?;
        } else {
            args.check_no_named()?;
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
        if let Some(va) = &self.1 {
            write!(out, ", ${}...", va)?;
        }
        out.write_str(")")
    }
}

/// Error evaluating arguments
#[derive(Debug)]
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
    Eval(Box<Error>),
}

impl ArgsError {
    /// This argument error happend for args declared at the given pos.
    pub fn declared_at(self, pos: &SourcePos) -> CallError {
        match self {
            ArgsError::Eval(e) => CallError::Wrap(e),
            ae => CallError::Args(ae, pos.clone()),
        }
    }
}

impl fmt::Display for ArgsError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ArgsError::TooManyPos(n, m) => write!(
                out,
                "Only {} positional argument{} allowed, but {} {} passed.",
                n,
                if *n != 1 { "s" } else { "" },
                m,
                if *m != 1 { "were" } else { "was" },
            ),
            ArgsError::TooMany(n, m) => write!(
                out,
                "Only {} argument{} allowed, but {} {} passed.",
                n,
                if *n != 1 { "s" } else { "" },
                m,
                if *m != 1 { "were" } else { "was" },
            ),
            ArgsError::Missing(name) => {
                write!(out, "Missing argument ${}.", name)
            }
            ArgsError::Unexpected(name) => {
                write!(out, "No argument named ${}.", name)
            }
            ArgsError::Eval(e) => e.fmt(out),
        }
    }
}

impl From<Error> for ArgsError {
    fn from(e: Error) -> ArgsError {
        ArgsError::Eval(Box::new(e))
    }
}
impl From<ScopeError> for ArgsError {
    fn from(e: ScopeError) -> ArgsError {
        Error::from(e).into()
    }
}
