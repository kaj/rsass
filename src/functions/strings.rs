use super::{Error, SassFunction};
use crate::css::Value;
use crate::value::{Number, Quotes, Unit};
use lazy_static::lazy_static;
use num_rational::Rational;
use std::cmp::max;
use std::collections::BTreeMap;
use std::sync::Mutex;

pub fn register(f: &mut BTreeMap<&'static str, SassFunction>) {
    def!(f, quote(string), |s| {
        let v = match s.get("string")? {
            Value::Literal(v, Quotes::None) => v.replace('\\', "\\\\"),
            Value::Literal(v, _) => v,
            v => format!("{}", v.format(Default::default())),
        };
        if v.contains('"') && !v.contains('\'') {
            Ok(Value::Literal(v, Quotes::Single))
        } else {
            Ok(Value::Literal(v, Quotes::Double))
        }
    });
    def!(f, unquote(string), |s| match s.get("string")? {
        Value::Literal(v, Quotes::None) =>
            Ok(Value::Literal(v, Quotes::None)),
        Value::Literal(v, _) =>
            Ok(Value::Literal(v.replace("\\\\", "\\"), Quotes::None)),
        v => {
            dep_warn!(
                "Passing {}, a non-string value, to unquote()",
                v.format(Default::default())
            );
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
            let index = index.value.to_integer();
            let i = if index.is_negative() {
                let len = s.chars().count() as isize;
                max(len + 1 + index, 0) as usize
            } else if index.is_positive() {
                index as usize - 1
            } else {
                0
            };
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
    def!(f, str_slice(string, start_at, end_at = b"-1"), |s| match (
        s.get("string")?,
        s.get("start_at")?,
        s.get("end_at")?
    ) {
        (
            Value::Literal(s, q),
            Value::Numeric(start_at, Unit::None, ..),
            Value::Numeric(end_at, Unit::None, ..),
        ) => {
            let start_at = index_to_rust(start_at.value, &s)?;
            let end_at = index_to_rust_end(end_at.value, &s)?;
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
    def!(f, url(string), |s| {
        Ok(Value::Literal(
            format!("url({})", s.get("string")?.format(Default::default())),
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
fn index_to_rust(index: Rational, s: &str) -> Result<usize, Error> {
    let index = require_integer(index)?;
    let len = s.chars().count();
    Ok(if index.is_negative() {
        let i = index.abs() as usize;
        if i <= len {
            len - i
        } else {
            0
        }
    } else if index.is_positive() {
        let i = index as usize - 1;
        if i <= len {
            i
        } else {
            len
        }
    } else {
        0
    })
}

/// Convert index from sass (rational number, first is one) to rust
/// (usize, first is zero).  Sass values might be negative, then -1 is
/// the last char in the string.
fn index_to_rust_end(index: Rational, s: &str) -> Result<usize, Error> {
    let index = require_integer(index)?;
    Ok(if index.is_negative() {
        let len = s.chars().count();
        let i = index.abs() as usize - 1;
        if i <= len {
            len - i
        } else {
            0
        }
    } else if index.is_positive() {
        index as usize
    } else {
        0
    })
}

fn require_integer(value: Rational) -> Result<isize, Error> {
    if value.is_integer() {
        Ok(value.to_integer())
    } else {
        Err(Error::S(format!("{} is not an int", value)))
    }
}
