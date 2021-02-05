use super::SassFunction;
use crate::css::Value;
use crate::sass::Name;
use std::collections::BTreeMap;

#[derive(Debug, Default)]
pub struct Module {
    functions: BTreeMap<Name, SassFunction>,
    variables: BTreeMap<Name, Value>,
}

impl Module {
    pub fn new() -> Module {
        Default::default()
    }

    pub fn get_function(&self, name: &Name) -> Option<&SassFunction> {
        self.functions.get(name)
    }
    pub fn insert_function(&mut self, name: Name, function: SassFunction) {
        self.functions.insert(name, function);
    }
    pub fn functions(&self) -> impl Iterator<Item = (&Name, &SassFunction)> {
        self.functions.iter()
    }

    pub fn get_variable(&self, name: &Name) -> Option<&Value> {
        self.variables.get(name)
    }
    pub fn set_variable(&mut self, name: Name, value: Value) {
        self.variables.insert(name, value);
    }
    pub fn variables(&self) -> impl Iterator<Item = (&Name, &Value)> {
        self.variables.iter()
    }

    pub(super) fn expose(
        &mut self,
        name: &'static str,
        from: &Module,
        from_name: &'static str,
    ) {
        self.insert_function(
            Name::from_static(name),
            from.get_function(&Name::from_static(from_name))
                .unwrap()
                .clone(),
        );
    }
}
