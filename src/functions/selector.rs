use super::SassFunction;
use css::Value;
use nom::types::CompleteByteSlice as Input;
use parser::selectors::{selector, selectors};
use selectors::{Selector, Selectors};
use std::collections::BTreeMap;
use value::Quotes;

pub fn register(f: &mut BTreeMap<&'static str, SassFunction>) {
    def_va!(f, selector_nest(selectors), |s| match s.get("selectors")? {
        Value::List(v, _, _) => Ok(Value::Literal(
            format!(
                "{}",
                v.into_iter()
                    .map(parse_selectors)
                    .fold(Selectors::root(), |b, e| e.inside(&b)),
            ),
            Quotes::None,
        )),
        v => Ok(Value::Literal(
            format!("{}", parse_selectors(v)),
            Quotes::None,
        )),
    });
    def_va!(
        f,
        selector_append(selectors),
        |s| match s.get("selectors")? {
            Value::List(v, _, _) => Ok(Value::Literal(
                format!(
                    "{}",
                    v.into_iter().map(parse_selectors).fold(
                        Selectors::root(),
                        |base, ext| Selectors::new(
                            base.s
                                .into_iter()
                                .flat_map(|b| ext.s.iter().map(move |e| {
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
    );
    def!(f, selector_parse(selector), |s| Ok(parse_selectors(
        s.get("selector")?
    )
    .to_value()));
}

// Note, this helper should probably handle errors and return a Result.
fn parse_selectors(v: Value) -> Selectors {
    let s = format!("{}", v.unquote());
    if s.is_empty() {
        Selectors::root()
    } else {
        let (rest, result) = selectors(Input(s.as_bytes())).unwrap();
        assert!(rest.is_empty() || rest == Input(b","));
        result
    }
}

// Note, this helper should probably handle errors and return a Result.
fn parse_selector(s: &str) -> Selector {
    let (rest, result) = selector(Input(s.as_bytes())).unwrap();
    assert!(rest.is_empty());
    result
}
