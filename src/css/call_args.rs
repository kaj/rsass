use super::Value;
use crate::value::{ListSeparator, Quotes};
use std::default::Default;
use std::fmt;

/// the actual arguments of a function or mixin call.
///
/// Each argument has a Value.  Arguments may be named.
/// If the optional name is None, the argument is positional.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct CallArgs(pub Vec<(Option<String>, Value)>);

impl CallArgs {
    /// Create args from a vec of values with optional names.
    pub fn new(v: Vec<(Option<String>, Value)>) -> Self {
        CallArgs(v)
    }

    /// Create args from a Value.
    ///
    /// The value may be a list of arguments or a single argument.
    /// There is also special recognition of a list ending in an
    /// unquoted "..." string, makring a varargs argument list.
    pub fn from_value(v: Value) -> Self {
        match v {
            Value::List(mut v, Some(ListSeparator::Comma), false) => {
                if v.len() == 2 && is_mark(&v[1]) {
                    match &v[0] {
                        Value::Map(map) => {
                            return CallArgs(
                                map.iter()
                                    .map(|(k, v)| {
                                        (
                                            match k {
                                                Value::Null => None,
                                                Value::Literal(s, _) => {
                                                    Some(s.clone())
                                                }
                                                _x => None, // TODO return Err(Error::bad_value("string", &x)),
                                            },
                                            v.clone(),
                                        )
                                    })
                                    .collect(),
                            );
                        }
                        Value::List(list, ..) => {
                            return CallArgs(
                                list.iter()
                                    .map(|v| (None, v.clone()))
                                    .collect(),
                            );
                        }
                        _ => (),
                    }
                }
                if let Some(last) = v.pop() {
                    if let Value::List(
                        vv,
                        Some(ListSeparator::Space),
                        false,
                    ) = &last
                    {
                        match &vv[..] {
                            [Value::List(vv, _, _), Value::Literal(mark, Quotes::None)]
                                if mark == "..." =>
                            {
                                v.extend(vv.iter().cloned());
                            }
                            _ => {
                                v.push(last);
                            }
                        }
                    } else {
                        v.push(last);
                    }
                }
                CallArgs(v.into_iter().map(|v| (None, v)).collect())
            }
            Value::Null => CallArgs(vec![]),
            v => CallArgs(vec![(None, v)]),
        }
    }

    /// Iterate over values (and their optional names).
    pub fn iter(&self) -> ::std::slice::Iter<(Option<String>, Value)> {
        self.0.iter()
    }

    /// Get number of arguments.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Return true if the argument list is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Get a specific argument by position.
    pub fn get(&self, index: usize) -> Option<&(Option<String>, Value)> {
        self.0.get(index)
    }
}

fn is_mark(v: &Value) -> bool {
    match v {
        Value::Literal(mark, Quotes::None) => mark == "...",
        _ => false,
    }
}

impl Default for CallArgs {
    fn default() -> Self {
        CallArgs(vec![])
    }
}

impl fmt::Display for CallArgs {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        let t = self
            .0
            .iter()
            .map(|kv| match *kv {
                (Some(ref k), ref v) => {
                    format!("${}: {}", k, v.format(Default::default()))
                }
                (None, ref v) => format!("{}", v.format(Default::default())),
            })
            .collect::<Vec<_>>()
            .join(", ");
        write!(out, "{}", t)
    }
}
