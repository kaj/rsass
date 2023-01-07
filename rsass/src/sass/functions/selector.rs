use super::{check, CallError, FunctionMap, ResolvedArgs};
use crate::css::{BadSelector, Selectors, Value};
use crate::sass::Name;
use crate::Scope;

pub fn create_module() -> Scope {
    let mut f = Scope::builtin_module("sass:selector");
    // TODO: is_superselector
    def_va!(f, append(selectors), |s| {
        let mut s = get_selectors(s, name!(selectors))?.into_iter();
        if let Some(base) = s.next() {
            Ok(s.try_fold(base, |base, ext| base.append(ext))?.into())
        } else {
            // Not really reachable, get_selectors requires at least one item.
            Ok(Selectors::root().into())
        }
    });
    // TODO: extend
    def_va!(f, nest(selectors), |s| {
        let mut v = get_selectors(s, name!(selectors))?.into_iter();
        let first = v.next().unwrap().css_ok()?;
        Ok(v.fold(first, |b, e| e.inside(&b)).into())
    });
    def!(f, parse(selector), |s| {
        Ok(s.get_map(name!(selector), parse_selectors_x)?.into())
    });
    // TODO: replace, unify, simple_selectors
    f
}

fn get_selectors(
    s: &ResolvedArgs,
    name: Name,
) -> Result<Vec<Selectors>, CallError> {
    Ok(s.get_map(name, check::va_list_nonempty)?
        .into_iter()
        .map(|v| v.try_into())
        .collect::<Result<_, BadSelector>>()?)
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

fn parse_selectors_x(v: Value) -> Result<Selectors, String> {
    Selectors::try_from(v)
        .and_then(|s| s.css_ok())
        .map_err(|e| e.to_string())
}
