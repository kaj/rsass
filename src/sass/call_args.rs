use super::{Name, Value};
use crate::css;
use crate::error::Error;
use crate::ScopeRef;
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
        scope: ScopeRef,
        arithmetic: bool,
    ) -> Result<css::CallArgs, Error> {
        Ok(css::CallArgs(self.0.iter().fold(
            Ok(vec![]),
            |acc, (name, value)| {
                let mut acc = acc?;
                if let (None, Value::List(list, _, false)) = (name, value) {
                    if list.len() == 2 && is_mark(&list[1]) {
                        let splat =
                            list[0].do_evaluate(scope.clone(), arithmetic)?;
                        match splat {
                            css::Value::Map(map) => {
                                for (k, v) in map {
                                    let k = match k {
                                        css::Value::Null => None,
                                        css::Value::Literal(s, _) => Some(s),
                                        x => {
                                            return Err(Error::bad_value(
                                                "string", &x,
                                            ))
                                        }
                                    };
                                    acc.push((k, v));
                                }
                                return Ok(acc);
                            }
                            css::Value::Null => (),
                            css::Value::List(items, ..) => {
                                for item in items {
                                    acc.push((None, item));
                                }
                            }
                            item => {
                                acc.push((None, item));
                            }
                        }
                        return Ok(acc);
                    }
                }
                acc.push((
                    name.clone(),
                    value.do_evaluate(scope.clone(), arithmetic)?,
                ));
                Ok(acc)
            },
        )?))
    }

    /// Evaluate a single argument
    ///
    /// Only used by the `if` function, which is the only sass
    /// function that evaluates its arguments lazily.
    pub fn evaluate_single(
        &self,
        scope: ScopeRef,
        name: Name,
        num: usize,
    ) -> Result<css::Value, Error> {
        self.0
            .iter()
            .find(|(n, _)| n.as_ref().map(Name::from).as_ref() == Some(&name))
            .or(self.0.get(num))
            .map(|(_, v)| v.do_evaluate(scope, true))
            .unwrap_or(Ok(css::Value::Null))
    }
}

impl Default for CallArgs {
    fn default() -> Self {
        CallArgs(vec![])
    }
}

fn is_mark(v: &Value) -> bool {
    match v {
        Value::Literal(v, ..) => {
            v.is_unquoted() && v.single_raw() == Some("...")
        }
        _ => false,
    }
}
