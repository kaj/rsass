use super::{FormalArgs, Item};
use crate::{ScopeRef, SourcePos};

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
