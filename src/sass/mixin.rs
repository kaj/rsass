use super::{
    CallArgs, CallError, Callable, Closure, FormalArgs, Name, ResolvedArgs,
    Value,
};
use crate::css::{self, CssString, ValueToMapError};
use crate::input::{Context, Loader, Parsed, SourceKind};
use crate::ordermap::OrderMap;
use crate::parser::SourcePos;
use crate::ScopeRef;
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
        scope: ScopeRef,
        call_args: &CallArgs,
        call_pos: &SourcePos,
        file_context: &mut Context<impl Loader>,
    ) -> Result<Mixin, CallError> {
        match self {
            MixinDecl::Sass(decl) => {
                let sel = scope.get_selectors().clone();
                Ok(Mixin {
                    scope: decl.eval_args(
                        ScopeRef::sub_selectors(decl.scope.clone(), sel),
                        call_args.evaluate(scope)?.args,
                    )?,
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
                let argscope = fargs
                    .eval_call(
                        scope.clone(),
                        call_args.evaluate(scope.clone())?,
                    )
                    .map_err(|e| e.declared_at(&pos))?;
                let url: CssString = argscope.get(name!(url))?;
                let with = get_opt_map(&argscope, name!(with))?;

                if url.value().starts_with("sass:") {
                    if with.unwrap_or_default().is_empty() {
                        return Ok(Mixin::empty(scope));
                    } else {
                        return Err(CallError::msg(format!(
                            "Built-in module {} can't be configured.",
                            url.value()
                        )));
                    }
                }
                let source = file_context
                    .find_file(url.value(), SourceKind::load_css(call_pos))?
                    .ok_or_else(|| {
                        CallError::msg("Can't find stylesheet to import.")
                    })?;

                let scope = ScopeRef::sub(scope);
                if let Some(with) = with {
                    for (key, value) in with.into_iter() {
                        scope.define(key.into(), value)?;
                    }
                }
                file_context.unlock_loading(&source);
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
        self.scope.define_content(match body {
            Some(body) => body.closure(scope).into(),
            None => MixinDecl::NoBody,
        })
    }
}

fn get_opt_map(
    s: &ResolvedArgs,
    name: Name,
) -> Result<Option<OrderMap<CssString, css::Value>>, CallError> {
    match s.get(name.clone())? {
        css::Value::Null => Ok(None),
        v => v.try_into().map(Some).map_err(|e| match e {
            ValueToMapError::Root(err) => CallError::BadArgument(name, err),
            ValueToMapError::Key(err) => {
                CallError::msg(format!("${} key: {}", name, err))
            }
        }),
    }
}
