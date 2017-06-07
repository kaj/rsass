use error::Error;
use formalargs::{CallArgs, FormalArgs};
use sass_item::SassItem;
use std::{cmp, fmt};
use std::collections::BTreeMap;
use std::sync::Arc;
use value::Value;
use variablescope::Scope;

#[macro_use]
mod macros;

mod colors_rgb;
mod colors_hsl;
mod colors_other;
mod introspection;
mod numbers;
mod strings;
mod lists;

pub fn get_builtin_function(name: &str) -> Option<&'static SassFunction> {
    let name = name.replace("-", "_");
    let name: &str = &name;
    FUNCTIONS.get(name)
}

type BuiltinFn = Fn(&Scope) -> Result<Value, Error> + Send + Sync;

/// A function that can be called from a sass value.
///
/// The function can be either "builtin" (implemented in rust) or
/// "user defined" (implemented in scss).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SassFunction {
    args: FormalArgs,
    body: FuncImpl,
}

#[derive(Clone)]
pub enum FuncImpl {
    Builtin(Arc<BuiltinFn>),
    UserDefined(Vec<SassItem>),
}

impl cmp::PartialEq for FuncImpl {
    fn eq(&self, rhs: &FuncImpl) -> bool {
        match (self, rhs) {
            (&FuncImpl::UserDefined(ref a), &FuncImpl::UserDefined(ref b)) => {
                a == b
            }
            // Note: Maybe consider builtins equal if same Arc?
            _ => false,
        }
    }
}
impl cmp::Eq for FuncImpl {}

impl fmt::Debug for FuncImpl {
    fn fmt(&self, out: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            FuncImpl::Builtin(_) => write!(out, "(builtin function)"),
            FuncImpl::UserDefined(_) => write!(out, "(user-defined function)"),
        }
    }
}

impl SassFunction {
    /// Create a new `SassFunction` from a rust implementation.
    pub fn builtin(args: Vec<(String, Value)>,
                   is_varargs: bool,
                   body: Arc<BuiltinFn>)
                   -> Self {
        SassFunction {
            args: FormalArgs::new(args, is_varargs),
            body: FuncImpl::Builtin(body),
        }
    }

    /// Create a new `SassFunction` from a scss implementation.
    pub fn new(args: FormalArgs, body: Vec<SassItem>) -> Self {
        SassFunction { args: args, body: FuncImpl::UserDefined(body) }
    }

    /// Call the function from a given scope and with a given set of
    /// arguments.
    pub fn call(&self, scope: &Scope, args: &CallArgs) -> Result<Value, Error> {
        let mut s = self.args.eval(scope, args);
        match self.body {
            FuncImpl::Builtin(ref body) => body(&s),
            FuncImpl::UserDefined(ref body) => {
                Ok(s.eval_body(body).unwrap_or(Value::Null))
            }
        }
    }
}

lazy_static! {
    static ref FUNCTIONS: BTreeMap<&'static str, SassFunction> = {
        let mut f = BTreeMap::new();
        def!(f, if(condition, if_true, if_false), |s| {
            if s.get("condition").is_true() {
                Ok(s.get("if_true"))
            } else {
                Ok(s.get("if_false"))
            }
        });
        colors_hsl::register(&mut f);
        colors_rgb::register(&mut f);
        colors_other::register(&mut f);
        introspection::register(&mut f);
        strings::register(&mut f);
        numbers::register(&mut f);
        lists::register(&mut f);
        f
    };
}

#[test]
fn test_rgb() {
    use parser::formalargs::call_args;
    use num_rational::Rational;
    use num_traits::{One, Zero};
    use variablescope::GlobalScope;
    assert_eq!(FUNCTIONS
                   .get("rgb")
                   .unwrap()
                   .call(&GlobalScope::new(),
                         &call_args(b"(17, 0, 225)").unwrap().1)
                   .unwrap(),
               Value::Color(Rational::new(17, 1),
                            Rational::zero(),
                            Rational::new(225, 1),
                            Rational::one(),
                            None))
}

#[test]
fn test_nth() {
    assert_eq!("foo", do_evaluate(&[("x", "foo, bar")], b"nth($x, 1);"))
}

#[cfg(test)]
use super::variablescope::test::do_evaluate;
