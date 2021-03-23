use crate::css::{CallArgs, Value};
use crate::error::Error;
use crate::parser::SourcePos;
use crate::sass::{FormalArgs, Name};
use crate::value::{Numeric, Quotes};
use crate::{sass, Scope, ScopeRef};
use lazy_static::lazy_static;
use std::collections::BTreeMap;
use std::sync::Arc;
use std::{cmp, fmt};

#[macro_use]
mod macros;

mod color;
mod list;
mod map;
mod math;
mod meta;
mod selector;
mod string;

type BuiltinFn = dyn Fn(&Scope) -> Result<Value, Error> + Send + Sync;

/// A function that can be called from a sass value.
///
/// The function can be either "builtin" (implemented in rust) or
/// "user defined" (implemented in scss).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct Function {
    args: FormalArgs,
    pos: SourcePos,
    body: FuncImpl,
}

#[derive(Clone)]
pub enum FuncImpl {
    Builtin(Arc<BuiltinFn>),
    /// A user-defined function is really a closure, it has a scope
    /// where it is defined and a body of items.
    UserDefined(ScopeRef, Vec<sass::Item>),
}

impl PartialOrd for FuncImpl {
    fn partial_cmp(&self, rhs: &Self) -> Option<cmp::Ordering> {
        match (self, rhs) {
            (&FuncImpl::Builtin(..), &FuncImpl::Builtin(..)) => None,
            (&FuncImpl::Builtin(..), &FuncImpl::UserDefined(..)) => {
                Some(cmp::Ordering::Less)
            }
            (&FuncImpl::UserDefined(..), &FuncImpl::Builtin(..)) => {
                Some(cmp::Ordering::Greater)
            }
            (
                &FuncImpl::UserDefined(ref _sa, ref a),
                &FuncImpl::UserDefined(ref _sb, ref b),
            ) => a.partial_cmp(b),
        }
    }
}

impl cmp::PartialEq for FuncImpl {
    fn eq(&self, rhs: &FuncImpl) -> bool {
        match (self, rhs) {
            (
                &FuncImpl::UserDefined(ref sa, ref a),
                &FuncImpl::UserDefined(ref sb, ref b),
            ) => ScopeRef::is_same(sa, sb) && a == b,
            (&FuncImpl::Builtin(ref a), &FuncImpl::Builtin(ref b)) => {
                // Each builtin function is only created once, so this
                // should be ok.
                #[allow(clippy::vtable_address_comparisons)]
                Arc::ptr_eq(a, b)
            }
            _ => false,
        }
    }
}
impl cmp::Eq for FuncImpl {}

impl fmt::Debug for FuncImpl {
    fn fmt(&self, out: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            FuncImpl::Builtin(_) => write!(out, "(builtin function)"),
            FuncImpl::UserDefined(..) => {
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
    pub fn get_builtin(name: &Name) -> Option<&'static Function> {
        FUNCTIONS.get(name)
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
        Function {
            args,
            pos,
            body: FuncImpl::Builtin(body),
        }
    }

    /// Create a new `Function` from a scss implementation.
    ///
    /// The scope is where the function is defined, used to bind any
    /// non-parameter names in the body.
    pub fn closure(
        args: FormalArgs,
        pos: SourcePos,
        scope: ScopeRef,
        body: Vec<sass::Item>,
    ) -> Self {
        Function {
            args,
            pos,
            body: FuncImpl::UserDefined(scope, body),
        }
    }

    /// Call the function from a given scope and with a given set of
    /// arguments.
    pub fn call(
        &self,
        callscope: ScopeRef,
        args: &CallArgs,
    ) -> Result<Value, Error> {
        let cs = "%%CALLING_SCOPE%%";
        match self.body {
            FuncImpl::Builtin(ref body) => {
                let s = self.do_eval_args(
                    ScopeRef::new_global(callscope.get_format()),
                    args,
                )?;
                s.define_module(cs.into(), callscope);
                body(&s)
            }
            FuncImpl::UserDefined(ref defscope, ref body) => {
                let s = self.do_eval_args(defscope.clone(), args)?;
                s.define_module(cs.into(), callscope);
                Ok(s.eval_body(body)?.unwrap_or(Value::Null))
            }
        }
    }

    fn do_eval_args(
        &self,
        def: ScopeRef,
        args: &CallArgs,
    ) -> Result<ScopeRef, Error> {
        self.args.eval(def, args).map_err(|e| match e {
            sass::ArgsError::Eval(e) => e,
            ae => Error::BadArguments(ae.to_string(), self.pos.clone()),
        })
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
            if s.get("condition")?.is_true() {
                Ok(s.get("if_true")?)
            } else {
                Ok(s.get("if_false")?)
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
    fn named(self, name: Name) -> Result<T, Error>;
}
impl<T> CheckedArg<T> for Result<T, String> {
    fn named(self, name: Name) -> Result<T, Error> {
        self.map_err(|e| Error::BadArgument(name, e))
    }
}

fn get_checked<T, F>(s: &Scope, name: Name, check: F) -> Result<T, Error>
where
    F: Fn(Value) -> Result<T, String>,
{
    check(s.get(name.as_ref())?).named(name)
}

fn get_opt_check<T, F>(
    s: &Scope,
    name: Name,
    check: F,
) -> Result<Option<T>, Error>
where
    F: Fn(Value) -> Result<T, String>,
{
    match s.get(name.as_ref())? {
        Value::Null => Ok(None),
        v => check(v).named(name).map(Some),
    }
}

fn get_numeric(s: &Scope, name: &str) -> Result<Numeric, Error> {
    get_checked(s, name.into(), check::numeric)
}

fn get_integer(s: &Scope, name: Name) -> Result<i64, Error> {
    get_checked(s, name, check::unitless_int)
}

fn get_string(
    s: &Scope,
    name: &'static str,
) -> Result<(String, Quotes), Error> {
    get_checked(s, name.into(), check::string)
}

mod check {
    use crate::css::Value;
    use crate::output::Format;
    use crate::value::{Number, Numeric, Quotes};

    pub fn numeric(v: Value) -> Result<Numeric, String> {
        v.numeric_value().map_err(|v| {
            format!("{} is not a number", v.format(Format::introspect()))
        })
    }
    pub fn int(v: Value) -> Result<i64, String> {
        numeric(v)?.value.into_integer().map_err(|v| {
            format!("{} is not an int", v.format(Format::introspect()))
        })
    }

    pub fn unitless(v: Value) -> Result<Number, String> {
        let val = numeric(v)?;
        if val.is_no_unit() {
            Ok(val.value)
        } else {
            Err(format!(
                "Expected {} to have no units",
                val.format(Format::introspect())
            ))
        }
    }
    pub fn unitless_int(v: Value) -> Result<i64, String> {
        let v0 = unitless(v)?;
        v0.into_integer().map_err(|v| {
            format!("{} is not an int", v.format(Format::introspect()))
        })
    }

    pub fn string(v: Value) -> Result<(String, Quotes), String> {
        match v {
            Value::Literal(s, q) => Ok((s, q)),
            v => Err(format!(
                "{} is not a string",
                v.format(Format::introspect())
            )),
        }
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
            scope.clone(),
            &call_args(code_span(b"(17, 0, 225)"))?
                .1
                .evaluate(scope, true)?
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
