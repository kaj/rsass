use formalargs::FormalArgs;
use functions::{SassFunction, badarg};
use std::collections::BTreeMap;
use valueexpression::{Quotes, Value};

pub fn register(f: &mut BTreeMap<&'static str, SassFunction>) {
    f.insert("variable_exists",
             func!((name), |s| match &s.get("name") {
                 &Value::Literal(ref v, _) => {
                     Ok(Value::bool(s.get(v) != Value::Null))
                 }
                 v => Err(badarg("string", v)),
             }));
    f.insert("type_of",
             func!((value), |s| {
        Ok(Value::Literal(s.get("value").type_name().into(), Quotes::None))
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

    #[test]
    fn type_of_number() {
        assert_eq!("number", do_evaluate(&[], b"type_of(100px);"))
    }

    #[test]
    fn type_of_string() {
        assert_eq!("string", do_evaluate(&[], b"type_of(asdf);"))
    }

    #[test]
    fn type_of_color() {
        assert_eq!("color", do_evaluate(&[], b"type_of(#fff);"))
    }

    #[test]
    fn type_of_color_by_name() {
        assert_eq!("color", do_evaluate(&[], b"type_of(red);"))
    }
}
