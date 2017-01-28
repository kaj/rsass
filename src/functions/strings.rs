use formalargs::FormalArgs;
use functions::{SassFunction, badarg, badargs};
use num_rational::Rational;
use num_traits::Signed;
use std::collections::BTreeMap;
use valueexpression::{Quotes, Value, Unit};

pub fn register(f: &mut BTreeMap<&'static str, SassFunction>) {
    f.insert("quote",
             func!((contents), |s| {
                 Ok(match s.get("contents") {
                     Value::Literal(v, _) => Value::Literal(v, Quotes::Double),
                     v => Value::Literal(format!("{}", v), Quotes::Double),
                 })
             }));
    f.insert("unquote",
             func!((contents), |s| {
                 Ok(match s.get("contents") {
                     Value::Literal(v, _) => Value::Literal(v, Quotes::None),
                     v => v,
                 })
             }));
    f.insert("str_insert",
             func!((string, insert, index), |s| {
        match (s.get("string"), s.get("insert"), s.get("index")) {
            (Value::Literal(s, q),
             Value::Literal(insert, _),
             Value::Numeric(index, Unit::None, _)) => {
                let i = index_to_rust(index, &s);
                let c = s.chars();
                Ok(Value::Literal(format!("{}{}{}",
                                          c.clone().take(i).collect::<String>(),
                                          insert,
                                          c.skip(i).collect::<String>()),
                                  q))
            }
            (s, i, v) => Err(badargs(&["string", "string", "number"],
                                     &[&s, &i, &v])),
        }
    }));
    f.insert("str_slice",
             func!((string, start_at, end_at = b"-1;"), |s| {
        match (s.get("string"), s.get("start_at"), s.get("end_at")) {
            (Value::Literal(s, q),
             Value::Numeric(start_at, Unit::None, _),
             Value::Numeric(end_at, Unit::None, _)) => {
                let start_at = index_to_rust(start_at, &s);
                let end_at = index_to_rust(end_at, &s);
                let c = s.chars();
                Ok(Value::Literal(c.skip(start_at)
                                      .take(end_at + 1 - start_at)
                                      .collect::<String>(),
                                  q))
            }
            (v, s, e) => {
                Err(badargs(&["string", "number", "number"], &[&v, &s, &e]))
            }
        }
    }));
    f.insert("str_length",
             func!((string), |s| {
        match &s.get("string") {
            &Value::Literal(ref v, _) => {
                let n = v.chars().count() as isize;
                Ok(Value::Numeric(Rational::from_integer(n), Unit::None, true))
            }
            v => Err(badarg("string", v)),
        }
    }));
    f.insert("str_index",
             func!((string, substring), |s| {
        match (s.get("string"), s.get("substring")) {
            (Value::Literal(s, _), Value::Literal(sub, _)) => {
                Ok(match s.find(&sub) {
                    Some(o) => {
                        let n = s[0..o].chars().count() as isize;
                        Value::Numeric(Rational::from_integer(1 + n),
                                       Unit::None,
                                       true)
                    }
                    None => Value::Null,
                })
            }
            (full, sub) => Err(badargs(&["string", "string"], &[&full, &sub])),
        }
    }));
    f.insert("to_upper_case",
             func!((string), |s| {
        Ok(match s.get("string") {
            Value::Literal(v, q) => Value::Literal(v.to_uppercase(), q),
            v => v,
        })
    }));
    f.insert("to_lower_case",
             func!((string), |s| {
        Ok(match s.get("string") {
            Value::Literal(v, q) => Value::Literal(v.to_lowercase(), q),
            v => v,
        })
    }));
}

/// Convert index from sass (rational number, first is one) to rust
/// (usize, first is zero).  Sass values might be negative, then -1 is
/// the last char in the string.
fn index_to_rust(index: Rational, s: &str) -> usize {
    if index.is_negative() {
        let l = s.chars().count();
        let i = index.to_integer().abs() as usize;
        if l > i { l - i + 1 } else { 0 }
    } else if index.is_positive() {
        index.to_integer() as usize - 1
    } else {
        0
    }
}
