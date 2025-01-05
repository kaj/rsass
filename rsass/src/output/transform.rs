//! This module provides `handle_body` (and internally `handle_item`),
//! that does most of the work for [`crate::input::Context::transform`].

use super::cssdest::CssDestination;
use super::CssData;
use crate::css::{self, AtRule, Import, SelectorCtx, Value};
use crate::error::ResultPos;
use crate::input::{Context, Loader, Parsed, SourceKind};
use crate::sass::{get_global_module, Expose, Item, UseAs};
use crate::{Error, Invalid, ScopeRef};

pub fn handle_parsed(
    items: Parsed,
    dest: &mut dyn CssDestination,
    scope: ScopeRef,
    file_context: &mut Context<impl Loader>,
) -> Result<(), Error> {
    match items {
        Parsed::Scss(items) => handle_body(&items, dest, scope, file_context),
        Parsed::Css(items) => push_items(dest, items).no_pos(),
    }
}

fn handle_body(
    items: &[Item],
    dest: &mut dyn CssDestination,
    scope: ScopeRef,
    file_context: &mut Context<impl Loader>,
) -> Result<(), Error> {
    for b in items {
        handle_item(b, dest, scope.clone(), file_context)?;
    }
    Ok(())
}

fn handle_item(
    item: &Item,
    dest: &mut dyn CssDestination,
    scope: ScopeRef,
    file_context: &mut Context<impl Loader>,
) -> Result<(), Error> {
    match item {
        Item::Use(ref name, ref as_n, ref with, ref pos) => {
            let name = name.evaluate(scope.clone())?.take_value();
            let module = if let Some(module) = get_global_module(&name) {
                if !with.is_empty() {
                    return Err(Invalid::ConfigBuiltin).at(pos);
                }
                module
            } else if let Some(sourcefile) =
                file_context.find_file(&name, SourceKind::Use(pos.clone()))?
            {
                let module = dest.head().load_module(
                    sourcefile.path(),
                    |dest| {
                        let module = ScopeRef::new_global(scope.get_format());
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
                                module.define(name.clone(), value)?;
                            } else {
                                return Err(Error::S(
                                    "The same variable may only be configured once.".to_string(),
                                ));
                            }
                        }
                        handle_parsed(
                            sourcefile.parse()?,
                            dest,
                            module.clone(),
                            file_context,
                        )?;
                        Ok(module)
                    })?;
                file_context.unlock_loading(&sourcefile);
                module
            } else {
                return Err(Error::BadCall(
                    "Can't find stylesheet to import.".into(),
                    pos.clone(),
                    None,
                ));
            };
            scope.do_use(module, &name, as_n, &Expose::All)?;
        }
        Item::Forward(ref name, ref as_n, ref expose, ref with, ref pos) => {
            let name = name.evaluate(scope.clone())?.take_value();
            let module = if let Some(module) = get_global_module(&name) {
                if !with.is_empty() {
                    return Err(Invalid::ConfigBuiltin).at(pos);
                }
                module
            } else if let Some(sourcefile) = file_context
                .find_file(&name, SourceKind::Forward(pos.clone()))?
            {
                let module = dest.head().load_module(
                    sourcefile.path(),
                    |dest| {
                        let module = ScopeRef::new_global(scope.get_format());
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
                                module.define(name.clone(), value)?;
                            } else {
                                return Err(Error::S(
                                    "The same variable may only be configured once.".to_string(),
                                ));
                            }
                        }
                        handle_parsed(
                            sourcefile.parse()?,
                            dest,
                            module.clone(),
                            file_context,
                        )?;
                        Ok(module)
                    });
                file_context.unlock_loading(&sourcefile);
                module?
            } else {
                return Err(Error::S(format!("Module {name} not found")));
            };
            scope.forward().do_use(module, &name, as_n, expose)?;
        }
        Item::Import(ref names, ref args, ref pos) => {
            'name: for name in names {
                let name = name.evaluate(scope.clone())?;
                if args.is_null() {
                    let x = name.value();
                    if let Some(sourcefile) = file_context
                        .find_file(x, SourceKind::Import(pos.clone()))?
                    {
                        match sourcefile.parse()? {
                            Parsed::Scss(items) => {
                                let mut thead = CssData::new();
                                let module = ScopeRef::sub(scope.clone());
                                let selectors = scope.get_selectors();
                                if !selectors.is_root() {
                                    let mut rule = thead
                                        .start_rule(selectors.real())
                                        .at(pos)?;
                                    handle_body(
                                        &items,
                                        &mut rule,
                                        module.clone(),
                                        file_context,
                                    )?;
                                } else {
                                    handle_body(
                                        &items,
                                        &mut thead,
                                        module.clone(),
                                        file_context,
                                    )?;
                                }
                                push_items(dest, thead.into_iter())
                                    .at(pos)?;
                                scope.do_use(
                                    module,
                                    "",
                                    &UseAs::Star,
                                    &Expose::All,
                                )?;
                            }
                            Parsed::Css(items) => {
                                push_items(dest, items).at(pos)?;
                            }
                        }
                        file_context.unlock_loading(&sourcefile);
                        continue 'name;
                    }
                    if !(x.starts_with("http://")
                        || x.starts_with("https://")
                        || x.starts_with("//")
                        || x.ends_with(".css")
                        || name.is_css_url())
                    {
                        return Err(Error::BadCall(
                            "Can't find stylesheet to import.".into(),
                            pos.clone(),
                            None,
                        ));
                    }
                }
                let args = args.evaluate(scope.clone())?;
                dest.push_import(Import::new(Value::Literal(name), args));
            }
        }
        Item::AtRoot(ref selectors, ref body) => {
            let selectors = selectors.eval(scope.clone())?;
            let ctx = scope.get_selectors().at_root(selectors);
            let selectors = ctx.real();
            let subscope = ScopeRef::sub_selectors(scope, ctx);
            if !selectors.is_root() {
                let mut rule = dest.start_rule(selectors).no_pos()?;
                handle_body(body, &mut rule, subscope, file_context)?;
            } else {
                handle_body(body, dest, subscope, file_context)?;
            }
        }
        Item::AtMedia { args, body, pos: _ } => {
            let args = args.evaluate(scope.clone())?;
            let mut atmedia = dest.start_atmedia(args.try_into()?);
            if let Some(ref body) = *body {
                let local = ScopeRef::sub(scope);
                handle_body(body, &mut atmedia, local, file_context)?;
            }
        }
        Item::AtRule {
            name,
            args,
            body,
            pos,
        } => {
            let name = name.evaluate(scope.clone())?.take_value();
            let args = args.evaluate(scope.clone())?;
            if let Some(ref body) = *body {
                let mut atrule = dest.start_atrule(name.clone(), args);
                let local = if name == "keyframes" {
                    ScopeRef::sub_selectors(scope, SelectorCtx::root())
                } else {
                    ScopeRef::sub(scope)
                };
                handle_body(body, &mut atrule, local, file_context)?;
            } else {
                dest.push_item(AtRule::new(name, args, None).into())
                    .at(pos)?;
            }
        }
        Item::VariableDeclaration(ref var) => {
            var.evaluate(&scope)?;
        }
        Item::FunctionDeclaration(ref name, ref body) => {
            if name == "calc"
                || name == "element"
                || name == "expression"
                || name == "url"
            {
                // Ok, this is cheating for the test suite ...
                let p = body.decl.clone().opt_back("@function ");
                return Err(Invalid::FunctionName.at(p));
            }
            check_body(&body.body, BodyContext::Function)?;
            scope.define_function(name.into(), body.closure(&scope).into());
        }
        Item::Return(_, ref pos) => {
            return Err(Invalid::AtRule.at(pos.clone()));
        }

        Item::MixinDeclaration(ref name, ref body) => {
            check_body(&body.body, BodyContext::Mixin)?;
            scope.define_mixin(name.into(), body.closure(&scope).into());
        }
        Item::MixinCall(ref name, ref args, ref body, ref pos) => {
            if let Some(mixin) = scope.get_mixin(&name.into()) {
                let mixin = mixin
                    .get(scope.clone(), args, pos, file_context)
                    .map_err(|e| e.called_from(pos, name))?;
                mixin.define_content(&scope, body);
                handle_parsed(mixin.body, dest, mixin.scope, file_context)
                    .map_err(|e: Error| match e {
                        Error::Invalid(err, _) => err.at(pos.clone()),
                        Error::BadCall(msg, pos, p2) => {
                            Error::BadCall(msg, pos.in_call(name), p2)
                        }
                        e => {
                            let pos = pos.in_call(name);
                            Error::BadCall(e.to_string(), pos, None)
                        }
                    })?;
            } else {
                return Err(Error::BadCall(
                    "Undefined mixin.".into(),
                    pos.clone(),
                    None,
                ));
            }
        }
        Item::Content(args, pos) => {
            if let Some(content) = scope.get_content() {
                let mixin = content
                    .get(scope, args, pos, file_context)
                    .map_err(|e| e.called_from(pos, "@content"))?;
                handle_parsed(mixin.body, dest, mixin.scope, file_context)?;
            }
        }

        Item::IfStatement(ref cond, ref do_if, ref do_else) => {
            let cond = cond.evaluate(scope.clone())?.is_true();
            let items = if cond { do_if } else { do_else };
            check_body(items, BodyContext::Control)?;
            handle_body(items, dest, scope, file_context)?;
        }
        Item::Each(ref names, ref values, ref body) => {
            check_body(body, BodyContext::Control)?;
            let pushed = scope.store_local_values(names);
            for value in values.evaluate(scope.clone())?.iter_items() {
                scope.define_multi(names, value)?;
                handle_body(body, dest, scope.clone(), file_context)?;
            }
            scope.restore_local_values(pushed);
        }
        Item::For(ref name, ref range, ref body) => {
            let range = range.evaluate(scope.clone())?;
            check_body(body, BodyContext::Control)?;
            for value in range {
                let scope = ScopeRef::sub(scope.clone());
                scope.define(name.clone(), value)?;
                handle_body(body, dest, scope, file_context)?;
            }
        }
        Item::While(ref cond, ref body) => {
            check_body(body, BodyContext::Control)?;
            let scope = ScopeRef::sub(scope);
            while cond.evaluate(scope.clone())?.is_true() {
                handle_body(body, dest, scope.clone(), file_context)?;
            }
        }

        Item::Debug(ref value) => {
            eprintln!("DEBUG: {}", value.evaluate(scope)?.introspect());
        }
        Item::Warn(ref value) => {
            eprintln!("WARNING: {}", value.evaluate(scope)?.introspect());
        }
        Item::Error(ref value, ref pos) => {
            let msg = value.evaluate(scope)?.introspect();
            return Err(Invalid::AtError(msg).at(pos.clone()));
        }
        Item::Extend(_selectors) => {
            return Err(Error::S("@extend is not supported yet".to_string()));
        }

        Item::Rule(ref selectors, ref body) => {
            check_body(body, BodyContext::Rule)?;
            let selectors = selectors.eval(scope.clone())?;
            let selectors = scope.get_selectors().nest(selectors);
            let mut dest = dest.start_rule(selectors.clone()).no_pos()?;
            let scope = ScopeRef::sub_selectors(scope, selectors.into());
            handle_body(body, &mut dest, scope, file_context)?;
        }
        Item::Property(ref name, ref value, ref pos) => {
            let v = value.evaluate(scope.clone())?;
            if !v.is_null() {
                let name = name.evaluate(scope)?.take_value();
                // Note: inner pos here is correctly the value pos,
                // but the outher should probably be the entire property.
                dest.push_property(name, v.valid_css().at(pos)?).at(pos)?;
            }
        }
        Item::CustomProperty(ref name, ref value) => {
            let v = value.evaluate(scope.clone())?;
            if !v.is_null() {
                let name = name.evaluate(scope)?.take_value();
                dest.push_custom_property(name, v).no_pos()?;
            }
        }
        Item::NamespaceRule(ref name, ref value, ref body) => {
            check_body(body, BodyContext::NsRule)?;
            let value = value.evaluate(scope.clone())?;
            let name = name.evaluate(scope.clone())?.take_value();
            if !value.is_null() {
                dest.push_property(name.clone(), value).no_pos()?;
            }
            let mut dest = dest.start_nsrule(name).no_pos()?;
            handle_body(body, &mut dest, scope, file_context)?;
        }
        Item::Comment(ref c) => {
            if !scope.get_format().is_compressed() {
                dest.push_comment(c.evaluate(scope)?.take_value().into());
            }
        }
        Item::None => (),
    }
    Ok(())
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum BodyContext {
    Mixin,
    Function,
    Control,
    Rule,
    NsRule,
}

