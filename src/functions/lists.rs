use functions::{SassFunction, badarg};
use std::collections::BTreeMap;
use valueexpression::Value;

pub fn register(f: &mut BTreeMap<&'static str, SassFunction>) {
    def!(f, length(list), |s| match s.get("list") {
        Value::MultiComma(v) => Ok(Value::scalar(v.len() as isize)),
        Value::MultiSpace(v) => Ok(Value::scalar(v.len() as isize)),
        v => Err(badarg("list", &v)),
    });
    def!(f, nth(list, n), |s| {
        let n = match s.get("n") {
            Value::Numeric(val, _, _) if val.denom() == &1 => val.to_integer(),
            x => return Err(badarg("integer", &x)),
        };
        let list = match s.get("list") {
            Value::MultiComma(v) => v,
            Value::MultiSpace(v) => v,
            v => return Err(badarg("list", &v)),
        };
        Ok(list[n as usize - 1].clone())
    });
    def!(f, index(list, value), |s| {
        let v = match s.get("list") {
            Value::MultiComma(v) => v,
            Value::MultiSpace(v) => v,
            v => return Err(badarg("list", &v)),
        };
        let value = s.get("value");
        for (i, v) in v.iter().enumerate() {
            if v == &value {
                return Ok(Value::scalar(i as isize + 1));
            }
        }
        Ok(Value::Null)
    });
}
