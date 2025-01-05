use super::Value;
use crate::{css, input::SourcePos, Error, Invalid, ScopeRef};

/// A value with a specific source position
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct SrcValue {
    value: Value,
    pos: SourcePos,
}

impl SrcValue {
    pub fn new(value: Value, pos: SourcePos) -> Self {
        Self { value, pos }
    }
    pub fn eval_map<T, F>(&self, scope: ScopeRef, f: F) -> Result<T, Error>
    where
        F: Fn(css::Value) -> Result<T, Invalid>,
    {
        // TODO: The position should be applied to err from evaluate as well!
        self.value
            .evaluate(scope)
            .and_then(|v| f(v).map_err(|e| e.at(self.pos.clone())))
    }
}
