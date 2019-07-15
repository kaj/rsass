use super::{Error, SassFunction};
use crate::css::Value;
use crate::value::{Number, Quotes, Unit};
use lazy_static::lazy_static;
use num_traits::Signed;
use std::collections::BTreeMap;
use std::sync::Mutex;

pub fn register(f: &mut BTreeMap<&'static str, SassFunction>) {
    def!(f, quote(contents), |s| {
        let v = match s.get("contents")? {
            Value::Literal(v, _) => v,
            v => format!("{}", v),
        };
        if v.contains('"') && !v.contains('\'') {
            Ok(Value::Literal(v, Quotes::Single))
        } else {
            Ok(Value::Literal(v, Quotes::Double))
        }
    });
    def!(f, unquote(contents), |s| match s.get("contents")? {
        Value::Literal(v, _) => Ok(Value::Literal(v, Quotes::None)),
        v => {
            dep_warn!("Passing {}, a non-string value, to unquote()", v);
            Ok(v)
        }
    });
    def!(f, str_insert(string, insert, index), |s| match (
        s.get("string")?,
        s.get("insert")?,
        s.get("index")?,
    ) {
        (
            Value::Literal(s, q),
            Value::Literal(insert, _),
            Value::Numeric(index, Unit::None, ..),
        ) => {
            let i = index_to_rust(&index, &s);
            let mut s = s.chars();
            Ok(Value::Literal(
                format!(
                    "{}{}{}",
                    s.by_ref().take(i).collect::<String>(),
                    insert,
                    s.collect::<String>()
                ),
                q,
            ))
        }
        (s, i, v) => Err(Error::badargs(
            &["string", "string", "number"],
            &[&s, &i, &v],
        )),
    });
    def!(f, str_slice(string, start_at, end_at = b"-1;"), |s| match (
        s.get("string")?,
        s.get("start_at")?,
        s.get("end_at")?
    ) {
        (
            Value::Literal(s, q),
            Value::Numeric(start_at, Unit::None, ..),
            Value::Numeric(end_at, Unit::None, ..),
        ) => {
            let start_at = index_to_rust(&start_at, &s);
            let end_at = index_to_rust(&end_at, &s) + 1;
            let c = s.chars();
            if start_at <= end_at {
                Ok(Value::Literal(
                    c.skip(start_at)
                        .take(end_at - start_at)
                        .collect::<String>(),
                    q,
                ))
            } else {
                Err(Error::S(format!(
                    "Bad indexes: {}..{}",
                    start_at, end_at
                )))
            }
        }
        (v, s, e) => Err(Error::badargs(
            &["string", "number", "number"],
            &[&v, &s, &e]
        )),
    });
    def!(f, str_length(string), |s| match &s.get("string")? {
        &Value::Literal(ref v, _) => Ok(intvalue(v.chars().count())),
        v => Err(Error::badarg("string", v)),
    });
    def!(f, str_index(string, substring), |s| match (
        s.get("string")?,
        s.get("substring")?,
    ) {
        (Value::Literal(s, _), Value::Literal(sub, _)) => {
            Ok(match s.find(&sub) {
                Some(o) => intvalue(1 + s[0..o].chars().count()),
                None => Value::Null,
            })
        }
        (full, sub) => {
            Err(Error::badargs(&["string", "string"], &[&full, &sub]))
        }
    });
    def!(f, to_upper_case(string), |s| match s.get("string")? {
        Value::Literal(v, q) => Ok(Value::Literal(v.to_uppercase(), q)),
        v => Ok(v),
    });
    def!(f, to_lower_case(string), |s| match s.get("string")? {
        Value::Literal(v, q) => Ok(Value::Literal(v.to_lowercase(), q)),
        v => Ok(v),
    });
    def!(f, unique_id(), |_s| {
        lazy_static! {
            static ref CALL_ID: Mutex<u64> =
                Mutex::new(u64::from(std::process::id()) * 0xa01);
        };
        Ok(Value::Literal(
            format!("x{:x}", {
                let mut v = CALL_ID.lock().unwrap();
                *v += 1;
                *v
            }),
            Quotes::None,
        ))
    });
}

fn intvalue(n: usize) -> Value {
    Value::Numeric(Number::from(n as isize), Unit::None, true)
}

/// Convert index from sass (rational number, first is one) to rust
/// (usize, first is zero).  Sass values might be negative, then -1 is
/// the last char in the string.
fn index_to_rust(index: &Number, s: &str) -> usize {
    if index.value.is_negative() {
        let l = s.chars().count();
        let i = index.to_integer().abs() as usize;
        if l > i {
            l - i + 1
        } else {
            0
        }
    } else if index.value.is_positive() {
        index.to_integer() as usize - 1
    } else {
        0
    }
}
