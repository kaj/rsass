use crate::error::Error;
use crate::sass::Name;
use crate::{css, sass, Scope, ScopeRef};
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

pub fn get_builtin_function(name: &Name) -> Option<&'static SassFunction> {
    FUNCTIONS.get(name)
}

type BuiltinFn = dyn Fn(&Scope) -> Result<css::Value, Error> + Send + Sync;

/// A function that can be called from a sass value.
///
/// The function can be either "builtin" (implemented in rust) or
/// "user defined" (implemented in scss).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct SassFunction {
    args: sass::FormalArgs,
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

impl SassFunction {
    /// Create a new `SassFunction` from a rust implementation.
    pub fn builtin(
        args: Vec<(Name, sass::Value)>,
        is_varargs: bool,
        body: Arc<BuiltinFn>,
    ) -> Self {
        SassFunction {
            args: sass::FormalArgs::new(args, is_varargs),
            body: FuncImpl::Builtin(body),
        }
    }

    /// Create a new `SassFunction` from a scss implementation.
    ///
    /// The scope is where the function is defined, used to bind any
    /// non-parameter names in the body.
    pub fn closure(
        args: sass::FormalArgs,
        scope: ScopeRef,
        body: Vec<sass::Item>,
    ) -> Self {
        SassFunction {
            args,
            body: FuncImpl::UserDefined(scope, body),
        }
    }

    /// Call the function from a given scope and with a given set of
    /// arguments.
    pub fn call(
        &self,
        callscope: ScopeRef,
        args: &css::CallArgs,
    ) -> Result<css::Value, Error> {
        let cs = Name::from_static("%%CALLING_SCOPE%%");
        match self.body {
            FuncImpl::Builtin(ref body) => {
                let s = self.args.eval(
                    Scope::new_global_ref(callscope.get_format()),
                    args,
                )?;
                s.define_module(cs, callscope);
                body(&s)
            }
            FuncImpl::UserDefined(ref defscope, ref body) => {
                let s = self.args.eval(defscope.clone(), args)?;
                s.define_module(cs, callscope);
                Ok(s.eval_body(body)?.unwrap_or(css::Value::Null))
            }
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

pub fn get_global_module(name: &str) -> Option<ScopeRef> {
    MODULES.get(name).map(ScopeRef::Builtin)
}

type FunctionMap = BTreeMap<Name, SassFunction>;

lazy_static! {
    static ref FUNCTIONS: FunctionMap = {
        let mut f = BTreeMap::new();
        f.insert(
            name!(if),
            func!((condition, if_true, if_false), |s| {
                if s.get("condition")?.is_true() {
                    Ok(s.get("if_true")?)
                } else {
                    Ok(s.get("if_false")?)
                }
            }),
        );
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

#[test]
fn test_rgb() -> Result<(), Box<dyn std::error::Error>> {
    use crate::parser::code_span;
    use crate::parser::formalargs::call_args;
    use crate::value::Rgba;
    let scope = Scope::new_global_ref(Default::default());
    assert_eq!(
        FUNCTIONS.get(&name!(rgb)).unwrap().call(
            scope.clone(),
            &call_args(code_span(b"(17, 0, 225)"))?
                .1
                .evaluate(scope, true)?
        )?,
        css::Value::Color(Rgba::from_rgb(17, 0, 225).into(), None)
    );
    Ok(())
}

#[test]
fn test_nth() {
    assert_eq!("foo", do_evaluate(&[("x", "foo, bar")], b"nth($x, 1);"))
}

#[cfg(test)]
use super::variablescope::test::do_evaluate;
