use super::{check, get_checked, is_not, Error, FunctionMap};
use crate::css::parser::selectors::{selector, selector_part, selectors};
use crate::css::{Selector, SelectorPart, Selectors, Value};
use crate::parser::input_span;
use crate::sass::Name;
use crate::value::ListSeparator;
use crate::{ParseError, Scope};

pub fn create_module() -> Scope {
    let mut f = Scope::builtin_module("sass:selector");
    // TODO: is_superselector
    def_va!(f, append(selectors), |s| {
        get_selectors(s, name!(selectors))?
            .into_iter()
            .try_fold(Selectors::root(), |base, ext| {
                let ext = no_backref(ext)?;
                Ok(Selectors::new(
                    base.s
                        .into_iter()
                        .flat_map(|b| {
                            ext.s.iter().map(move |e| {
                                if e.0[0].is_operator()
                                    || e.0[0].is_wildcard()
                                {
                                    return Err(Error::error(&format!(
                                        "Can't append {} to {}.",
                                        e, b
                                    )));
                                }
                                parse_selector(&format!("{}{}", b, e))
                            })
                        })
                        .collect::<Result<_, _>>()?,
                ))
            })
            .map(|v| v.to_value())
    });
    // TODO: extend
    def_va!(f, nest(selectors), |s| {
        let mut v = get_selectors(s, name!(selectors))?.into_iter();
        let first = no_backref(v.next().unwrap())?;
        Ok(v.fold(first, |b, e| e.inside(&b)).to_value())
    });
    def!(f, parse(selector), |s| {
        Ok(get_checked(s, name!(selector), parse_selectors_x)?.to_value())
    });
    // TODO: replace, unify, simple_selectors
    f
}

fn get_selectors(s: &Scope, name: Name) -> Result<Vec<Selectors>, Error> {
    get_checked(s, name, check::va_list_nonempty)?
        .into_iter()
        .map(parse_selectors_e)
        .collect()
}

fn no_backref(sel: Selectors) -> Result<Selectors, ParseError> {
    if sel.has_backref() {
        let sel = sel.to_string();
        Err(ParseError::mock(
            "Error: Parent selectors aren\'t allowed here.".into(),
            input_span(sel.as_bytes()),
        ))
    } else {
        Ok(sel)
    }
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
        global.insert(gname.clone(), m.get_lfunction(lname));
    }
}

fn parse_selectors_e(v: Value) -> Result<Selectors, Error> {
    parse_selectors(v.clone()).map_err(|e| e.context(&v))
}
fn parse_selectors_x(v: Value) -> Result<Selectors, String> {
    parse_selectors(v.clone())
        .map_err(|e| e.context_s(&v))
        .and_then(|s| {
            no_backref(s).map_err(|e| {
                e.to_string().trim_start_matches("Error: ").to_string()
            })
        })
}

fn parse_selectors(v: Value) -> Result<Selectors, BadSelector> {
    match v {
        Value::List(vv, s, _) => match s {
            Some(ListSeparator::Comma) => {
                let vv = vv
                    .iter()
                    .map(check_selector)
                    .collect::<Result<_, _>>()?;
                Ok(Selectors::new(vv))
            }
            Some(ListSeparator::Space) => {
                let (mut outer, last) = vv.iter().fold(
                    Result::<_, BadSelector>::Ok((vec![], vec![])),
                    |a, v: &Value| {
                        let (mut outer, mut a) = a?;
                        if let Ok(ref mut s) = check_selector_str(v) {
                            push_descendant(&mut a, s)
                        } else {
                            let mut s = parse_selectors_str(v.clone())?;
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
            _ => Err(BadSelector::Value),
        },
        Value::Literal(s) => {
            if s.value().is_empty() {
                Ok(Selectors::root())
            } else {
                let span = input_span(s.value().as_bytes());
                Ok(ParseError::check(selectors(span))?)
            }
        }
        _ => Err(BadSelector::Value),
    }
}
fn parse_selectors_str(v: Value) -> Result<Selectors, BadSelector> {
    match v {
        Value::Literal(s) => {
            if s.value().is_empty() {
                Ok(Selectors::root())
            } else {
                let span = input_span(s.value().as_bytes());
                Ok(ParseError::check(selectors(span))?)
            }
        }
        _ => Err(BadSelector::Value),
    }
}

fn check_selector(v: &Value) -> Result<Selector, BadSelector> {
    match v {
        Value::List(list, sep, _) => {
            if sep == &None || sep == &Some(ListSeparator::Space) {
                list_to_selector(list)
            } else {
                Err(BadSelector::Value)
            }
        }
        Value::Literal(s) => {
            if s.value().is_empty() {
                Ok(Selector::root())
            } else {
                let span = input_span(s.value().as_bytes());
                Ok(ParseError::check(selector(span))?)
            }
        }
        _ => Err(BadSelector::Value),
    }
}
fn check_selector_str(v: &Value) -> Result<Selector, BadSelector> {
    match v {
        Value::Literal(s) => {
            if s.value().is_empty() {
                Ok(Selector::root())
            } else {
                let span = input_span(s.value().as_bytes());
                Ok(ParseError::check(selector(span))?)
            }
        }
        _ => Err(BadSelector::Value),
    }
}
fn list_to_selector(list: &[Value]) -> Result<Selector, BadSelector> {
    Ok(Selector(list.iter().fold(
        Result::<_, BadSelector>::Ok(vec![]),
        |a, v| {
            let mut a = a?;
            if !a.is_empty() {
                a.push(SelectorPart::Descendant)
            }
            a.push(check_selector_part(v)?);
            Ok(a)
        },
    )?))
}

fn push_descendant(to: &mut Vec<SelectorPart>, from: &mut Selector) {
    if !to.is_empty() {
        to.push(SelectorPart::Descendant)
    }
    to.append(&mut from.0);
}

fn check_selector_part(v: &Value) -> Result<SelectorPart, BadSelector> {
    match v {
        Value::Literal(s) => Ok(ParseError::check(selector_part(
            input_span(s.value().as_bytes()),
        ))?),
        _ => Err(BadSelector::Value),
    }
}

fn parse_selector(s: &str) -> Result<Selector, Error> {
    Ok(ParseError::check(selector(input_span(s.as_bytes())))?)
}

#[derive(Debug)]
enum BadSelector {
    Value,
    Parse(ParseError),
}

impl BadSelector {
    fn context(self, v: &Value) -> Error {
        match self {
            BadSelector::Value => Error::error(bad_selector(v)),
            BadSelector::Parse(e) => e.into(),
        }
    }
    fn context_s(self, v: &Value) -> String {
        match self {
            BadSelector::Value => bad_selector(v),
            BadSelector::Parse(e) => e.to_string(),
        }
    }
}
impl From<ParseError> for BadSelector {
    fn from(e: ParseError) -> Self {
        BadSelector::Parse(e)
    }
}

fn bad_selector(v: &Value) -> String {
    is_not(
        v,
        "a valid selector: it must be a string,\
         \na list of strings, or a list of lists of strings",
    )
}
