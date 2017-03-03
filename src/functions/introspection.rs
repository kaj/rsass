use formalargs::FormalArgs;
use functions::{SassFunction, badarg};
use std::collections::BTreeMap;
use valueexpression::Value;

pub fn register(f: &mut BTreeMap<&'static str, SassFunction>) {
    f.insert("variable_exists",
             func!((name), |s| match &s.get("name") {
                 &Value::Literal(ref v, _) => {
                     Ok(Value::bool(s.get(v) != Value::Null))
                 }
                 v => Err(badarg("string", v)),
             }));
}

#[cfg(test)]
mod test {
    use super::super::super::variablescope::test::do_evaluate;

    #[test]
    fn variable_exists() {
        assert_eq!("true", do_evaluate(&[("x", "17")], b"variable-exists(x);"))
    }

    #[test]
    fn variable_exists_not() {
        assert_eq!("false", do_evaluate(&[], b"variable-exists(x);"))
    }
}
