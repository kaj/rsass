//! A scope is something that contains variable values.
use std::collections::BTreeMap;
use valueexpression::Value;

pub struct Scope<'a> {
    parent: Option<&'a Scope<'a>>,
    variables: BTreeMap<String, Value>,
}

impl<'a> Scope<'a> {
    pub fn new() -> Self {
        Scope {
            parent: None,
            variables: BTreeMap::new(),
        }
    }
    pub fn sub(parent: &'a Scope) -> Self {
        Scope {
            parent: Some(parent),
            variables: BTreeMap::new(),
        }
    }
    pub fn define(&mut self, name: &str, val: &Value) {
        let val = Value::Literal(self.evaluate(val));
        self.variables.insert(name.to_string(), val);
    }
    pub fn get(&self, name: &str) -> Option<&Value> {
        self.variables
            .get(name)
            .or_else(|| self.parent.and_then(|p| p.get(name)))
    }
    pub fn evaluate(&self, val: &Value) -> String {
        match val {
            &Value::Literal(ref v) => v.clone(),
            &Value::Variable(ref name) => {
                self.get(&name)
                    .map(|n| self.evaluate(n))
                    .unwrap_or_else(|| format!("${}", name))
            }
            &Value::Multi(ref v) => {
                v.iter().map(|v| self.evaluate(v)).collect::<Vec<_>>().join(" ")
            }
        }
    }
}

#[test]
fn test_variable_value() {
    use valueexpression::value_expression;
    let mut scope = Scope::new();
    let red = Value::Literal("#f02a42".to_string());
    scope.define("red", &red);
    let (end, foo) = value_expression(b"$red;").unwrap();
    assert_eq!(b";", end);
    assert_eq!("#f02a42", scope.evaluate(&foo));
}

#[test]
fn test_partial_variable_value() {
    use valueexpression::value_expression;
    let mut scope = Scope::new();
    let red = Value::Literal("#f02a42".to_string());
    scope.define("red", &red);
    let (end, foo) = value_expression(b"solid 1px $red;").unwrap();
    assert_eq!(b";", end);
    assert_eq!("solid 1px #f02a42", scope.evaluate(&foo));
}
