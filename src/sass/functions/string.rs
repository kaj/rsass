use super::{get_integer, get_string, Error, FunctionMap};
use crate::css::{CssString, Value};
use crate::Scope;
use lazy_static::lazy_static;
use std::cmp::min;
use std::sync::Mutex;

pub fn create_module() -> Scope {
    let mut f = Scope::builtin_module("sass:string");
    def!(f, quote(string), |s| {
        Ok(get_string(s, "string")?.quote().into())
    });
    def!(f, index(string, substring), |s| {
        let string = get_string(s, "string")?;
        Ok(string
            .value()
            .find(get_string(s, "substring")?.value())
            .map(|i| Value::scalar(1 + string.value()[0..i].chars().count()))
            .unwrap_or(Value::Null))
    });
    def!(f, insert(string, insert, index), |s| {
        let string = get_string(s, "string")?;
        let insert = get_string(s, "insert")?;
        let index = get_integer(s, name!(index))?;
        let index = if index.is_negative() {
            let len = string.value().chars().count();
            len.saturating_sub(index.abs() as usize - 1)
        } else {
            (index as usize).saturating_sub(1)
        };
        let mut s = string.value().chars();
        let mut join = s.by_ref().take(index).collect::<String>();
        join.push_str(insert.value());
        join.extend(s);
        Ok(CssString::new(join, string.quotes()).into())
    });
    def!(f, length(string), |s| {
        let v = get_string(s, "string")?;
        Ok(Value::scalar(v.value().chars().count()))
    });
    def!(f, slice(string, start_at, end_at = b"-1"), |s| {
        let string = get_string(s, "string")?;
        let st = string.value();
        let len = st.chars().count();
        let start_at = get_integer(s, name!(start_at))?;
        let start_at = if start_at.is_negative() {
            len.saturating_sub(start_at.abs() as usize)
        } else if start_at.is_positive() {
            min(start_at as usize - 1, len)
        } else {
            0
        };
        let end_at = get_integer(s, name!(end_at))?;
        let end_at = if end_at.is_negative() {
            len.saturating_sub(end_at.abs() as usize - 1)
        } else {
            end_at as usize
        };
        if start_at <= end_at {
            let part =
                st.chars().skip(start_at).take(end_at - start_at).collect();
            Ok(CssString::new(part, string.quotes()).into())
        } else {
            Err(Error::S(format!("Bad indexes: {}..{}", start_at, end_at)))
        }
    });
    def!(f, to_upper_case(string), |s| {
        let v = get_string(s, "string")?;
        Ok(CssString::new(v.value().to_ascii_uppercase(), v.quotes()).into())
    });
    def!(f, to_upper_case_unicode(string), |s| {
        let v = get_string(s, "string")?;
        Ok(CssString::new(v.value().to_uppercase(), v.quotes()).into())
    });
    def!(f, to_lower_case(string), |s| {
        let v = get_string(s, "string")?;
        Ok(CssString::new(v.value().to_ascii_lowercase(), v.quotes()).into())
    });
    def!(f, to_lower_case_unicode(string), |s| {
        let v = get_string(s, "string")?;
        Ok(CssString::new(v.value().to_lowercase(), v.quotes()).into())
    });
    def!(f, unique_id(), |_s| {
        lazy_static! {
            static ref CALL_ID: Mutex<u64> =
                Mutex::new(u64::from(std::process::id()) * 0xa01);
        };
        let v = {
            let mut v = CALL_ID.lock().unwrap();
            *v += 1;
            *v
        };
        Ok(format!("x{:x}", v).into())
    });
    def!(f, unquote(string), |s| {
        Ok(get_string(s, "string")?.unquote().into())
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
        global.insert(gname.clone(), m.get_lfunction(lname));
    }
    // And special one that isn't part of the string module
    def!(global, url(string = b"null"), |s| {
        let string = s.get("string")?;
        Ok(format!("url({})", string.format(Default::default())).into())
    });
}
