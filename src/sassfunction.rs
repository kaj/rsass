use formalargs::{CallArgs, FormalArgs};
use num_rational::Rational;
use std::collections::BTreeMap;
use valueexpression::{Unit, Value};
use variablescope::Scope;

pub fn get_function(name: &str) -> Option<&SassFunction> {
    let name = name.to_string().replace("-", "_");
    FUNCTIONS.get(&name)
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

macro_rules! one_arg {
    ($name:ident) => {
        (stringify!($name).into(), None)
    };
    ($name:ident = $value:expr) => {{
        use valueexpression::value_expression;
        (stringify!($name).into(), Some(value_expression($value).unwrap().1))
    }};
}

macro_rules! func {
    (( $($arg:ident $( = $value:expr )* ),* ), $body:expr) => {
        SassFunction {
            args: FormalArgs::new(vec![ $( one_arg!($arg $( = $value)* ) ),* ]),
            body: Box::new($body),
        }
    };
}

lazy_static! {
    static ref FUNCTIONS: BTreeMap<String, SassFunction> = {
        let mut f = BTreeMap::new();
        f.insert("rgb".into(), func!((red, green, blue), |s| {
            Value::Color(s.get("red").map(to_int).unwrap_or(0),
                         s.get("green").map(to_int).unwrap_or(0),
                         s.get("blue").map(to_int).unwrap_or(0),
                         Rational::from_integer(1),
                         None)
        }));
        f.insert("rgba".into(), func!((red, green, blue, alpha), |s: &Scope| {
            let red = s.get("red");
            let alpha = s.get("alpha");
            if let Some(Value::Color(r, g, b, _, _)) = red {
                let a = alpha.or_else(|| s.get("green"))
                    .map(to_rational)
                    .unwrap_or(Rational::from_integer(1));
                Value::Color(r, g, b, a, None)
            } else {
                Value::Color(red.map(to_int).unwrap_or(0),
                             s.get("green").map(to_int).unwrap_or(0),
                             s.get("blue").map(to_int).unwrap_or(0),
                             alpha.map(to_rational)
                             .unwrap_or(Rational::from_integer(1)),
                             None)
            }
        }));
        f.insert("red".into(), func!((color), |s: &Scope| {
            match s.get("color") {
                Some(Value::Color(red, _, _, _, _)) => {
                    Value::Numeric(b2rat(red), Unit::None, true)
                }
                Some(value) => value,
                None => Value::Literal("".into()),
            }
        }));
        f.insert("green".into(), func!((color), |s: &Scope| {
            match s.get("color") {
                Some(Value::Color(_, green, _, _, _)) => {
                    Value::Numeric(b2rat(green), Unit::None, true)
                }
                Some(value) => value,
                None => Value::Literal("".into()),
            }
        }));
        f.insert("blue".into(), func!((color), |s: &Scope| {
            match s.get("color") {
                Some(Value::Color(_, _, blue, _, _)) => {
                    Value::Numeric(b2rat(blue), Unit::None, true)
                }
                Some(value) => value,
                None => Value::Literal("".into()),
            }
        }));
        f.insert("invert".into(), func!((color), |s: &Scope| {
            let color = s.get("color");
            if let &Some(Value::Color(ref r, ref g, ref b, ref a, _)) =
                &color {
                    Value::Color(0xff - r, 0xff - g, 0xff - b, *a, None)
                } else {
                    panic!(format!("Unexpected arguments to invert: ({:?})",
                                   color))
                }
        }));
        f.insert("mix".into(), func!((color1, color2, weight=b"50%"), |s| {
            let color1 = s.get("color1");
            let color2 = s.get("color2");
            let weight = s.get("weight");
            if let (&Some(Value::Color(ref r1, ref g1, ref b1, ref a1, _)),
                    &Some(Value::Color(ref r2, ref g2, ref b2, ref a2, _)),
                    &Some(Value::Numeric(ref w, ref wu, _))) =
                (&color1, &color2, &weight) {
                let w = if wu == &Unit::Percent {
                    w / Rational::from_integer(100)
                } else {
                    w.clone()
                };
                let one = Rational::from_integer(1);
                let w2 = one - (one - w * a1) * a2;
                fn mv(v1: u8, v2: u8, w: Rational) -> Rational {
                    mr(Rational::from_integer(v1 as isize),
                       Rational::from_integer(v2 as isize),
                       w)
                }
                fn mr(v1: Rational, v2: Rational, w: Rational) -> Rational {
                    v1 * w + v2 * (Rational::from_integer(1) - w)
                }
                fn rb(r: Rational) -> u8 {
                    (r + Rational::new(1, 2)).to_integer() as u8
                }
                Value::Color(rb(mv(*r1, *r2, w2)),
                             rb(mv(*g1, *g2, w2)),
                             rb(mv(*b1, *b2, w2)),
                             mr(*a1, *a2, w),
                             None)
            } else {
                panic!(format!("Unexpected arguments to mix: \
                                ({:?}, {:?}, {:?})",
                               color1,
                               color2,
                               weight))
            }
        }));
        f.insert("type_of".into(), func!((value), |s: &Scope| {
            let value = s.get("value");
            Value::Literal(match value {
                Some(Value::Color(..)) => "color",
                Some(Value::Literal(..)) => "string",
                Some(Value::Numeric(..)) => "number",
                _ => "unknown",
            }.into())
        }));
        f.insert("if".into(), func!((condition, if_true, if_false), |s| {
            if s.get("condition").map(|v| v.is_true()).unwrap_or(false) {
                s.get("if_true").unwrap()
            } else {
                s.get("if_false").unwrap()
            }
        }));
        f
    };
}

fn b2rat(byte: u8) -> Rational {
    Rational::from_integer(byte as isize)
}

fn to_int(v: Value) -> u8 {
    match v {
        Value::Numeric(v, _, _) => v.to_integer() as u8,
        v => format!("{}", v).parse().unwrap(),
    }
}

fn to_rational(v: Value) -> Rational {
    match v {
        Value::Numeric(v, _, _) => v,
        v => panic!("Expected rational, got {:?}", v),
    }
}

#[test]
fn test_rgb() {
    use formalargs::call_args;
    use variablescope::ScopeImpl;
    assert_eq!(Value::Color(17, 0, 225, Rational::from_integer(1), None),
               FUNCTIONS.get("rgb")
                   .unwrap()
                   .call(&mut ScopeImpl::new(),
                         &call_args(b"(17, 0, 225)").unwrap().1))
}
