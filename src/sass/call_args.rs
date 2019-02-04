use crate::css;
use crate::error::Error;
use crate::sass::Value;
use std::default::Default;
use crate::variablescope::Scope;

/// the actual arguments of a function or mixin call.
///
/// Each argument has a Value.  Arguments may be named.
/// If the optional name is None, the argument is positional.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CallArgs(Vec<(Option<String>, Value)>);

impl CallArgs {
    pub fn new(v: Vec<(Option<String>, Value)>) -> Self {
        CallArgs(v)
    }

    pub fn from_value(v: Value) -> Self {
        match v {
            Value::List(v, _, false, _) => {
                CallArgs(v.into_iter().map(|v| (None, v)).collect())
            }
            v => CallArgs(vec![(None, v)]),
        }
    }

    pub fn iter(&self) -> ::std::slice::Iter<(Option<String>, Value)> {
        self.0.iter()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&(Option<String>, Value)> {
        self.0.get(index)
    }

    pub fn evaluate(
        &self,
        scope: &Scope,
        arithmetic: bool,
    ) -> Result<css::CallArgs, Error> {
        let args = self.0
                .iter()
                .map(|&(ref n, ref v)| -> Result<(Option<String>, css::Value), Error> {
                    Ok((n.clone(), v.do_evaluate(scope, arithmetic)?))
                })
                .collect::<Result<Vec<_>, Error>>()?;
        Ok(css::CallArgs(args))
    }
}

impl Default for CallArgs {
    fn default() -> Self {
        CallArgs(vec![])
    }
}
