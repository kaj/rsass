use super::{FormalArgs, Item};
use crate::SourcePos;

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
}
