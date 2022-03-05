use super::functions::get_string;
use super::{CallArgs, FormalArgs, Item, Name, Value};
use crate::css::{CssString, ValueToMapError};
use crate::file_context::FileContext;
use crate::ordermap::OrderMap;
use crate::parser::SourcePos;
use crate::{Error, Scope, ScopeRef};
use std::convert::TryInto;

/// A declared mixin
#[derive(Clone)]
pub enum MixinDecl {
    /// an actual mixin
    Sass(MixinDeclImpl),
    /// The body of a mixin call that does not have a body
    NoBody,
    /// The special `load-css` mixin.
    LoadCss,
}

#[derive(Clone)]
pub struct MixinDeclImpl {
    args: FormalArgs,
    scope: ScopeRef,
    body: Vec<Item>,
    pos: SourcePos,
}

impl From<MixinDeclImpl> for MixinDecl {
    fn from(decl: MixinDeclImpl) -> Self {
        MixinDecl::Sass(decl)
    }
}

impl MixinDecl {
    /// Create a new Mixin.
    pub fn new(
        args: &FormalArgs,
        scope: &ScopeRef,
        body: &[Item],
        pos: &SourcePos,
    ) -> Self {
        MixinDeclImpl {
            args: args.clone(),
            scope: scope.clone(),
            body: body.to_vec(),
            pos: pos.clone(),
        }
        .into()
    }
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
                        .args
                        .eval(
                            ScopeRef::sub_selectors(decl.scope.clone(), sel),
                            call_args
                                .evaluate(scope)
                                .map_err(crate::sass::ArgsError::Eval)?,
                        )
                        .map_err(|e| match e {
                            crate::sass::ArgsError::Eval(e) => e,
                            ae => Error::BadCall(
                                ae.to_string(),
                                call_pos.in_call(name),
                                Some(decl.pos.clone()),
                            ),
                        })?,
                    body: decl.body,
                })
            }
            MixinDecl::NoBody => unreachable!(),
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
                    .eval(
                        scope.clone(),
                        call_args
                            .evaluate(scope.clone())
                            .map_err(crate::sass::ArgsError::Eval)?,
                    )
                    .map_err(|e| match e {
                        crate::sass::ArgsError::Eval(e) => e,
                        ae => Error::BadCall(
                            ae.to_string(),
                            call_pos2,
                            Some(pos),
                        ),
                    })?;
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
                        return Ok(Mixin {
                            scope,
                            body: vec![],
                        });
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

                let body = source.parse()?;

                let scope = ScopeRef::sub(scope);
                if let Some(with) = with {
                    for (key, value) in with.into_iter() {
                        scope.define(key.into(), &value);
                    }
                }
                Ok(Mixin { scope, body })
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
    pub body: Vec<Item>,
}

impl Mixin {
    pub(crate) fn define_content(
        &self,
        scope: &ScopeRef,
        body: &Option<Vec<Item>>,
        pos: &SourcePos,
    ) {
        self.scope.define_mixin(
            Name::from_static("%%BODY%%"),
            match body {
                Some(body) => MixinDeclImpl {
                    args: FormalArgs::none(),
                    scope: scope.clone(),
                    body: body.to_vec(),
                    pos: pos.clone(),
                }
                .into(),
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
