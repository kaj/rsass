use super::SassFunction;
use crate::css::Value;
use std::collections::btree_map::Iter;
use std::collections::BTreeMap;

#[derive(Debug, Default)]
pub struct Module {
    functions: BTreeMap<&'static str, SassFunction>,
    variables: BTreeMap<&'static str, Value>,
}

impl Module {
    pub fn new() -> Module {
        Default::default()
    }
    // FIXME: Rename to get_function
    pub fn get(&self, name: &str) -> Option<&SassFunction> {
        self.functions.get(name)
    }
    // FIXME: Rename to insert_function
    pub fn insert(&mut self, name: &'static str, function: SassFunction) {
        self.functions.insert(name, function);
    }
    pub fn functions(&self) -> Iter<&str, SassFunction> {
        self.functions.iter()
    }
    pub fn get_variable(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }
    pub fn set_variable(&mut self, name: &'static str, value: Value) {
        self.variables.insert(name, value);
    }
}
