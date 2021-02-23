use crate::css;
use crate::error::Error;
use crate::sass::Value;
use crate::Scope;
use std::default::Default;

/// the actual arguments of a function or mixin call.
///
/// Each argument has a Value.  Arguments may be named.
/// If the optional name is None, the argument is positional.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct CallArgs(Vec<(Option<String>, Value)>);

impl CallArgs {
    /// Create a new callargs from a vec of name-value pairs.
    ///
    /// The names is none for positional arguments.
    pub fn new(v: Vec<(Option<String>, Value)>) -> Self {
        CallArgs(v)
    }

    /// Create a new callargs from a single value.
    ///
    /// If the value is a list, it used as a positional argument list.
    /// Otherwise it is used as a single positional argument.
    pub fn from_value(v: Value) -> Self {
        match v {
            Value::List(v, _, false) => {
                CallArgs(v.into_iter().map(|v| (None, v)).collect())
            }
            v => CallArgs(vec![(None, v)]),
        }
    }

    /// Evaluate these sass CallArgs to css CallArgs.
    pub fn evaluate(
        &self,
        scope: &Scope,
        arithmetic: bool,
    ) -> Result<css::CallArgs, Error> {
        if let [(None, Value::List(list, _, false))] = &self.0[..] {
            if let [Value::Map(map), Value::Literal(mark)] = &list[..] {
                if mark.is_unquoted() && mark.single_raw() == Some("...") {
                    return Ok(css::CallArgs(
                        map.iter()
                            .map(|(k, v)| {
                                Ok((
                                    match k.do_evaluate(scope, arithmetic)? {
                                        css::Value::Null => None,
                                        css::Value::Literal(s, _) => Some(s),
                                        x => {
                                            return Err(Error::bad_value(
                                                "string", &x,
                                            ))
                                        }
                                    },
                                    v.do_evaluate(scope, arithmetic)?,
                                ))
                            })
                            .collect::<Result<Vec<_>, Error>>()?,
                    ));
                }
            }
        }
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
