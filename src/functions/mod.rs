use formalargs::{CallArgs, FormalArgs};
use std::collections::BTreeMap;
use valueexpression::{Quotes, Value};
use variablescope::Scope;

#[macro_use]
mod macros;

mod colors_rgb;
mod colors_hsl;
mod colors_other;
mod numbers;
mod strings;

pub fn get_function(name: &str) -> Option<&SassFunction> {
    let name = name.replace("-", "_");
    let name: &str = &name;
    FUNCTIONS.get(name)
}

pub struct SassFunction {
    args: FormalArgs,
    body: Box<Fn(&Scope) -> Value + Send + Sync>,
}

impl SassFunction {
    pub fn call(&self, scope: &mut Scope, args: &CallArgs) -> Value {
        let s = self.args.eval(scope, args);
        (self.body)(&s)
    }
}

lazy_static! {
    static ref FUNCTIONS: BTreeMap<&'static str, SassFunction> = {
        let mut f = BTreeMap::new();
        f.insert("type_of", func!((value), |s: &Scope| {
            let value = s.get("value");
            Value::Literal(
                match value {
                    Value::Color(..) => "color",
                    Value::Literal(..) => "string",
                    Value::Numeric(..) => "number",
                    _ => "unknown",
                }.into(),
                Quotes::None)
        }));
        f.insert("if", func!((condition, if_true, if_false), |s| {
            if s.get("condition").is_true() {
                s.get("if_true")
            } else {
                s.get("if_false")
            }
        }));
        f.insert("nth", func!((list, n), |s| {
            let n = if let Value::Numeric(val, _, _) = s.get("n") {
                assert!(*val.denom() == 1);
                val.to_integer()
            } else {
                panic!("n argument must be integer")
            };
            let list = match s.get("list") {
                Value::MultiComma(v) => v,
                Value::MultiSpace(v) => v,
                v => panic!("list argument must be list, was {}", v),
            };
            list[n as usize - 1].clone()
        }));
        colors_hsl::register(&mut f);
        colors_rgb::register(&mut f);
        colors_other::register(&mut f);
        strings::register(&mut f);
        numbers::register(&mut f);
        f
    };
}

#[test]
fn test_rgb() {
    use formalargs::call_args;
    use num_rational::Rational;
    use variablescope::ScopeImpl;
    assert_eq!(Value::Color(17, 0, 225, Rational::from_integer(1), None),
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
