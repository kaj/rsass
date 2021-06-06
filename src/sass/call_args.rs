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
            if let Some(splat) = is_splat(arg) {
                match splat.do_evaluate(scope.clone(), true)? {
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
                }
            } else {
                result
                    .positional
                    .push(arg.do_evaluate(scope.clone(), true)?);
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
        self.positional
            .get(num)
            .or_else(|| self.named.get(&name))
            .map(|v| v.do_evaluate(scope, true))
            .unwrap_or(Ok(css::Value::Null))
    }
}

fn is_splat(arg: &Value) -> Option<&Value> {
    match arg {
        Value::List(list, _, false) => match &list[..] {
            [splat, Value::Literal(v, ..)]
                if v.is_unquoted() && v.single_raw() == Some("...") =>
            {
                Some(splat)
            }
            _ => None,
        },
        _ => None,
    }
}
