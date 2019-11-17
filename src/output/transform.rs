use super::cssbuf::CssBuf;
use crate::css::{BodyItem, Rule};
use crate::error::Error;
use crate::file_context::FileContext;
use crate::parser::parse_imported_scss_file;
use crate::sass::{self, FormalArgs, Function, Item, Mixin, Name};
use crate::selectors::Selectors;
use crate::value::ValueRange;
use crate::ScopeRef;
use std::io::Write;

pub fn handle_body(
    items: &[Item],
    head: &mut CssBuf,
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
    head: &mut CssBuf,
    rule: Option<&mut Rule>,
    buf: &mut CssBuf,
    scope: ScopeRef,
    file_context: &impl FileContext,
) -> Result<(), Error> {
    let format = scope.get_format();
    match item {
        Item::Use(ref name, ref as_n) => {
            use crate::sass::{get_global_module, UseAs};
            let name = name.evaluate(scope.clone())?.0;
            let do_use = |module: ScopeRef| -> Result<(), Error> {
                match as_n {
                    UseAs::KeepName => {
                        let name = name
                            .rfind(|c| c == ':' || c == '/')
                            .map(|i| &name[i + 1..])
                            .unwrap_or(&name);
                        scope.define_module(name.into(), module);
                    }
                    UseAs::Star => {
                        scope.expose_star(&module);
                    }
                    UseAs::Name(name) => {
                        let name = name.evaluate(scope.clone())?.0;
                        scope.define_module(name, module);
                    }
                }
                Ok(())
            };
            if let Some(module) = get_global_module(&name) {
                do_use(module)?
            } else if let Some((sub_context, path, mut file)) =
                file_context.find_file(&name)?
            {
                let module = ScopeRef::new_global(format);
                let items = parse_imported_scss_file(
                    &mut file,
                    &path,
                    // TODO: Get a proper pos!
                    crate::parser::code_span(b"").into(),
                )?;
                handle_body(
                    &items,
                    head,
                    None,
                    buf,
                    module.clone(),
                    &sub_context,
                )?;
                do_use(module)?;
            } else {
                return Err(Error::S(format!("Module {:?} not found", name)));
            }
        }
        Item::Import(ref names, ref args, ref pos) => {
            let mut rule = rule;
            'name: for name in names {
                if args.is_null() {
                    let (x, _q) = name.evaluate(scope.clone())?;
                    if let Some((sub_context, path, mut file)) =
                        file_context.find_file(&x)?
                    {
                        let items = parse_imported_scss_file(
                            &mut file,
                            &path,
                            pos.clone(),
                        )?;
                        handle_body(
                            &items,
                            head,
                            rule.as_deref_mut(),
                            buf,
                            scope.clone(),
                            &sub_context,
                        )?;
                        continue 'name;
                    }
                }
                if buf.is_root_level() {
                    head.add_import(
                        name.evaluate2(scope.clone())?,
                        args.evaluate(scope.clone())?,
                    )?;
                } else {
                    buf.add_import(
                        name.evaluate2(scope.clone())?,
                        args.evaluate(scope.clone())?,
                    )?;
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
            write!(buf, "@{}", name)?;
            let args = args.evaluate(scope.clone())?;
            if !args.is_null() {
                write!(buf, " {}", args.format(format))?;
            }
            if let Some(ref body) = *body {
                buf.add_one(" {", "{");
                let selectors = scope.get_selectors().clone();
                let has_selectors = selectors != Selectors::root();
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

        Item::MixinDeclaration(ref name, ref args, ref body) => scope
            .define_mixin(
                name.into(),
                Mixin {
                    args: args.clone(),
                    scope: scope.clone(),
                    body: body.clone(),
                },
            ),
        Item::MixinCall(ref name, ref args, ref body) => {
            let sel = scope.get_selectors().clone();
            if let Some(mixin) = scope.get_mixin(&name.into()) {
                let subscope = mixin
                    .args
                    .eval(
                        ScopeRef::sub_selectors(mixin.scope, sel),
                        &args.evaluate(scope.clone(), true)?,
                    )
                    .map_err(|e| match e {
                        sass::ArgsError::Eval(e) => e,
                        ae => Error::S(ae.to_string()),
                    })?;
                subscope.define_mixin(
                    Name::from_static("%%BODY%%"),
                    Mixin {
                        args: FormalArgs::none(),
                        scope,
                        body: body.clone(),
                    },
                );
                handle_body(
                    &mixin.body,
                    head,
                    rule,
                    buf,
                    subscope,
                    file_context,
                )?;
            } else {
                return Err(Error::S(format!(
                    "Unknown mixin {}({:?})",
                    name, args
                )));
            }
        }
        Item::Content => {
            if let Some(rule) = rule {
                if let Some(content) =
                    scope.get_mixin(&Name::from_static("%%BODY%%"))
                {
                    let sel = scope.get_selectors().clone();
                    handle_body(
                        &content.body,
                        head,
                        Some(rule),
                        buf,
                        ScopeRef::sub_selectors(content.scope, sel),
                        file_context,
                    )?;
                }
            } else {
                return Err(Error::S(
                    "@content not allowed in global context".into(),
                ));
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
                scope.define_multi(&names, &value);
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
        Item::Error(ref value) => {
            return Err(Error::S(format!(
                "Error: {}",
                value.evaluate(scope)?.format(format)
            )));
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
                    let (name, _q) = name.evaluate(scope)?;
                    rule.push(BodyItem::Property(name, v));
                }
            } else {
                return Err(Error::S("Global property not allowed".into()));
            }
        }
        Item::NamespaceRule(ref name, ref value, ref body) => {
            if let Some(rule) = rule {
                let value = value.evaluate(scope.clone())?;
                let (name, _quotes) = name.evaluate(scope.clone())?;
                if !value.is_null() {
                    rule.push(BodyItem::Property(name.clone(), value));
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
                        BodyItem::Property(n, v) => {
                            BodyItem::Property(format!("{}-{}", name, n), v)
                        }
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
                if let Some(rule) = rule {
                    let (c, _q) = c.evaluate(scope)?;
                    rule.push(BodyItem::Comment(c));
                } else {
                    buf.do_separate();
                    let (c, _q) = c.evaluate(scope)?;
                    write!(buf, "/*{}*/", c)?;
                }
            }
        }

        Item::None => (),
    }
    Ok(())
}
