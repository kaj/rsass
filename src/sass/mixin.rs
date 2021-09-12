use crate::parser::SourcePos;
use crate::sass::{FormalArgs, Item, Value};
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
    /// The position where the mixin is declared.
    pub pos: SourcePos,
}

impl Mixin {
    /// An illegal mixin body, used for `@content` on mixin calls sans body.
    pub(crate) fn no_body() -> Vec<Item> {
        vec![Item::Property("%%NO-BODY%%".into(), Value::Null)]
    }
    pub(crate) fn is_no_body(body: &[Item]) -> bool {
        body == [Item::Property("%%NO-BODY%%".into(), Value::Null)]
    }
}
