use super::{Call, Closure, FormalArgs, Name};
use crate::css::{is_not, Value};
use crate::input::SourcePos;
use crate::{Scope, ScopeRef};
use lazy_static::lazy_static;
use std::collections::BTreeMap;
use std::sync::Arc;
use std::{cmp, fmt};

#[macro_use]
mod macros;

mod call_error;
mod color;
mod list;
mod map;
mod math;
mod meta;
mod num_or_special;
mod resolvedargs;
mod selector;
mod string;

pub use call_error::CallError;
use num_or_special::NumOrSpecial;
pub use resolvedargs::ResolvedArgs;

type BuiltinFn =
    dyn Fn(&ResolvedArgs) -> Result<Value, CallError> + Send + Sync;

/// A function that can be called from a sass value.
///
/// The function can be either "builtin" (implemented in rust) or
/// "user defined" (implemented in scss).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct Function {
    body: FuncImpl,
}

#[derive(Clone, PartialEq, Eq, PartialOrd)]
enum FuncImpl {
    Builtin(Builtin),
    /// A user-defined function is really a closure, it has a scope
    /// where it is defined and a body of items.
    UserDefined(Closure),
}

#[derive(Clone)]
struct Builtin {
    args: FormalArgs,
    pos: SourcePos,
    body: Arc<BuiltinFn>,
}

impl Builtin {
    fn eval_value(&self, call: Call) -> Result<Value, CallError> {
        let s = self
            .args
            .eval_call(ScopeRef::new_global(call.scope.get_format()), call)
            .map_err(|e| e.declared_at(&self.pos))?;
        (self.body)(&s)
    }
}

impl PartialOrd for Builtin {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        if self == other {
            Some(cmp::Ordering::Equal)
        } else {
            None
        }
    }
}

impl cmp::PartialEq for Builtin {
    fn eq(&self, other: &Self) -> bool {
        // Each builtin function is only created once, so this
        // should be ok.
        self.args == other.args && self.pos == other.pos && {
            Arc::ptr_eq(&self.body, &other.body)
        }
    }
}
impl cmp::Eq for Builtin {}

impl fmt::Debug for FuncImpl {
    fn fmt(&self, out: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Self::Builtin(_) => write!(out, "(builtin function)"),
            Self::UserDefined(_) => {
                write!(out, "(user-defined function)")
            }
        }
    }
}

trait Functions {
    fn builtin_fn(
        &mut self,
        name: Name,
        args: FormalArgs,
        body: Arc<BuiltinFn>,
    );
}
impl Functions for Scope {
    fn builtin_fn(
        &mut self,
        name: Name,
        args: FormalArgs,
        body: Arc<BuiltinFn>,
    ) {
        let f = Function::builtin(&self.get_name(), &name, args, body);
        self.define_function(name, f);
    }
}

impl Function {
    /// Get a built-in function by name.
    pub fn get_builtin(name: &Name) -> Option<&'static Self> {
        FUNCTIONS.get(name).or_else(|| {
            // Builtin function names are caseless.
            let name = name.as_ref().to_lowercase();
            FUNCTIONS.get(&name.into())
        })
    }

    /// Create a new `Function` from a rust implementation.
    ///
    /// Note: This does not expose the function in any scope, it just
    /// creates it.
    pub fn builtin(
        module: &str,
        name: &Name,
        args: FormalArgs,
        body: Arc<BuiltinFn>,
    ) -> Self {
        let pos = SourcePos::mock_function(name, &args, module);
        Self {
            body: FuncImpl::Builtin(Builtin { args, pos, body }),
        }
    }

    /// Call the function from a given scope and with a given set of
    /// arguments.
    pub fn call(&self, call: Call) -> Result<Value, CallError> {
        let result = match self.body {
            FuncImpl::Builtin(ref builtin) => builtin.eval_value(call),
            FuncImpl::UserDefined(ref closure) => closure.eval_value(call),
        };
        // The result of a calc function is either a calc function call or an
        // "atomic" number, i.e. one that is _not_ marked as "calculated".
        if Some(self) == FUNCTIONS.get(&name!(calc)) {
            match result {
                Ok(Value::Numeric(v, _)) => Ok(Value::Numeric(v, false)),
                result => result,
            }
        } else {
            result.map(Value::into_calculated)
        }
    }
}

impl From<Closure> for Function {
    fn from(c: Closure) -> Self {
        Self {
            body: FuncImpl::UserDefined(c),
        }
    }
}

