use super::{Error, FunctionMap};
use crate::css::Value;
use crate::parser::input_span;
use crate::parser::selectors::{selector, selector_part, selectors};
use crate::selectors::{Selector, SelectorPart, Selectors};
use crate::value::{ListSeparator, Quotes};
use crate::{ParseError, Scope};

pub fn create_module() -> Scope {
    let mut f = Scope::builtin_module("sass:selector");
    // TODO: is_superselector
    def_va!(f, append(selectors), |s| match s.get("selectors")? {
        Value::List(v, _, _) => Ok(v
            .into_iter()
            .map(parse_selectors)
            .try_fold(Selectors::root(), |base, ext| ext.and_then(|ext| Ok(
                Selectors::new(
                    base.s
                        .into_iter()
                        .flat_map(|b| {
                            ext.s.iter().map(move |e| {
                                parse_selector(&format!("{}{}", b, e))
                            })
                        })
                        .collect::<Result<_, _>>()?
                )
            )),)?
            .to_value()),
        v => Ok(parse_selectors(v)?.to_value()),
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

pub fn expose(m: &Scope, global: &mut FunctionMap) {
    for (gname, lname) in &[
        // - - - Mixins - - -
        //(name!(is_superselector), name!(is_superselector)),
        (name!(selector_append), name!(append)),
        //(name!(selector_extend), name!(extend)),
        (name!(selector_nest), name!(nest)),
        (name!(selector_parse), name!(parse)),
        //(name!(selector_replace), name!(replace)),
        //(name!(selector_unify), name!(unify)),
        //(name!(simple_selectors), name!(simple_selectors)),
    ] {
        global.insert(gname.clone(), m.get_function(&lname).unwrap().clone());
    }
}

fn parse_selectors(v: Value) -> Result<Selectors, Error> {
    match v {
        Value::List(v, s, _) => {
            if s != Some(ListSeparator::Space) {
                let v =
                    v.iter().map(check_selector).collect::<Result<_, _>>()?;
                Ok(Selectors::new(v))
            } else {
                let (mut outer, last) = v.iter().fold(
                    Result::<_, Error>::Ok((vec![], vec![])),
                    |a, v: &Value| {
                        let (mut outer, mut a) = a?;
                        if let Ok(ref mut s) = check_selector(v) {
                            push_descendant(&mut a, s)
                        } else {
                            let mut s = parse_selectors(v.clone())?;
                            if let Some(f) = s.s.first_mut() {
                                push_descendant(&mut a, f);
                                std::mem::swap(&mut a, &mut f.0);
                            }
                            if let Some(last) = s.s.pop() {
                                a = last.0;
                            }
                            outer.extend(s.s);
                        }
                        Ok((outer, a))
                    },
                )?;
                outer.push(Selector(last));
                Ok(Selectors::new(outer))
            }
        }
        Value::Literal(s, _) => {
            if s.is_empty() {
                Ok(Selectors::root())
            } else {
                Ok(ParseError::check(selectors(input_span(s.as_bytes())))?)
            }
        }
        v => Err(bad_selector(&v)),
    }
}

fn push_descendant(to: &mut Vec<SelectorPart>, from: &mut Selector) {
    if !to.is_empty() {
        to.push(SelectorPart::Descendant)
    }
    to.extend(from.0.drain(..));
}

fn check_selector(v: &Value) -> Result<Selector, Error> {
    match v {
        Value::List(v, _, _) => Ok(Selector(v.iter().fold(
            Result::<_, Error>::Ok(vec![]),
            |a, v| {
                let mut a = a?;
                if !a.is_empty() {
                    a.push(SelectorPart::Descendant)
                }
                a.push(check_selector_part(v)?);
                Ok(a)
            },
        )?)),
        Value::Literal(s, _) => {
            if s.is_empty() {
                Ok(Selector::root())
            } else {
                Ok(ParseError::check(selector(input_span(s.as_bytes())))?)
            }
        }
        v => Err(bad_selector(v)),
    }
}
fn check_selector_part(v: &Value) -> Result<SelectorPart, Error> {
    match v {
        Value::Literal(s, _) => {
            Ok(ParseError::check(selector_part(input_span(s.as_bytes())))?)
        }
        v => Err(bad_selector(v)),
    }
}

fn parse_selector(s: &str) -> Result<Selector, Error> {
    Ok(ParseError::check(selector(input_span(s.as_bytes())))?)
}

fn bad_selector(v: &Value) -> Error {
    Error::bad_value(
        "a valid selector: it must be a string,\
         \na list of strings, or a list of lists of strings",
        v,
    )
}
