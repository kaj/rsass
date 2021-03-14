use super::{get_integer, get_string, Error, FunctionMap};
use crate::css::Value;
use crate::value::Quotes;
use crate::Scope;
use lazy_static::lazy_static;
use std::cmp::max;
use std::sync::Mutex;

pub fn create_module() -> Scope {
    let mut f = Scope::builtin_module("sass:string");
    def!(f, quote(string), |s| {
        let v = match get_string(s, "string")? {
            (v, Quotes::None) => v.replace('\\', "\\\\"),
            (v, _) => v,
        };
        if v.contains('"') && !v.contains('\'') {
            Ok(Value::Literal(v, Quotes::Single))
        } else {
            Ok(Value::Literal(v, Quotes::Double))
        }
    });
    def!(f, index(string, substring), |s| {
        let (string, _) = get_string(s, "string")?;
        let (sub, _) = get_string(s, "substring")?;
        Ok(match string.find(&sub) {
            Some(o) => Value::scalar(1 + string[0..o].chars().count()),
            None => Value::Null,
        })
    });
    def!(f, insert(string, insert, index), |s| {
        let (string, q) = get_string(s, "string")?;
        let (insert, _) = get_string(s, "insert")?;
        let index = get_integer(s, name!(index))?;

        let i = if index.is_negative() {
            let len = string.chars().count() as i64;
            max(len + 1 + index, 0) as usize
        } else if index.is_positive() {
            index as usize - 1
        } else {
            0
        };
        let mut s = string.chars();
        Ok(Value::Literal(
            format!(
                "{}{}{}",
                s.by_ref().take(i).collect::<String>(),
                insert,
                s.collect::<String>()
            ),
            q,
        ))
    });
    def!(f, length(string), |s| {
        let (v, _q) = get_string(s, "string")?;
        Ok(Value::scalar(v.chars().count()))
    });
    def!(f, slice(string, start_at, end_at = b"-1"), |s| {
        let (st, q) = get_string(s, "string")?;
        let start_at = index_to_rust(get_integer(s, name!(start_at))?, &st);
        let end_at = index_to_rust_end(get_integer(s, name!(end_at))?, &st);
        let c = st.chars();
        if start_at <= end_at {
            Ok(Value::Literal(
                c.skip(start_at).take(end_at - start_at).collect::<String>(),
                q,
            ))
        } else {
            Err(Error::S(format!("Bad indexes: {}..{}", start_at, end_at)))
        }
    });
    def!(f, to_upper_case(string), |s| {
        let (v, q) = get_string(s, "string")?;
        Ok(Value::Literal(v.to_ascii_uppercase(), q))
    });
    def!(f, to_upper_case_unicode(string), |s| {
        let (v, q) = get_string(s, "string")?;
        Ok(Value::Literal(v.to_uppercase(), q))
    });
    def!(f, to_lower_case(string), |s| {
        let (v, q) = get_string(s, "string")?;
        Ok(Value::Literal(v.to_ascii_lowercase(), q))
    });
    def!(f, to_lower_case_unicode(string), |s| {
        let (v, q) = get_string(s, "string")?;
        Ok(Value::Literal(v.to_lowercase(), q))
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
    def!(f, unquote(string), |s| match get_string(s, "string")? {
        (v, Quotes::None) => Ok(Value::Literal(v, Quotes::None)),
        (v, _) => Ok(Value::Literal(v.replace("\\\\", "\\"), Quotes::None)),
    });

    f
}

pub fn expose(m: &Scope, global: &mut FunctionMap) {
    for (gname, lname) in &[
        (name!(quote), name!(quote)),
        (name!(str_index), name!(index)),
        (name!(str_insert), name!(insert)),
        (name!(str_length), name!(length)),
        (name!(str_slice), name!(slice)),
        (name!(to_upper_case), name!(to_upper_case)),
        (name!(to_lower_case), name!(to_lower_case)),
        (name!(unique_id), name!(unique_id)),
        (name!(unquote), name!(unquote)),
    ] {
        global.insert(gname.clone(), m.get_function(&lname).unwrap().clone());
    }
    // And special one that isn't part of the string module
    def!(global, url(string = b"null"), |s| {
        Ok(Value::Literal(
            format!("url({})", s.get("string")?.format(Default::default())),
            Quotes::None,
        ))
    });
}

/// Convert index from sass (rational number, first is one) to rust
/// (usize, first is zero).  Sass values might be negative, then -1 is
/// the last char in the string.
fn index_to_rust(index: i64, s: &str) -> usize {
    let len = s.chars().count();
    if index.is_negative() {
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
    }
}

/// Convert index from sass (rational number, first is one) to rust
/// (usize, first is zero).  Sass values might be negative, then -1 is
/// the last char in the string.
fn index_to_rust_end(index: i64, s: &str) -> usize {
    if index.is_negative() {
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
    }
}
