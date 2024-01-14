use super::{unnamed, CheckedArg, FunctionMap};
use crate::css::CssSelectorSet;
use crate::Scope;

pub fn create_module() -> Scope {
    let mut f = Scope::builtin_module("sass:selector");
    def!(f, is_superselector(super, sub), |s| {
        let sup: CssSelectorSet = s.get(name!(super))?;
        let sub: CssSelectorSet = s.get(name!(sub))?;
        Ok(sup.is_superselector(&sub).into())
    });
    def_va!(f, append(selectors), |s| {
        let mut s = unnamed(s.get_va::<CssSelectorSet>(name!(selectors)))?
            .into_iter();
        if let Some(base) = s.next() {
            Ok(s.try_fold(base, |base, s| base.append(s))?.into())
        } else {
            Err("At least one selector must be passed.")
                .named(name!(selectors))
        }
    });
    // TODO: extend
    def_va!(f, nest(selectors), |s| {
        let mut v = unnamed(s.get_va(name!(selectors)))?.into_iter();
        let first = v
            .next()
            .ok_or("At least one selector must be passed.")
            .named(name!(selectors))?;
        let first = CssSelectorSet::try_from(first)?;
        Ok(v.fold(first, |b, e| b.nest(e)).into())
    });
    def!(f, parse(selector), |s| {
        Ok(s.get::<CssSelectorSet>(name!(selector))?.into())
    });
    def!(f, replace(selector, original, replacement), |s| {
        let selector: CssSelectorSet = s.get(name!(selector))?;
        let original: CssSelectorSet = s.get(name!(original))?;
        let replacement: CssSelectorSet = s.get(name!(replacement))?;
        Ok(selector.replace(&original, &replacement)?.into())
    });
    // TODO: simple_selectors
    def!(f, unify(selector1, selector2), |s| {
        let a: CssSelectorSet = s.get(name!(selector1))?;
        let b: CssSelectorSet = s.get(name!(selector2))?;
        Ok(a.unify(b).into())
    });

    f
}

pub fn expose(m: &Scope, global: &mut FunctionMap) {
    for (gname, lname) in &[
        // - - - Mixins - - -
        (name!(is_superselector), name!(is_superselector)),
        (name!(selector_append), name!(append)),
        //(name!(selector_extend), name!(extend)),
        (name!(selector_nest), name!(nest)),
        (name!(selector_parse), name!(parse)),
        (name!(selector_replace), name!(replace)),
        (name!(selector_unify), name!(unify)),
        //(name!(simple_selectors), name!(simple_selectors)),
    ] {
        global.insert(gname.clone(), m.get_lfunction(lname));
    }
}
