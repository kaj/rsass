use super::{CallError, CheckedArg, Name};
use crate::ScopeRef;
use crate::css::Value;
use crate::value::ListSeparator;

/// The arguments to a builtin function.
pub struct ResolvedArgs {
    // Start out by wrapping a scoperef, turn it aound later.
    scope: ScopeRef,
    caller: ScopeRef,
}

impl ResolvedArgs {
    pub(crate) fn new(scope: ScopeRef, caller: ScopeRef) -> Self {
        Self { scope, caller }
    }

    /// Get a named argument.
    ///
    /// The argument is converted to the expected type with `TryFrom`.
    pub fn get<T>(&self, name: Name) -> Result<T, CallError>
    where
        T: TryFrom<Value>,
        Result<T, <T as TryFrom<Value>>::Error>: CheckedArg<T>,
    {
        self.get_map(name, T::try_from)
    }

    /// Get a checked var-args parameter as a Vec of a given type.
    pub fn get_va<T>(&self, name: Name) -> Result<Vec<T>, CallError>
    where
        T: TryFrom<Value>,
        Result<Vec<T>, <T as TryFrom<Value>>::Error>: CheckedArg<Vec<T>>,
    {
        match self.scope.get(&name)? {
            Value::ArgList(args) => {
                args.check_no_named().named(name.clone())?;
                args.positional
                    .into_iter()
                    .map(T::try_from)
                    .collect::<Result<Vec<T>, _>>()
                    .named(name)
            }
            Value::List(v, Some(ListSeparator::Comma), false) => v
                .into_iter()
                .map(T::try_from)
                .collect::<Result<Vec<T>, _>>()
                .named(name),
            single => T::try_from(single).map(|t| vec![t]).named(name),
        }
    }

    /// Get an optional named argument.
    pub fn get_opt<T>(&self, name: Name) -> Result<Option<T>, CallError>
    where
        T: TryFrom<Value>,
        Result<T, <T as TryFrom<Value>>::Error>: CheckedArg<T>,
    {
        self.get_opt_map(name, T::try_from)
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
        Result<T, E>: CheckedArg<T>,
    {
        check(self.scope.get(&name)?).named(name)
    }

    /// Get an optional named argument.
    pub fn get_opt_map<T, F, E>(
        &self,
        name: Name,
        check: F,
    ) -> Result<Option<T>, CallError>
    where
        F: Fn(Value) -> Result<T, E>,
        Result<T, E>: CheckedArg<T>,
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
