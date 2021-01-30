use super::SassFunction;
use crate::css::Value;
use std::borrow::Cow;
use std::collections::BTreeMap;

pub type Key = Cow<'static, str>;

#[derive(Debug, Default)]
pub struct Module {
    functions: BTreeMap<Key, SassFunction>,
    variables: BTreeMap<Key, Value>,
}

impl Module {
    pub fn new() -> Module {
        Default::default()
    }

    pub fn get_function(&self, name: &str) -> Option<&SassFunction> {
        self.functions.get(name)
    }
    pub fn insert_function<T>(&mut self, name: T, function: SassFunction)
    where
        T: Into<Key>,
    {
        self.functions.insert(name.into(), function);
    }
    pub fn functions(&self) -> impl Iterator<Item = (&str, &SassFunction)> {
        self.functions.iter().map(|(n, v)| (n.as_ref(), v))
    }

    pub fn get_variable(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }
    pub fn set_variable<T: Into<Key>>(&mut self, name: T, value: Value) {
        self.variables.insert(name.into(), value);
    }
    pub fn variables(&self) -> impl Iterator<Item = (&str, &Value)> {
        self.variables.iter().map(|(n, v)| (n.as_ref(), v))
    }
}
