use super::SassFunction;
use css::Value;
use parser::selectors::selectors;
use selectors::Selectors;
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
}

// Note, this helper should probably handle errors and return a Result.
fn parse_selectors(v: Value) -> Selectors {
    selectors(format!("{}", v.unquote()).as_bytes()).unwrap().1
}
