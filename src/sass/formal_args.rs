use crate::css;
use crate::error::Error;
use crate::sass::{Name, Value};
use crate::value::ListSeparator;
use crate::variablescope::{Scope, ScopeImpl};
use std::default::Default;

/// The declared arguments of a mixin or function declaration.
///
/// The arguments are ordered (so they have a position).
/// Each argument also has a name and may have a default value.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct FormalArgs(Vec<(Name, Value)>, bool);

impl FormalArgs {
    /// Create a new FormalArgs.
    ///
    /// The given arg-pairs each have a name and a default value.
    /// It the default value is [`Null`](Value::Null), the argument
    /// does not have a default.
    ///
    /// If `is_varargs` is true, all extra call arguments are bundled
    /// as a List value for the last named argument.
    pub fn new(a: Vec<(Name, Value)>, is_varargs: bool) -> Self {
        FormalArgs(a, is_varargs)
    }

    /// Evaluate a set of call arguments for these formal arguments.
    ///
    /// Returns a Scope that is a sub-scope to the given `scope`.
    pub fn eval<'a>(
        &self,
        scope: &'a dyn Scope,
        args: &css::CallArgs,
    ) -> Result<ScopeImpl<'a>, Error> {
        let mut argscope = ScopeImpl::sub(scope);
        let n = self.0.len();
        for (i, &(ref name, ref default)) in self.0.iter().enumerate() {
            if let Some(value) = args
                .iter()
                .find(|&&(ref k, ref _v)| k.as_deref() == Some(name.as_ref()))
                .map(|&(ref _k, ref v)| v)
            {
                argscope.define(name.clone(), value);
            } else if self.1 && i + 1 == n && args.len() > n {
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
                match args.get(i) {
                    Some(&(None, ref v)) => argscope.define(name.clone(), v),
                    _ => {
                        let v = default.do_evaluate(&argscope, true)?;
                        argscope.define(name.clone(), &v)
                    }
                };
            }
        }
        Ok(argscope)
    }
}

impl Default for FormalArgs {
    fn default() -> Self {
        FormalArgs::new(vec![], false)
    }
}
