use super::{CallError, FormalArgs, Item};
use crate::css::{CallArgs, Value};
use crate::{Error, ScopeRef, SourcePos};

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
    pub(crate) fn eval_args(
        &self,
        scope: ScopeRef,
        args: CallArgs,
    ) -> Result<ScopeRef, CallError> {
        self.body
            .args
            .eval(scope, args)
            .map_err(|e| e.declared_at(&self.body.decl))
    }

    /// Evaluate this callable as a value.
    ///
    /// This is used when the callable is a scss function.
    pub fn eval_value(&self, call: Call) -> Result<Value, CallError> {
        Ok(self
            .eval_args(self.scope.clone(), call.args)?
            .eval_body(&self.body.body)
            .map_err(|e| match e {
                Error::Invalid(err, _pos) => CallError::Invalid(err),
                e => CallError::from(e),
            })?
            .unwrap_or(Value::Null))
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

/// The calling context for a call to a callable.
///
/// This is the evaluated argumends and the calling scope.
pub struct Call {
    pub(crate) args: CallArgs,
    pub(crate) scope: ScopeRef,
}
