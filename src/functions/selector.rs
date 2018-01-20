use super::SassFunction;
use css::Value;
use parser::selectors::{selector, selectors};
use selectors::{Selector, Selectors};
use std::collections::BTreeMap;
use value::Quotes;

pub fn register(f: &mut BTreeMap<&'static str, SassFunction>) {
    def_va!(f, selector_nest(selectors), |s| match s.get("selectors") {
        Value::List(v, _, _) => Ok(Value::Literal(
            format!(
                "{}",
                v.into_iter()
                    .map(parse_selectors)
                    .fold(Selectors::root(), |b, e| e.inside(Some(&b))),
            ),
            Quotes::None,
        )),
        v => Ok(Value::Literal(
            format!("{}", parse_selectors(v)),
            Quotes::None,
        )),
    });
    def_va!(f, selector_append(selectors), |s| {
        match s.get("selectors") {
            Value::List(v, _, _) => Ok(Value::Literal(
                format!(
                    "{}",
                    v.into_iter().map(parse_selectors,).fold(
                        Selectors::root(),
                        |base, ext| Selectors(
                            base.0
                                .into_iter()
                                .flat_map(|b| ext.0.iter().map(move |e| {
                                    parse_selector(&format!("{}{}", b, e))
                                }))
                                .collect()
                        ),
                    ),
                ),
                Quotes::None,
            )),
            v => Ok(Value::Literal(
                format!("{}", parse_selectors(v)),
                Quotes::None,
            )),
        }
    });
}

// Note, this helper should probably handle errors and return a Result.
fn parse_selectors(v: Value) -> Selectors {
    let s = format!("{}", v.unquote());
    let (rest, result) = selectors(s.as_bytes()).unwrap();
    assert!(rest == b"");
    result
}

// Note, this helper should probably handle errors and return a Result.
fn parse_selector(s: &str) -> Selector {
    let (rest, result) = selector(s.as_bytes()).unwrap();
    assert!(rest == b"");
    result
}
