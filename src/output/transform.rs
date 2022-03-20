//! This module provides `handle_body` (and internally `handle_item`),
//! that does most of the work for [`super::Format::write_root`].

// https://github.com/rust-lang/rust-clippy/issues/7846
// https://users.rust-lang.org/t/using-an-option-mut-t-in-a-loop-clippy-complains/72481/2
#![allow(clippy::needless_option_as_deref)]

use super::cssbuf::{CssBuf, CssHead};
use crate::css::{BodyItem, Rule, Selectors};
use crate::error::Error;
use crate::file_context::FileContext;
use crate::sass::{
    get_global_module, CallArgs, Expose, Function, Item, MixinDecl, Name,
    UseAs,
};
use crate::value::ValueRange;
use crate::ScopeRef;
use std::io::Write;

pub fn handle_body(
    items: &[Item],
    head: &mut CssHead,
    rule: Option<&mut Rule>,
    buf: &mut CssBuf,
    scope: ScopeRef,
    file_context: &impl FileContext,
) -> Result<(), Error> {
    let mut rule = rule;
    for b in items {
        handle_item(
            b,
            head,
            rule.as_deref_mut(),
            buf,
            scope.clone(),
            file_context,
        )?;
    }
    Ok(())
}

fn handle_item(
    item: &Item,
    head: &mut CssHead,
    rule: Option<&mut Rule>,
    buf: &mut CssBuf,
    scope: ScopeRef,
    file_context: &impl FileContext,
) -> Result<(), Error> {
    let format = scope.get_format();
    match item {
        Item::Use(ref name, ref as_n, ref with) => {
            let name = name.evaluate(scope.clone())?;
            let module = if let Some(module) = get_global_module(name.value())
            {
                if !with.is_empty() {
                    return Err(Error::error(
                        "Built-in modules can\'t be configured.",
                    ));
                }
                module
            } else if let Some(sourcefile) = file_context.find_file_use(
                name.value(),
                // TODO: Get a proper pos!
                crate::parser::code_span(b"").into(),
            )? {
                head.load_module(
                    sourcefile.path(),
                    |head| {
                        let module = ScopeRef::new_global(format);
                        for (name, value, default) in with {
                            let default = if *default {
                                scope.get_or_none(name)
                            } else {
                                None
                            };
                            let value = default.ok_or(()).or_else(|()| {
                                value.do_evaluate(scope.clone(), true)
                            })?;
                            if module.get_or_none(name).is_none() {
                                module.define(name.clone(), &value);
                            } else {
                                return Err(Error::error(
                                    "The same variable may only be configured once.",
                                ));
                            }
                        }
                        let items = sourcefile.parse()?;
                        handle_body(
                            &items,
                            head,
                            None,
                            buf,
                            module.clone(),
                            file_context,
                        )?;
                        Ok(module)
                    })?
            } else {
                return Err(Error::S(format!("Module {} not found", name)));
            };
            scope.do_use(module, name.value(), as_n, &Expose::All)?;
        }
        Item::Forward(ref name, ref as_n, ref expose, ref with) => {
            let name = name.evaluate(scope.clone())?;
            let module = if let Some(module) = get_global_module(name.value())
            {
                if !with.is_empty() {
                    return Err(Error::error(
                        "Built-in modules can\'t be configured.",
                    ));
                }
                module
            } else if let Some(sourcefile) = file_context.find_file_use(
                name.value(),
                // TODO: Get a proper pos!
                crate::parser::code_span(b"").into(),
            )? {
                head.load_module(
                    sourcefile.path(),
                    |head| {
                        let module = ScopeRef::new_global(format);
                        for (name, value, default) in with {
                            let default = if *default {
                                scope.get_or_none(name)
                            } else {
                                None
                            };
                            let value = default.ok_or(()).or_else(|()| {
                                value.do_evaluate(scope.clone(), true)
                            })?;
                            if module.get_or_none(name).is_none() {
                                module.define(name.clone(), &value);
                            } else {
                                return Err(Error::error(
                                    "The same variable may only be configured once.",
                                ));
                            }
                        }
                        let items = sourcefile.parse()?;
                        handle_body(
                            &items,
                            head,
                            None,
                            buf,
                            module.clone(),
                            file_context,
                        )?;
                        Ok(module)
                    })?
            } else {
                return Err(Error::S(format!("Module {} not found", name)));
            };
            scope.forward().do_use(module, name.value(), as_n, expose)?;
        }
        Item::Import(ref names, ref args, ref pos) => {
            let mut rule = rule;
            'name: for name in names {
                if args.is_null() {
                    let x = name.evaluate(scope.clone())?;
                    if let Some(sourcefile) = file_context
                        .find_file_import(x.value(), pos.clone())?
                    {
                        scope.lock_loading(sourcefile.path(), pos.clone())?;
                        let items = sourcefile.parse()?;
                        let mut thead = CssHead::new(format);
                        let module = ScopeRef::sub(scope.clone());
                        handle_body(
                            &items,
                            &mut thead,
                            rule.as_deref_mut(),
                            buf,
                            module.clone(),
                            file_context,
                        )?;
                        head.merge_imports(thead);
                        scope.do_use(
                            module,
                            "",
                            &UseAs::Star,
                            &Expose::All,
                        )?;
                        scope.unlock_loading(sourcefile.path());
                        continue 'name;
                    }
                }
                let name = name.evaluate(scope.clone())?;
                let args = args.evaluate(scope.clone())?;
                if let Some(ref mut rule) =
                    rule.as_deref_mut().filter(|r| !r.selectors.is_root())
                {
                    rule.add_import(name, args);
                } else if buf.is_root_level() {
                    head.add_import(name, args)?;
                } else {
                    buf.add_import(name, args)?;
                }
            }
        }
        Item::AtRoot(ref selectors, ref body) => {
            let selectors = selectors
                .eval(scope.clone())?
                .with_backref(scope.get_selectors().one());
            let mut rule = Rule::new(selectors.clone());
            let mut sub = CssBuf::new_as(buf);
            handle_body(
                body,
                head,
                Some(&mut rule),
                &mut sub,
                ScopeRef::sub_selectors(scope, selectors),
                file_context,
            )?;
            buf.write_rule(&rule, true)?;
            buf.join(sub);
        }
        Item::AtRule {
            ref name,
            ref args,
            ref body,
        } => {
            buf.do_separate();
            let name = name.evaluate(scope.clone())?;
            write!(buf, "@{}", name.value())?;
            let args = args.evaluate(scope.clone())?;
            if !args.is_null() {
                write!(buf, " {}", args.format(format))?;
            }
            if let Some(ref body) = *body {
                buf.add_one(" {", "{");
                let selectors = scope.get_selectors().clone();
                let has_selectors = !selectors.is_root();
                let mut rule = Rule::new(selectors);
                let mut sub = CssBuf::new_below(buf);
                handle_body(
                    body,
                    head,
                    Some(&mut rule),
                    &mut sub,
                    ScopeRef::sub(scope),
                    file_context,
                )?;
                let mut t = CssBuf::new_as(&sub);
                if has_selectors {
                    t.write_rule(&rule, false)?;
                } else {
                    t.write_body_items(&rule.body)?;
                };
                if !t.is_empty() || !sub.is_empty() {
                    buf.join(t);
                    buf.do_indent();
                    buf.join(sub);
                }
                buf.add_str("}");
                buf.do_indent();
            } else {
                buf.add_str(";");
            }
        }

        Item::VariableDeclaration {
            ref name,
            ref val,
            default,
            global,
        } => {
            let val = val.do_evaluate(scope.clone(), true)?;
            scope.set_variable(name.into(), val, *default, *global);
        }
        Item::FunctionDeclaration(ref name, ref args, ref pos, ref body) => {
            if name == "calc"
                || name == "element"
                || name == "expression"
                || name == "url"
            {
                let mut p = pos.clone();
                // Ok, this is cheating for the test suite ...
                p.opt_back("@function ");
                return Err(Error::InvalidFunctionName(p));
            }
            scope.define_function(
                name.into(),
                Function::closure(
                    args.clone(),
                    pos.clone(),
                    scope.clone(),
                    body.clone(),
                ),
            );
        }
        Item::Return(_) => {
            return Err(Error::S(
                "Return not allowed in plain context".into(),
            ));
        }

        Item::MixinDeclaration(ref name, ref args, ref body, ref pos) => {
            scope.define_mixin(
                name.into(),
                MixinDecl::new(args, &scope, body, pos),
            )
        }
        Item::MixinCall(ref name, ref args, ref body, ref pos) => {
            if let Some(mixin) = scope.get_mixin(&name.into()) {
                let mixin = mixin.get(
                    name,
                    scope.clone(),
                    args,
                    pos,
                    file_context,
                )?;
                mixin.define_content(&scope, body, pos);
                handle_body(
                    &mixin.body,
                    head,
                    rule,
                    buf,
                    mixin.scope,
                    file_context,
                )
                .map_err(|e: Error| match e {
                    Error::BadArguments(msg, decl) => {
                        let pos = pos.in_call(name);
                        Error::BadCall(msg, pos, Some(decl))
                    }
                    Error::AtError(msg, _pos) => {
                        let msg = format!("Error: {}", msg);
                        Error::BadCall(msg, pos.clone(), None)
                    }
                    e => {
                        let pos = pos.in_call(name);
                        Error::BadCall(e.to_string(), pos, None)
                    }
                })?;
            } else {
                return Err(Error::BadCall(
                    format!("Unknown mixin {}", name),
                    pos.clone(),
                    None,
                ));
            }
        }
        Item::Content(pos) => {
            if let Some(content) =
                scope.get_mixin(&Name::from_static("%%BODY%%"))
            {
                if !content.is_no_body() {
                    let mixin = content.get(
                        "@content",
                        scope,
                        &CallArgs::new(vec![])?,
                        pos,
                        file_context,
                    )?;
                    handle_body(
                        &mixin.body,
                        head,
                        rule,
                        buf,
                        mixin.scope,
                        file_context,
                    )?;
                }
            }
        }

        Item::IfStatement(ref cond, ref do_if, ref do_else) => {
            let cond = cond.evaluate(scope.clone())?.is_true();
            let items = if cond { do_if } else { do_else };
            handle_body(items, head, rule, buf, scope, file_context)?;
        }
        Item::Each(ref names, ref values, ref body) => {
            let mut rule = rule;
            let pushed = scope.store_local_values(names);
            for value in values.evaluate(scope.clone())?.iter_items() {
                scope.define_multi(names, &value);
                handle_body(
                    body,
                    head,
                    rule.as_deref_mut(),
                    buf,
                    scope.clone(),
                    file_context,
                )?;
            }
            scope.restore_local_values(pushed);
        }
        Item::For {
            ref name,
            ref from,
            ref to,
            inclusive,
            ref body,
        } => {
            let range = ValueRange::new(
                from.evaluate(scope.clone())?,
                to.evaluate(scope.clone())?,
                *inclusive,
            )?;
            let mut rule = rule;
            for value in range {
                let scope = ScopeRef::sub(scope.clone());
                scope.define(name.clone(), &value);
                handle_body(
                    body,
                    head,
                    rule.as_deref_mut(),
                    buf,
                    scope,
                    file_context,
                )?;
            }
        }
        Item::While(ref cond, ref body) => {
            let mut rule = rule;
            let scope = ScopeRef::sub(scope);
            while cond.evaluate(scope.clone())?.is_true() {
                handle_body(
                    body,
                    head,
                    rule.as_deref_mut(),
                    buf,
                    scope.clone(),
                    file_context,
                )?;
            }
        }

        Item::Debug(ref value) => {
            eprintln!("DEBUG: {}", value.evaluate(scope)?.format(format));
        }
        Item::Warn(ref value) => {
            eprintln!("WARNING: {}", value.evaluate(scope)?.format(format));
        }
        Item::Error(ref value, ref pos) => {
            return Err(Error::AtError(
                value.evaluate(scope)?.format(format).to_string(),
                pos.clone(),
            ));
        }

        Item::Rule(ref selectors, ref body) => {
            if rule.is_none() {
                buf.do_separate();
            }
            let selectors =
                selectors.eval(scope.clone())?.inside(scope.get_selectors());
            let mut rule = Rule::new(selectors.clone());
            let mut sub = CssBuf::new_as(buf);
            handle_body(
                body,
                head,
                Some(&mut rule),
                &mut sub,
                ScopeRef::sub_selectors(scope, selectors),
                file_context,
            )?;
            buf.write_rule(&rule, true)?;
            buf.join(sub);
        }
        Item::Property(ref name, ref value) => {
            if let Some(rule) = rule {
                let v = value.evaluate(scope.clone())?;
                if !v.is_null() {
                    let name = name.evaluate(scope)?;
                    rule.push(BodyItem::Property(name.value().into(), v));
                }
            } else {
                return Err(Error::S("Global property not allowed".into()));
            }
        }
        Item::CustomProperty(ref name, ref value) => {
            if let Some(rule) = rule {
                let v = value.evaluate(scope.clone())?;
                if !v.is_null() {
                    let name = name.evaluate(scope)?;
                    rule.push(BodyItem::CustomProperty(
                        name.value().into(),
                        v,
                    ));
                }
            } else {
                return Err(Error::S(
                    "Global custom property not allowed".into(),
                ));
            }
        }
        Item::NamespaceRule(ref name, ref value, ref body) => {
            if let Some(rule) = rule {
                let value = value.evaluate(scope.clone())?;
                let name = name.evaluate(scope.clone())?;
                if !value.is_null() {
                    rule.push(BodyItem::Property(
                        name.value().to_string(),
                        value,
                    ));
                }
                let mut t = Rule::new(Selectors::root());
                let mut sub = CssBuf::new(format);
                handle_body(
                    body,
                    head,
                    Some(&mut t),
                    &mut sub,
                    scope,
                    file_context,
                )?;
                for item in t.body {
                    rule.push(match item {
                        BodyItem::Property(n, v) => BodyItem::Property(
                            format!("{}-{}", name.value(), n),
                            v,
                        ),
                        c => c,
                    })
                }
                if !sub.is_empty() {
                    return Err(Error::S(
                        "Unexpected content in namespace rule".into(),
                    ));
                }
            } else {
                return Err(Error::S(
                    "Global namespaced property not allowed".into(),
                ));
            }
        }
        Item::Comment(ref c) => {
            if !format.is_compressed() {
                let c = c.evaluate(scope)?;
                if let Some(rule) = rule {
                    rule.push(BodyItem::Comment(c.value().into()));
                } else {
                    buf.do_separate();
                    write!(buf, "/*{}*/", c)?;
                }
            }
        }

        Item::None => (),
    }
    Ok(())
}
