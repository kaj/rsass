use formalargs::FormalArgs;
use functions::SassFunction;
use num_rational::Rational;
use std::collections::BTreeMap;
use valueexpression::{Value, Unit};

pub fn register(f: &mut BTreeMap<&'static str, SassFunction>) {
    f.insert("quote",
             func!((contents), |s| {
                 match s.get("contents") {
                     Value::Literal(v, _) => Value::Literal(v, true),
                     v => Value::Literal(format!("{}", v), true),
                 }
             }));
    f.insert("unquote",
             func!((contents), |s| {
                 match s.get("contents") {
                     Value::Literal(v, _) => Value::Literal(v, false),
                     v => v,
                 }
             }));
    f.insert("str_index",
             func!((string, substring), |s| {
        match (s.get("string"), s.get("substring")) {
            (Value::Literal(s, _), Value::Literal(sub, _)) => {
                match s.find(&sub) {
                    Some(o) => {
                        let n = s[0..o].chars().count() as isize;
                        Value::Numeric(Rational::from_integer(1 + n),
                                       Unit::None,
                                       true)
                    }
                    None => Value::Null,
                }
            }
            _ => panic!("Parameter of wrong type"),
        }
    }));
    f.insert("to_upper_case",
             func!((string), |s| {
        match s.get("string") {
            Value::Literal(v, q) => Value::Literal(v.to_uppercase(), q),
            v => v,
        }
    }));
    f.insert("to_lower_case",
             func!((string), |s| {
        match s.get("string") {
            Value::Literal(v, q) => Value::Literal(v.to_lowercase(), q),
            v => v,
        }
    }));
}