fn check_body(body: &[Item], context: BodyContext) -> Result<(), Error> {
    for item in body {
        match item {
            Item::Forward(_, _, _, _, pos) => {
                return Err(Invalid::AtRule.at(pos.clone()));
            }
            Item::Use(_, _, _, pos) => {
                return Err(Invalid::AtRule.at(pos.clone()));
            }
            Item::MixinDeclaration(.., ref decl) => {
                let pos = decl.decl.clone().opt_back("@mixin ");
                match context {
                    BodyContext::Mixin => {
                        return Err(Invalid::MixinInMixin.at(pos));
                    }
                    BodyContext::Control => {
                        return Err(Invalid::MixinInControl.at(pos));
                    }
                    BodyContext::Rule => (), // This is ok
                    _ => {
                        return Err(Invalid::AtRule.at(pos.opt_trail_ws()));
                    }
                }
            }
            Item::FunctionDeclaration(_, ref body) => {
                let pos = body.decl.clone().opt_back("@function ");
                match context {
                    BodyContext::Mixin => {
                        return Err(Invalid::FunctionInMixin.at(pos));
                    }
                    BodyContext::Control => {
                        return Err(Invalid::FunctionInControl.at(pos));
                    }
                    BodyContext::Rule => (), // This is ok
                    _ => {
                        return Err(Invalid::AtRule.at(pos.opt_trail_ws()));
                    }
                }
            }
            Item::Return(_, ref pos) if context != BodyContext::Function => {
                return Err(Invalid::AtRule.at(pos.clone()));
            }
            Item::AtRule {
                name,
                args: _,
                body: _,
                pos,
            } if context != BodyContext::Rule => {
                if !name
                    .single_raw()
                    .map_or(false, |name| CSS_AT_RULES.contains(&name))
                {
                    return Err(Invalid::AtRule.at(pos.clone()));
                }
            }
            _ => (),
        }
    }
    Ok(())
}

const CSS_AT_RULES: [&str; 16] = [
    "charset",
    "color-profile",
    "counter-style",
    "document",
    "font-face",
    "font-feature-values",
    "import",
    "keyframes",
    "layer",
    "media",
    "namespace",
    "page",
    "property",
    "scroll-timeline",
    "supports",
    "viewport",
];

fn push_items(
    dest: &mut dyn CssDestination,
    items: impl IntoIterator<Item = css::Item>,
) -> Result<(), Invalid> {
    for item in items {
        dest.push_item(item)?;
    }
    Ok(())
}
