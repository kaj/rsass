use formalargs::FormalArgs;
use functions::SassFunction;
use num_rational::Rational;
use num_traits::Signed;
use std::collections::BTreeMap;
use valueexpression::{Quotes, Value, Unit};

pub fn register(f: &mut BTreeMap<&'static str, SassFunction>) {
    f.insert("quote",
             func!((contents), |s| {
                 match s.get("contents") {
                     Value::Literal(v, _) => Value::Literal(v, Quotes::Double),
                     v => Value::Literal(format!("{}", v), Quotes::Double),
                 }
             }));
    f.insert("unquote",
             func!((contents), |s| {
                 match s.get("contents") {
                     Value::Literal(v, _) => Value::Literal(v, Quotes::None),
                     v => v,
                 }
             }));
    f.insert("str_insert",
             func!((string, insert, index), |s| {
        match (s.get("string"), s.get("insert"), s.get("index")) {
            (Value::Literal(s, q),
             Value::Literal(insert, _),
             Value::Numeric(index, Unit::None, _)) => {
                let i = if index.is_negative() {
                    let l = s.chars().count();
                    let i = index.to_integer().abs() as usize;
                    if l > i { l - i + 1 } else { 0 }
                } else if index.is_positive() {
                    index.to_integer() as usize - 1
                } else {
                    0
                };
                let c = s.chars();
                Value::Literal(format!("{}{}{}",
                                       c.clone().take(i).collect::<String>(),
                                       insert,
                                       c.skip(i).collect::<String>()),
                               q)
            }
            _ => panic!("Parameter of wrong type"),
        }
    }));
    f.insert("str_length",
             func!((string), |s| {
        match s.get("string") {
            Value::Literal(v, _) => {
                let n = v.chars().count() as isize;
                Value::Numeric(Rational::from_integer(n), Unit::None, true)
            }
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
