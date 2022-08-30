use super::{CallError, CheckedArg, Name};
use crate::css::Value;
use crate::ScopeRef;

/// The arguments to a builtin function.
pub struct ResolvedArgs {
    // Start out by wrapping a scoperef, turn it aound later.
    scope: ScopeRef,
    caller: ScopeRef,
}

impl ResolvedArgs {
    pub(crate) fn new(scope: ScopeRef, caller: ScopeRef) -> Self {
        ResolvedArgs { scope, caller }
    }

    /// Get a named argument.
    ///
    /// The argument is converted to the expected type with `TryFrom`.
    pub fn get<T>(&self, name: Name) -> Result<T, CallError>
    where
        T: TryFrom<Value>,
        <T as TryFrom<Value>>::Error: ToString,
    {
        self.get_map(name, |v| T::try_from(v).map_err(|e| e.to_string()))
    }

    /// Get an optional named argument.
    pub fn get_opt<T>(&self, name: Name) -> Result<Option<T>, CallError>
    where
        T: TryFrom<Value>,
        <T as TryFrom<Value>>::Error: ToString,
    {
        self.get_opt_map(name, |v| T::try_from(v).map_err(|e| e.to_string()))
    }

    /// Get a named argument.
    ///
    /// The `check` function is responsible for checking that the
    /// argument is valid and converting it to the correct type.
    pub fn get_map<T, F, E>(
        &self,
        name: Name,
        check: F,
    ) -> Result<T, CallError>
    where
        F: Fn(Value) -> Result<T, E>,
        E: ToString,
    {
        check(self.scope.get(&name)?).named(name)
    }

    /// Get an optional named argument.
    pub fn get_opt_map<T, F>(
        &self,
        name: Name,
        check: F,
    ) -> Result<Option<T>, CallError>
    where
        F: Fn(Value) -> Result<T, String>,
    {
        match self.scope.get(&name)? {
            Value::Null => Ok(None),
            v => check(v).named(name).map(Some),
        }
    }

    /// Get the scope from this the function was called.
    pub fn call_scope(&self) -> ScopeRef {
        self.caller.clone()
    }

    pub(crate) fn raw(&self) -> ScopeRef {
        self.scope.clone()
    }
}
