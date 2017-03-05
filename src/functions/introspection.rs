use formalargs::FormalArgs;
use functions::{SassFunction, badarg, badargs};
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
                 Ok(Value::Literal(s.get("value").type_name().into(),
                                   Quotes::None))
             }));
    f.insert("unit",
             func!((value), |s| {
        let v = match &s.get("value") {
            &Value::Numeric(_, ref unit, _) => format!("{}", unit),
            _ => "".into(),
        };
        Ok(Value::Literal(v, Quotes::Double))
    }));
    f.insert("comparable",
             func!((number1, number2),
                   |s| match (&s.get("number1"), &s.get("number2")) {
                       (&Value::Numeric(_, ref u1, _),
                        &Value::Numeric(_, ref u2, _)) => {
                           // TODO e.g. cm and mm are comparable too!
                           Ok(Value::bool(u1 == u2))
                       }
                       (v1, v2) => {
                           Err(badargs(&["number", "number"], &[v1, v2]))
                       }
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
