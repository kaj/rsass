use super::{Value, is_not};
use crate::ordermap::OrderMap;
use crate::output::Format;
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
                Ok(Self::from_list(v))
            }
            Value::Map(args) => {
                let mut result = Self::default();
                result.add_from_value_map(args)?;
                Ok(result)
            }
            Value::Null => Ok(Self::default()),
            v => Ok(Self::from_single(v)),
        }
    }
    /// Create args that is one single positional argument.
    pub(crate) fn from_single(arg: Value) -> Self {
        Self::from_list(vec![arg])
    }

    pub(crate) fn from_list(positional: Vec<Value>) -> Self {
        Self {
            positional,
            ..Default::default()
        }
    }

    pub(crate) fn from_iter<T, I>(positional: I) -> Self
    where
        T: Into<Value>,
        I: IntoIterator<Item = T>,
    {
        Self::from_list(positional.into_iter().map(Into::into).collect())
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
            0 | 1 => Ok(self.positional.first().unwrap_or(&Value::Null)),
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
    fn from(args: CallArgs) -> Self {
        Self::ArgList(args)
    }
}

impl fmt::Display for CallArgs {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        let format = Format::default(); // TODO: Get current output format!
        let sep = if format.is_compressed() { "," } else { ", " };
        if let Some((first, rest)) = self.positional.split_first() {
            first.format(format).fmt(out)?;
            for item in rest {
                out.write_str(sep)?;
                item.format(format).fmt(out)?;
            }
            if !self.named.is_empty() {
                out.write_str(sep)?;
            }
        }
        let mut named = self.named.iter();
        if let Some((key, val)) = named.next() {
            out.write_str(key.as_ref())?;
            out.write_str("=")?;
            val.format(format).fmt(out)?;
        }
        for (key, val) in named {
            out.write_str(sep)?;
            out.write_str(key.as_ref())?;
            out.write_str("=")?;
            val.format(format).fmt(out)?;
        }
        if self.trailing_comma {
            out.write_str(sep)?;
        }
        Ok(())
    }
}
