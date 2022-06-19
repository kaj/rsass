use super::{FormalArgs, Item};
use crate::{
    css::{CallArgs, Value},
    Error, ScopeRef, SourcePos,
};

/// The callable part of a sass mixin or function.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct Callable {
    pub(crate) args: FormalArgs,
    pub(crate) body: Vec<Item>,
    pub(crate) decl: SourcePos,
}

impl Callable {
    /// Create a new callable.
    pub fn new(args: FormalArgs, body: Vec<Item>, decl: SourcePos) -> Self {
        Callable { args, body, decl }
    }
    /// Create a new callable without arguments.
    pub fn no_args(body: Vec<Item>, decl: SourcePos) -> Self {
        Callable {
            args: FormalArgs::none(),
            body,
            decl,
        }
    }
    /// Combine this callable with a scope to get a closure.
    pub fn closure(&self, scope: &ScopeRef) -> Closure {
        Closure {
            body: self.clone(),
            scope: scope.clone(),
        }
    }
}

/// A callable combined with the runtime scope where it was declared.
#[derive(Clone)]
pub struct Closure {
    pub(crate) body: Callable,
    pub(crate) scope: ScopeRef,
}

impl Closure {
    /// Evaluate this callable as a value.
    ///
    /// This is used when the callable is a scss function.
    pub fn eval_value(
        &self,
        callscope: ScopeRef,
        args: CallArgs,
    ) -> Result<Value, Error> {
        let s = self.do_eval_args(self.scope.clone(), args)?;
        s.define_module("%%CALLING_SCOPE%%".into(), callscope);
        Ok(s.eval_body(&self.body.body)?.unwrap_or(Value::Null))
    }

    fn do_eval_args(
        &self,
        def: ScopeRef,
        args: CallArgs,
    ) -> Result<ScopeRef, Error> {
        self.body
            .args
            .eval(def, args)
            .map_err(|e| e.declared_at(&self.body.decl))
    }
}

impl std::cmp::PartialEq for Closure {
    fn eq(&self, other: &Self) -> bool {
        ScopeRef::is_same(&self.scope, &other.scope)
            && self.body == other.body
    }
}
impl std::cmp::Eq for Closure {}
impl std::cmp::PartialOrd for Closure {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.body.partial_cmp(&other.body) {
            None => None,
            Some(std::cmp::Ordering::Equal) => {
                if ScopeRef::is_same(&self.scope, &other.scope) {
                    Some(std::cmp::Ordering::Equal)
                } else {
                    None
                }
            }
            Some(defined) => Some(defined),
        }
    }
}
