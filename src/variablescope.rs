//! A scope is something that contains variable values.
use std::collections::BTreeMap;
use valueexpression::Value;

pub struct Scope {
    variables: BTreeMap<String, Value>,
}

impl Scope {
    pub fn new(values: &[(&str, Value)]) -> Self {
        let mut variables: BTreeMap<String, Value> = BTreeMap::new();
        for &(name, ref value) in values {
            variables.insert(name.to_string(), value.clone());
        }
        Scope { variables: variables }
    }
    pub fn get(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }
    pub fn evaluate(&self, val: &Value) -> String {
        match val {
            &Value::Literal(ref v) => v.clone(),
            &Value::Variable(ref name) => {
                self.get(&name)
                    .map(|n| self.evaluate(n))
                    .unwrap_or_else(|| format!("${}", name))
            }
        }
    }
}
