use super::{Name, Value};
use crate::css;
use crate::error::Error;
use crate::ordermap::OrderMap;
use crate::ScopeRef;
use std::default::Default;

/// the actual arguments of a function or mixin call.
///
/// Each argument has a Value.  Arguments may be named.
/// If the optional name is None, the argument is positional.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd)]
pub struct CallArgs {
    positional: Vec<Value>,
    // Ordered for formattig.
    named: OrderMap<Name, Value>,
}

impl CallArgs {
    /// Create a new callargs from a vec of name-value pairs.
    ///
    /// The names is none for positional arguments.
    pub fn new(v: Vec<(Option<Name>, Value)>) -> Result<Self, Error> {
        let mut positional = Vec::new();
        let mut named = OrderMap::new();
        for (name, value) in v {
            if let Some(name) = name {
                if let Some(_old) = named.insert(name, value) {
                    return Err(Error::error("Duplicate argument."));
                }
            } else if named.is_empty() || is_splat(&value).is_some() {
                positional.push(value);
            } else {
                return Err(Error::error("positional arg after named"));
            }
        }
        Ok(CallArgs { positional, named })
    }

    /// Evaluate these sass CallArgs to css CallArgs.
    pub fn evaluate(&self, scope: ScopeRef) -> Result<css::CallArgs, Error> {
        let positional = Vec::new();
        let named = self.named.iter().try_fold(
            OrderMap::new(),
            |mut acc, (name, arg)| {
                arg.do_evaluate(scope.clone(), true).map(|arg| {
                    acc.insert(name.clone(), arg);
                    acc
                })
            },
        )?;
        let mut result = css::CallArgs { positional, named };
        for arg in &self.positional {
            match is_splat(arg) {
                Some([one]) => match one.do_evaluate(scope.clone(), true)? {
                    css::Value::ArgList(args) => {
                        result.positional.extend(args.positional);
                        for (name, value) in args.named {
                            if let Some(_existing) =
                                result.named.insert(name, value)
                            {
                                return Err(Error::error(
                                    "Duplicate argument",
                                ));
                            }
                        }
                    }
                    css::Value::Map(map) => {
                        result.add_from_value_map(map)?;
                    }
                    css::Value::List(items, ..) => {
                        for item in items {
                            result.positional.push(item);
                        }
                    }
                    css::Value::Null => (),
                    item => {
                        result.positional.push(item);
                    }
                },
                Some(splat) => {
                    for arg in splat {
                        result
                            .positional
                            .push(arg.do_evaluate(scope.clone(), true)?);
                    }
                }
                None => {
                    result
                        .positional
                        .push(arg.do_evaluate(scope.clone(), true)?);
                }
            }
        }
        Ok(result)
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
        // TODO: Error if both name and posinal exists?
        if let Some(v) = self.named.get(&name) {
            return v.do_evaluate(scope, true);
        }
        let mut i = 0;
        for a in &self.positional {
            match is_splat(a) {
                Some([one]) => match one.do_evaluate(scope.clone(), true)? {
                    css::Value::ArgList(args) => {
                        if let Some(v) = args
                            .named
                            .get(&name)
                            .or_else(|| args.positional.get(num - i))
                        {
                            return Ok(v.clone());
                        }
                        i += if args.named.is_empty() {
                            args.len()
                        } else {
                            num + 1
                        };
                    }
                    css::Value::Map(map) => {
                        if let Some(v) = map.get(&name.to_string().into()) {
                            return Ok(v.clone());
                        }
                        i += num + 1;
                    }
                    css::Value::List(items, ..) => {
                        if let Some(v) = items.get(num - i) {
                            return Ok(v.clone());
                        } else {
                            i += items.len();
                        }
                    }
                    css::Value::Null => (),
                    v => {
                        if i == num {
                            return Ok(v);
                        } else {
                            i += 1;
                        }
                    }
                },
                Some(splat) => {
                    if let Some(v) = splat.get(num - i) {
                        return v.do_evaluate(scope, true);
                    } else {
                        i += splat.len();
                    }
                }
                None => {
                    if i == num {
                        return a.do_evaluate(scope, true);
                    } else {
                        i += 1;
                    }
                }
            }
        }
        Ok(css::Value::Null)
    }
}

fn is_splat(arg: &Value) -> Option<&[Value]> {
    if let Value::List(list, _, false) = arg {
        // TODO?  Check that sep is None or Space?
        if let Some((Value::Literal(v, ..), splat)) = list.split_last() {
            if v.is_unquoted() && v.single_raw() == Some("...") {
                return Some(splat);
            }
        }
    }
    None
}
