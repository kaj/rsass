use crate::sass::{FormalArgs, Item};
use crate::ScopeRef;

/// A mixin is a callable body of items.
#[derive(Clone)]
pub struct Mixin {
    /// The arguments to this mixin.
    pub args: FormalArgs,
    /// The scope where this mixin is defined.
    pub scope: ScopeRef,
    /// The body of this mixin.
    pub body: Vec<Item>,
}
