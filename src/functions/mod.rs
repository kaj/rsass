use formalargs::{CallArgs, FormalArgs};
use std::collections::BTreeMap;
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

pub fn get_function(name: &str) -> Option<&SassFunction> {
    let name = name.replace("-", "_");
    let name: &str = &name;
    FUNCTIONS.get(name)
}

pub struct SassFunction {
    args: FormalArgs,
    body: Box<Fn(&Scope) -> Result<Value, Error> + Send + Sync>,
}

impl SassFunction {
    pub fn call(&self,
                scope: &mut Scope,
                args: &CallArgs)
                -> Result<Value, Error> {
        let s = self.args.eval(scope, args);
        (self.body)(&s)
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
        f.insert("if", func!((condition, if_true, if_false), |s| {
            if s.get("condition").is_true() {
                Ok(s.get("if_true"))
            } else {
                Ok(s.get("if_false"))
            }
        }));
        f.insert("nth", func!((list, n), |s| {
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
        }));
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
               FUNCTIONS.get("rgb")
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
