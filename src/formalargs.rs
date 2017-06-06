use std::default::Default;
use std::fmt;
use value::{ListSeparator, Value};
use variablescope::{Scope, ScopeImpl};

/// The declared arguments of a mixin or function declaration.
///
/// The arguments are ordered (so they have a position).
/// Each argument also has a name and may have a default value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FormalArgs(Vec<(String, Value)>, bool);

impl FormalArgs {
    pub fn new(a: Vec<(String, Value)>, is_varargs: bool) -> Self {
        FormalArgs(a, is_varargs)
    }

    pub fn eval<'a>(&self, scope: &'a Scope, args: &CallArgs) -> ScopeImpl<'a> {
        let mut argscope = ScopeImpl::sub(scope);
        let n = self.0.len();
        for (i, &(ref name, ref default)) in self.0.iter().enumerate() {
            if let Some(value) = args.0
                   .iter()
                   .find(|&&(ref k, ref _v)| k.as_ref() == Some(name))
                   .map(|&(ref _k, ref v)| v) {
                argscope.define(name, value);
            } else if self.1 && i + 1 == n && args.0.len() > n {
                let args =
                    args.0[i..].iter().map(|&(_, ref v)| v.clone()).collect();
                argscope.define(name, &Value::List(args, ListSeparator::Comma));
            } else {
                argscope.define(name, match args.0.get(i) {
                    Some(&(None, ref v)) => v,
                    _ => default,
                });
            }
        }
        argscope
    }
}

impl Default for FormalArgs {
    fn default() -> Self {
        FormalArgs::new(vec![], false)
    }
}

/// the actual arguments of a function or mixin call.
///
/// Each argument has a Value.  Arguments may be named.
/// If the optional name is None, the argument is positional.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CallArgs(Vec<(Option<String>, Value)>);

impl CallArgs {
    pub fn new(v: Vec<(Option<String>, Value)>) -> Self {
        CallArgs(v)
    }

    pub fn from_value(v: Value) -> Self {
        match v {
            Value::List(v, _) => {
                CallArgs(v.into_iter().map(|v| (None, v)).collect())
            }
            v => CallArgs(vec![(None, v)]),
        }
    }

    pub fn xyzzy(&self, scope: &Scope) -> Self {
        CallArgs(self.0
                     .iter()
                     .map(|&(ref n, ref v)| (n.clone(), v.evaluate(scope)))
                     .collect())
    }
}

impl Default for CallArgs {
    fn default() -> Self {
        CallArgs(vec![])
    }
}

impl fmt::Display for CallArgs {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        let t = self.0
            .iter()
            .map(|kv| match *kv {
                     (Some(ref k), ref v) => format!("${}: {}", k, v),
                     (None, ref v) => format!("{}", v),
                 })
            .collect::<Vec<_>>()
            .join(", ");
        write!(out, "{}", t)
    }
}