lazy_static! {
    static ref MODULES: BTreeMap<&'static str, Scope> = {
        let mut modules = BTreeMap::new();
        modules.insert("sass:color", color::create_module());
        modules.insert("sass:list", list::create_module());
        modules.insert("sass:map", map::create_module());
        modules.insert("sass:math", math::create_module());
        modules.insert("sass:meta", meta::create_module());
        modules.insert("sass:selector", selector::create_module());
        modules.insert("sass:string", string::create_module());
        modules
    };
}

/// Get a global module (e.g. `sass:math`) by name.
pub fn get_global_module(name: &str) -> Option<ScopeRef> {
    MODULES.get(name).map(ScopeRef::Builtin)
}

type FunctionMap = BTreeMap<Name, Function>;
impl Functions for FunctionMap {
    fn builtin_fn(
        &mut self,
        name: Name,
        args: FormalArgs,
        body: Arc<BuiltinFn>,
    ) {
        let f = Function::builtin("", &name, args, body);
        self.insert(name, f);
    }
}

lazy_static! {
    static ref FUNCTIONS: FunctionMap = {
        let mut f = BTreeMap::new();
        def!(f, if(condition, if_true, if_false), |s| {
            if Value::is_true(&s.get(name!(condition))?) {
                Ok(s.get(name!(if_true))?)
            } else {
                Ok(s.get(name!(if_false))?)
            }
        });
        color::expose(MODULES.get("sass:color").unwrap(), &mut f);
        list::expose(MODULES.get("sass:list").unwrap(), &mut f);
        map::expose(MODULES.get("sass:map").unwrap(), &mut f);
        math::expose(MODULES.get("sass:math").unwrap(), &mut f);
        meta::expose(MODULES.get("sass:meta").unwrap(), &mut f);
        selector::expose(MODULES.get("sass:selector").unwrap(), &mut f);
        string::expose(MODULES.get("sass:string").unwrap(), &mut f);
        f
    };
}

// argument helpers for the actual functions

trait CheckedArg<T> {
    fn named(self, name: Name) -> Result<T, CallError>;
}
impl<T, E: ToString> CheckedArg<T> for Result<T, E> {
    fn named(self, name: Name) -> Result<T, CallError> {
        self.map_err(|e| CallError::BadArgument(name, e.to_string()))
    }
}

fn expected_to<T: Into<Value>>(value: T, cond: &str) -> String {
    format!("Expected {} to {}.", value.into().introspect(), cond)
}

/// Convert an error for a specific argument to not mentioning the argument.
///
/// Ok results and other errors are returned unmodified.
/// This is used in some cases to make errors from rsass the same as
/// the expected errors for dart sass.
fn unnamed<T>(result: Result<T, CallError>) -> Result<T, CallError> {
    match result {
        Err(CallError::BadArgument(_name, msg)) => Err(CallError::msg(msg)),
        other => other,
    }
}

mod check {
    use super::{expected_to, is_not};
    use crate::css::Value;
    use crate::value::{Number, Numeric};

    pub fn int(v: Value) -> Result<i64, String> {
        Numeric::try_from(v)?
            .value
            .into_integer()
            .map_err(|v| is_not(&v, "an int"))
    }

    pub fn positive_int(v: Value) -> Result<i64, String> {
        let v = int(v)?;
        if v > 0 {
            Ok(v)
        } else {
            Err(format!("Must be greater than 0, was {v}."))
        }
    }

    pub fn unitless(v: Value) -> Result<Number, String> {
        let val = Numeric::try_from(v)?;
        if val.is_no_unit() {
            Ok(val.value)
        } else {
            Err(expected_to(val, "have no units"))
        }
    }
    pub fn unitless_int(v: Value) -> Result<i64, String> {
        unitless(v)?
            .into_integer()
            .map_err(|v| is_not(&v, "an int"))
    }
}

#[test]
fn test_rgb() -> Result<(), Box<dyn std::error::Error>> {
    use crate::parser::code_span;
    use crate::parser::formalargs::call_args;
    use crate::value::Rgba;
    let scope = ScopeRef::new_global(Default::default());
    assert_eq!(
        FUNCTIONS.get(&name!(rgb)).unwrap().call(
            call_args(code_span(b"(17, 0, 225)").borrow())
                .unwrap()
                .1
                .evaluate(scope)?
        )?,
        Value::Color(Rgba::from_rgb(17, 0, 225).into(), None)
    );
    Ok(())
}

#[test]
fn test_nth() {
    assert_eq!("foo", do_evaluate(&[("x", "foo, bar")], b"nth($x, 1);"))
}

#[cfg(test)]
use crate::variablescope::test::do_evaluate;
