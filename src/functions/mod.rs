use super::SassItem;
use formalargs::{CallArgs, FormalArgs};
use std::{cmp, fmt};
use std::collections::BTreeMap;
use std::sync::Arc;
use valueexpression::Value;
use variablescope::Scope;

#[macro_use]
mod macros;

mod colors_rgb;
mod colors_hsl;
mod colors_other;
mod introspection;
mod numbers;
mod strings;

pub fn get_builtin_function(name: &str) -> Option<&'static SassFunction> {
    let name = name.replace("-", "_");
    let name: &str = &name;
    FUNCTIONS.get(name)
}

type BuiltinFn = Fn(&Scope) -> Result<Value, Error> + Send + Sync;

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
    pub fn builtin(args: Vec<(String, Value)>, body: Arc<BuiltinFn>) -> Self {
        SassFunction {
            args: FormalArgs::new(args),
            body: FuncImpl::Builtin(body),
        }
    }
    pub fn new(args: FormalArgs, body: Vec<SassItem>) -> Self {
        SassFunction { args: args, body: FuncImpl::UserDefined(body) }
    }

    pub fn call(&self,
                scope: &mut Scope,
                args: &CallArgs)
                -> Result<Value, Error> {
        let mut s = self.args.eval(scope, args);
        match self.body {
            FuncImpl::Builtin(ref body) => body(&s),
            FuncImpl::UserDefined(ref body) => {
                Ok(s.eval_body(body).unwrap_or(Value::Null))
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    BadArguments(String),
}

fn badarg(expected: &str, actual: &Value) -> Error {
    Error::BadArguments(format!("expected {}, got {} = {}",
                                expected,
                                actual.type_name(),
                                actual))
}

fn badargs(expected: &[&str], actual: &[&Value]) -> Error {
    // TODO Better message!
    Error::BadArguments(format!("expected {:?}, got {:?}", expected, actual))
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
        def!(f, nth(list, n), |s| {
            let n = match s.get("n") {
                Value::Numeric(val, _, _) if val.denom() == &1 => {
                    val.to_integer()
                }
                x => return Err(badarg("integer", &x))
            };
            let list = match s.get("list") {
                Value::MultiComma(v) => v,
                Value::MultiSpace(v) => v,
                v => return Err(badarg("list", &v)),
            };
            Ok(list[n as usize - 1].clone())
        });
        colors_hsl::register(&mut f);
        colors_rgb::register(&mut f);
        colors_other::register(&mut f);
        introspection::register(&mut f);
        strings::register(&mut f);
        numbers::register(&mut f);
        f
    };
}

#[test]
fn test_rgb() {
    use formalargs::call_args;
    use num_rational::Rational;
    use num_traits::{One, Zero};
    use variablescope::ScopeImpl;
    assert_eq!(Ok(Value::Color(Rational::new(17, 1),
                               Rational::zero(),
                               Rational::new(225, 1),
                               Rational::one(),
                               None)),
               FUNCTIONS
                   .get("rgb")
                   .unwrap()
                   .call(&mut ScopeImpl::new(),
                         &call_args(b"(17, 0, 225)").unwrap().1))
}

#[test]
fn test_nth() {
    assert_eq!("foo", do_evaluate(&[("x", "foo, bar")], b"nth($x, 1);"))
}

#[cfg(test)]
use super::variablescope::test::do_evaluate;
