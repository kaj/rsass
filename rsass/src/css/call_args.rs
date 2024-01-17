use super::{is_not, Value};
use crate::ordermap::OrderMap;
use crate::sass::{ArgsError, Name};
use crate::value::ListSeparator;
use std::default::Default;
use std::fmt;

/// the actual arguments of a function or mixin call.
///
/// There are both positional and named values.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd)]
pub struct CallArgs {
    pub(crate) positional: Vec<Value>,
    // Ordered for formattig.
    pub(crate) named: OrderMap<Name, Value>,
    pub(crate) trailing_comma: bool,
}

impl CallArgs {
    /// Create args from a Value.
    ///
    /// The value may be a list of arguments or a single argument.
    pub fn from_value(v: Value) -> Result<Self, String> {
        match v {
            Value::ArgList(args) => Ok(args),
            Value::List(v, Some(ListSeparator::Comma), false) => {
                Ok(CallArgs::from_list(v))
            }
            Value::Map(args) => {
                let mut result = CallArgs::default();
                result.add_from_value_map(args)?;
                Ok(result)
            }
            Value::Null => Ok(CallArgs::default()),
            v => Ok(CallArgs::from_single(v)),
        }
    }
    /// Create args that is one single positional argument.
    pub(crate) fn from_single(arg: Value) -> Self {
        CallArgs::from_list(vec![arg])
    }

    pub(crate) fn from_list(positional: Vec<Value>) -> Self {
        CallArgs {
            positional,
            ..Default::default()
        }
    }

    pub(crate) fn add_from_value_map(
        &mut self,
        map: OrderMap<Value, Value>,
    ) -> Result<(), String> {
        for (k, v) in map {
            match k {
                Value::Null => self.positional.push(v),
                Value::Literal(s) => {
                    self.named.insert(s.value().into(), v);
                }
                x => return Err(is_not(&x, "a string")),
            }
        }
        Ok(())
    }

    pub(crate) fn take_positional(&mut self, n: usize) -> Vec<Value> {
        let n = std::cmp::min(n, self.positional.len());
        let t = self.positional.split_off(n);
        std::mem::replace(&mut self.positional, t)
    }
    pub(crate) fn only_named(&mut self, name: &Name) -> Option<Value> {
        if self.positional.is_empty() && self.named.len() == 1 {
            self.named.remove(name)
        } else {
            None
        }
    }

    pub(crate) fn get_single(&self) -> Result<&Value, ArgsError> {
        self.check_no_named()?;
        match self.positional.len() {
            1 => Ok(self.positional.first().unwrap()),
            0 => Ok(&Value::Null),
            n => Err(ArgsError::TooMany(1, n)),
        }
    }

    pub(crate) fn check_no_named(&self) -> Result<(), ArgsError> {
        if let Some(extra) = self.named.keys().next() {
            Err(ArgsError::Unexpected(extra.clone()))
        } else {
            Ok(())
        }
    }

    /// Get number of arguments.
    pub fn len(&self) -> usize {
        self.positional.len() + self.named.len()
    }

    /// Return true if the argument list is empty.
    pub fn is_empty(&self) -> bool {
        self.positional.is_empty() && self.named.is_empty()
    }
}

impl From<CallArgs> for Value {
    fn from(args: CallArgs) -> Value {
        Value::ArgList(args)
    }
}

impl fmt::Display for CallArgs {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        let pos = self
            .positional
            .iter()
            .map(|v| format!("{}", v.format(Default::default())));
        let named = self
            .named
            .iter()
            .map(|(k, v)| format!("{}={}", k, v.format(Default::default())));
        let t = pos.chain(named).collect::<Vec<_>>().join(", ");
        write!(out, "{t}")?;
        if self.trailing_comma {
            write!(out, ", ")?;
        }
        Ok(())
    }
}
