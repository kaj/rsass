use crate::error::Error;
use crate::variablescope::Scope;
use crate::{css, sass};
use lazy_static::lazy_static;
use std::collections::BTreeMap;
use std::sync::Arc;
use std::{cmp, fmt};

#[macro_use]
mod macros;

mod colors_hsl;
mod colors_other;
mod colors_rgb;
mod introspection;
mod lists;
mod maps;
mod numbers;
mod selector;
mod strings;

pub fn get_builtin_function(name: &str) -> Option<&'static SassFunction> {
    let name = name.replace("-", "_");
    let name: &str = &name;
    FUNCTIONS.get(name)
}

type BuiltinFn =
    dyn Fn(&dyn Scope) -> Result<css::Value, Error> + Send + Sync;

/// A function that can be called from a sass value.
///
/// The function can be either "builtin" (implemented in rust) or
/// "user defined" (implemented in scss).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SassFunction {
    args: sass::FormalArgs,
    body: FuncImpl,
}

#[derive(Clone)]
pub enum FuncImpl {
    Builtin(Arc<BuiltinFn>),
    UserDefined(Vec<sass::Item>),
}

impl PartialOrd for FuncImpl {
    fn partial_cmp(&self, rhs: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(rhs))
    }
}
impl Ord for FuncImpl {
    fn cmp(&self, rhs: &Self) -> cmp::Ordering {
        match (self, rhs) {
            (&FuncImpl::Builtin(..), &FuncImpl::Builtin(..)) => {
                cmp::Ordering::Equal
            }
            (&FuncImpl::Builtin(..), &FuncImpl::UserDefined(..)) => {
                cmp::Ordering::Less
            }
            (&FuncImpl::UserDefined(..), &FuncImpl::Builtin(..)) => {
                cmp::Ordering::Greater
            }
            (
                &FuncImpl::UserDefined(ref a),
                &FuncImpl::UserDefined(ref b),
            ) => a.cmp(b),
        }
    }
}

impl cmp::PartialEq for FuncImpl {
    fn eq(&self, rhs: &FuncImpl) -> bool {
        match (self, rhs) {
            (
                &FuncImpl::UserDefined(ref a),
                &FuncImpl::UserDefined(ref b),
            ) => a == b,
            (&FuncImpl::Builtin(ref a), &FuncImpl::Builtin(ref b)) => {
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
            FuncImpl::UserDefined(_) => {
                write!(out, "(user-defined function)")
            }
        }
    }
}

impl SassFunction {
    /// Create a new `SassFunction` from a rust implementation.
    pub fn builtin(
        args: Vec<(String, sass::Value)>,
        is_varargs: bool,
        body: Arc<BuiltinFn>,
    ) -> Self {
        SassFunction {
            args: sass::FormalArgs::new(args, is_varargs),
            body: FuncImpl::Builtin(body),
        }
    }

    /// Create a new `SassFunction` from a scss implementation.
    pub fn new(args: sass::FormalArgs, body: Vec<sass::Item>) -> Self {
        SassFunction {
            args,
            body: FuncImpl::UserDefined(body),
        }
    }

    /// Call the function from a given scope and with a given set of
    /// arguments.
    pub fn call(
        &self,
        scope: &dyn Scope,
        args: &css::CallArgs,
    ) -> Result<css::Value, Error> {
        let mut s = self.args.eval(scope, args)?;
        match self.body {
            FuncImpl::Builtin(ref body) => body(&s),
            FuncImpl::UserDefined(ref body) => {
                Ok(s.eval_body(body)?.unwrap_or(css::Value::Null))
            }
        }
    }
}

lazy_static! {
    static ref FUNCTIONS: BTreeMap<&'static str, SassFunction> = {
        let mut f = BTreeMap::new();
        def!(f, if(condition, if_true, if_false), |s| {
            if s.get("condition")?.is_true() {
                Ok(s.get("if_true")?)
            } else {
                Ok(s.get("if_false")?)
            }
        });
        colors_hsl::register(&mut f);
        colors_rgb::register(&mut f);
        colors_other::register(&mut f);
        introspection::register(&mut f);
        selector::register(&mut f);
        strings::register(&mut f);
        numbers::register(&mut f);
        lists::register(&mut f);
        maps::register(&mut f);
        f
    };
}

fn make_call(name: &str, args: Vec<css::Value>) -> css::Value {
    css::Value::Call(
        name.into(),
        css::CallArgs::new(
            args.into_iter()
                .filter(|v| v != &css::Value::Null)
                .map(|v| (None, v))
                .collect(),
        ),
    )
}

#[test]
fn test_rgb() -> Result<(), Box<dyn std::error::Error>> {
    use crate::parser::formalargs::call_args;
    use crate::test_span;
    use crate::value::Rgba;
    use crate::variablescope::GlobalScope;
    let scope = GlobalScope::new(Default::default());
    assert_eq!(
        FUNCTIONS.get("rgb").unwrap().call(
            &scope,
            &call_args(test_span!(b"(17, 0, 225)"))?
                .1
                .evaluate(&scope, true)?
        )?,
        css::Value::Color(Rgba::from_rgb(17, 0, 225), None)
    );
    Ok(())
}

#[test]
fn test_nth() {
    assert_eq!("foo", do_evaluate(&[("x", "foo, bar")], b"nth($x, 1);"))
}

#[cfg(test)]
use super::variablescope::test::do_evaluate;
