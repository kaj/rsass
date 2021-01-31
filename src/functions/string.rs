use super::{Error, Module, SassFunction};
use crate::css::Value;
use crate::value::{Number, Quotes, Unit};
use lazy_static::lazy_static;
use std::cmp::max;
use std::sync::Mutex;

pub fn create_module() -> Module {
    let mut f = Module::new();
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
    def!(f, index(string, substring), |s| match (
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
    def!(f, insert(string, insert, index), |s| match (
        s.get("string")?,
        s.get("insert")?,
        s.get("index")?,
    ) {
        (
            Value::Literal(s, q),
            Value::Literal(insert, _),
            index @ Value::Numeric(_, Unit::None, ..),
        ) => {
            let index = index.integer_value()?;
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
    def!(f, length(string), |s| match &s.get("string")? {
        &Value::Literal(ref v, _) => Ok(intvalue(v.chars().count())),
        v => Err(Error::badarg("string", v)),
    });
    def!(f, slice(string, start_at, end_at = b"-1"), |s| match (
        s.get("string")?,
        s.get("start_at")?,
        s.get("end_at")?
    ) {
        (
            Value::Literal(s, q),
            Value::Numeric(start_at, Unit::None, ..),
            Value::Numeric(end_at, Unit::None, ..),
        ) => {
            let start_at = index_to_rust(start_at, &s)?;
            let end_at = index_to_rust_end(end_at, &s)?;
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
    def!(f, to_upper_case(string), |s| match s.get("string")? {
        Value::Literal(v, q) => Ok(Value::Literal(v.to_ascii_uppercase(), q)),
        v => Ok(v),
    });
    def!(
        f,
        to_upper_case_unicode(string),
        |s| match s.get("string")? {
            Value::Literal(v, q) => Ok(Value::Literal(v.to_uppercase(), q)),
            v => Ok(v),
        }
    );
    def!(f, to_lower_case(string), |s| match s.get("string")? {
        Value::Literal(v, q) => Ok(Value::Literal(v.to_ascii_lowercase(), q)),
        v => Ok(v),
    });
    def!(
        f,
        to_lower_case_unicode(string),
        |s| match s.get("string")? {
            Value::Literal(v, q) => Ok(Value::Literal(v.to_lowercase(), q)),
            v => Ok(v),
        }
    );
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

    f
}

pub fn expose(m: &Module, global: &mut Module) {
    for &(gname, lname) in &[
        ("quote", "quote"),
        ("str_index", "index"),
        ("str_insert", "insert"),
        ("str_length", "length"),
        ("str_slice", "slice"),
        ("to_upper_case", "to_upper_case"),
        ("to_lower_case", "to_lower_case"),
        ("unique_id", "unique_id"),
        ("unquote", "unquote"),
    ] {
        global.expose(gname, m, lname);
    }
    // And special one that isn't part of the string module
    def!(global, url(string), |s| {
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
fn index_to_rust(index: Number, s: &str) -> Result<usize, Error> {
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
fn index_to_rust_end(index: Number, s: &str) -> Result<usize, Error> {
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

fn require_integer(value: Number) -> Result<isize, Error> {
    value
        .to_integer()
        .ok_or_else(|| Error::S(format!("{:?} is not an int", value)))
}
