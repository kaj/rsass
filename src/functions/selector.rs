use super::{Error, Module, SassFunction};
use crate::css::Value;
use crate::parser::code_span;
use crate::parser::selectors::{selector, selectors};
use crate::selectors::{Selector, Selectors};
use crate::value::Quotes;
use crate::ParseError;

pub fn create_module() -> Module {
    let mut f = Module::new();
    // TODO: is_superselector
    def_va!(f, append(selectors), |s| match s.get("selectors")? {
        Value::List(v, _, _) => Ok(Value::Literal(
            format!(
                "{}",
                v.into_iter().map(parse_selectors).try_fold(
                    Selectors::root(),
                    |base, ext| ext.and_then(|ext| Ok(Selectors::new(
                        base.s
                            .into_iter()
                            .flat_map(|b| {
                                ext.s.iter().map(move |e| {
                                    parse_selector(&format!("{}{}", b, e))
                                })
                            })
                            .collect::<Result<_, _>>()?
                    ))),
                )?,
            ),
            Quotes::None,
        )),
        v => Ok(Value::Literal(
            format!("{}", parse_selectors(v)?),
            Quotes::None,
        )),
    });
    // TODO: extend
    def_va!(f, nest(selectors), |s| match s.get("selectors")? {
        Value::List(v, _, _) => Ok(Value::Literal(
            format!(
                "{}",
                v.into_iter()
                    .map(parse_selectors)
                    .try_fold(Selectors::root(), |b, e| e
                        .map(|e| e.inside(&b)))?
            ),
            Quotes::None,
        )),
        v => Ok(Value::Literal(
            format!("{}", parse_selectors(v)?),
            Quotes::None,
        )),
    });
    def!(f, parse(selector), |s| {
        Ok(parse_selectors(s.get("selector")?)?.to_value())
    });
    // TODO: replace, unify, simple_selectors
    f
}

pub fn expose(meta: &Module, global: &mut Module) {
    for (gname, lname) in &[
        // - - - Mixins - - -
        //("is_superselector", "is_superselector"),
        ("selector_append", "append"),
        //("selector_extend", "extend"),
        ("selector_nest", "nest"),
        ("selector_parse", "parse"),
        //("selector_replace", "replace"),
        //("selector_unify", "unify"),
        //("simple_selectors", "simple_selectors"),
    ] {
        global.insert_function(gname, meta.get_function(lname).unwrap().clone());
    }
}

fn parse_selectors(v: Value) -> Result<Selectors, Error> {
    let s = format!("{}", v.unquote().format(Default::default()));
    if s.is_empty() {
        Ok(Selectors::root())
    } else {
        // FIXME: Old code allowd a trailing comma here.  Add back or remove?
        Ok(ParseError::check(selectors(code_span(s.as_bytes())))?)
    }
}

fn parse_selector(s: &str) -> Result<Selector, Error> {
    // TODO: Use a span from the value rather than claiming its hardcoded.
    Ok(ParseError::check(selector(code_span(s.as_bytes())))?)
}
