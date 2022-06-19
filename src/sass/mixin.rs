use super::functions::get_string;
use super::{CallArgs, Callable, Closure, FormalArgs, Name, Value};
use crate::css::{CssString, ValueToMapError};
use crate::file_context::FileContext;
use crate::ordermap::OrderMap;
use crate::parser::{Parsed, SourcePos};
use crate::{Error, Scope, ScopeRef};
use std::convert::TryInto;

/// A declared mixin
#[derive(Clone)]
pub enum MixinDecl {
    /// an actual mixin
    Sass(Closure),
    /// The body of a mixin call that does not have a body
    NoBody,
    /// The special `load-css` mixin.
    LoadCss,
}

impl From<Closure> for MixinDecl {
    fn from(decl: Closure) -> Self {
        MixinDecl::Sass(decl)
    }
}

impl MixinDecl {
    pub(crate) fn get(
        self,
        name: &str,
        scope: ScopeRef,
        call_args: &CallArgs,
        call_pos: &SourcePos,
        file_context: &impl FileContext,
    ) -> Result<Mixin, Error> {
        match self {
            MixinDecl::Sass(decl) => {
                let sel = scope.get_selectors().clone();
                Ok(Mixin {
                    scope: decl
                        .body
                        .args
                        .eval(
                            ScopeRef::sub_selectors(decl.scope.clone(), sel),
                            call_args.evaluate(scope)?,
                        )
                        .map_err(|e| {
                            e.decl_called(
                                call_pos.in_call(name),
                                decl.body.decl.clone(),
                            )
                        })?,
                    body: Parsed::Scss(decl.body.body),
                })
            }
            MixinDecl::NoBody => Ok(Mixin::empty(scope)),
            MixinDecl::LoadCss => {
                let fargs = FormalArgs::new(vec![
                    (name!(url), None),
                    (name!(with), Some(Value::Null)),
                ]);
                let pos = SourcePos::mock_mixin(
                    &name!(load_css),
                    &fargs,
                    "sass:meta",
                );
                let call_pos2 = call_pos.clone();
                let argscope = fargs
                    .eval(scope.clone(), call_args.evaluate(scope.clone())?)
                    .map_err(|e| e.decl_called(call_pos2, pos))?;
                let call_pos2 = call_pos.clone();
                let url = get_string(&argscope, name!(url)).map_err(|e| {
                    Error::BadCall(e.to_string(), call_pos2, None)
                })?;
                let call_pos2 = call_pos.clone();
                let with = get_opt_map(&argscope, name!(with))
                    .map_err(|e| Error::BadCall(e, call_pos2, None))?;

                if ["sass:color", "sass:list" /*FIXME*/]
                    .iter()
                    .any(|name| *name == url.value())
                {
                    if with.unwrap_or_default().is_empty() {
                        return Ok(Mixin::empty(scope));
                    } else {
                        return Err(Error::BadCall(
                            format!(
                                "Error: Built-in module {} can't be configured.",
                                url.value()
                            ),
                            call_pos.clone(),
                            None,
                        ));
                    }
                }
                let call_pos2 = call_pos.clone();
                let source = file_context
                    .find_file_use(url.value(), call_pos.clone())?
                    .ok_or_else(|| {
                        Error::BadCall(
                            "Error: Can't find stylesheet to import.".into(),
                            call_pos2,
                            None,
                        )
                    })?;

                let scope = ScopeRef::sub(scope);
                if let Some(with) = with {
                    for (key, value) in with.into_iter() {
                        scope.define(key.into(), &value);
                    }
                }
                Ok(Mixin {
                    scope,
                    body: source.parse()?,
                })
            }
        }
    }
    pub(crate) fn is_no_body(&self) -> bool {
        matches!(self, MixinDecl::NoBody)
    }
}

/// A mixin is a callable body of items.
#[derive(Clone)]
pub struct Mixin {
    /// The scope where this mixin is defined.
    pub scope: ScopeRef,
    /// The body of this mixin.
    pub body: Parsed,
}

impl Mixin {
    fn empty(scope: ScopeRef) -> Self {
        Mixin {
            scope,
            body: Parsed::Css(vec![]),
        }
    }
    pub(crate) fn define_content(
        &self,
        scope: &ScopeRef,
        body: &Option<Callable>,
    ) {
        self.scope.define_mixin(
            Name::from_static("%%BODY%%"),
            match body {
                Some(body) => body.closure(scope).into(),
                None => MixinDecl::NoBody,
            },
        )
    }
}

pub(crate) fn get_opt_map(
    s: &Scope,
    name: Name,
) -> Result<Option<OrderMap<CssString, crate::css::Value>>, String> {
    match s.get(&name).map_err(|e| e.to_string())? {
        crate::css::Value::Null => Ok(None),
        v => v
            .try_into()
            .map_err(|e| match e {
                ValueToMapError::Root(err) => {
                    format!("Error: ${}: {}", name, err)
                }
                ValueToMapError::Key(err) => {
                    format!("Error: ${} key: {}", name, err)
                }
            })
            .map(Some),
    }
}
