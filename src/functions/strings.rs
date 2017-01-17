use formalargs::FormalArgs;
use functions::SassFunction;
use std::collections::BTreeMap;
use valueexpression::Value;

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
}
