use formalargs::{CallArgs, FormalArgs, formal_args};
use num_rational::Rational;
use valueexpression::{Unit, Value};
use variablescope::Scope;

pub fn get_function(name: &str) -> Option<SassFunction> {
    match name {
        "rgb" => Some(rgb()),
        "rgba" => Some(rgba()),
        "red" => Some(red()),
        "green" => Some(green()),
        "blue" => Some(blue()),
        "mix" => Some(mix()),
        "invert" => Some(invert()),
        "type-of" => Some(type_of()),
        "type_of" => Some(type_of()),
        "if" => Some(if_function()),
        _ => None,
    }
}

pub struct SassFunction {
    args: FormalArgs,
    body: Box<Fn(&Scope) -> Value>,
}

impl SassFunction {
    pub fn call(&self, scope: &mut Scope, args: &CallArgs) -> Value {
        let s = self.args.eval(scope, args);
        (self.body)(&s)
    }
}

fn rgb() -> SassFunction {
    SassFunction {
        args: formal_args(b"($red, $green, $blue)")
            .unwrap()
            .1,
        body: Box::new(|s: &Scope| {
            Value::HexColor(s.get("red").map(to_int).unwrap_or(0),
                            s.get("green").map(to_int).unwrap_or(0),
                            s.get("blue").map(to_int).unwrap_or(0),
                            Rational::from_integer(1),
                            None)
        }),
    }
}

fn red() -> SassFunction {
    SassFunction {
        args: formal_args(b"($c)")
            .unwrap()
            .1,
        body: Box::new(|s: &Scope| {
            match s.get("c") {
                Some(Value::HexColor(red, _, _, _, _)) => {
                    Value::Numeric(b2rat(red), Unit::None, true)
                }
                Some(value) => value,
                None => Value::Literal("".into()),
            }
        }),
    }
}

fn green() -> SassFunction {
    SassFunction {
        args: formal_args(b"($c)")
            .unwrap()
            .1,
        body: Box::new(|s: &Scope| {
            match s.get("c") {
                Some(Value::HexColor(_, green, _, _, _)) => {
                    Value::Numeric(b2rat(green), Unit::None, true)
                }
                Some(value) => value,
                None => Value::Literal("".into()),
            }
        }),
    }
}

fn blue() -> SassFunction {
    SassFunction {
        args: formal_args(b"($c)")
            .unwrap()
            .1,
        body: Box::new(|s: &Scope| {
            match s.get("c") {
                Some(Value::HexColor(_, _, blue, _, _)) => {
                    Value::Numeric(b2rat(blue), Unit::None, true)
                }
                Some(value) => value,
                None => Value::Literal("".into()),
            }
        }),
    }
}

fn b2rat(byte: u8) -> Rational {
    Rational::from_integer(byte as isize)
}

fn rgba() -> SassFunction {
    SassFunction {
        args: formal_args(b"($red, $green, $blue, $alpha)")
            .unwrap()
            .1,
        body: Box::new(|s: &Scope| {
            let red = s.get("red");
            let alpha = s.get("alpha");
            if let Some(Value::HexColor(r, g, b, _, _)) = red {
                Value::HexColor(r,
                                g,
                                b,
                                alpha.or_else(|| s.get("green"))
                                    .map(to_rational)
                                    .unwrap_or(Rational::from_integer(1)),
                                None)
            } else {
                Value::HexColor(red.map(to_int).unwrap_or(0),
                                s.get("green").map(to_int).unwrap_or(0),
                                s.get("blue").map(to_int).unwrap_or(0),
                                alpha.map(to_rational)
                                    .unwrap_or(Rational::from_integer(1)),
                                None)
            }
        }),
    }
}

fn invert() -> SassFunction {
    SassFunction {
        args: formal_args(b"($color)")
            .unwrap()
            .1,
        body: Box::new(|s: &Scope| {
            let color = s.get("color");
            if let &Some(Value::HexColor(ref r, ref g, ref b, ref a, _)) =
                &color {
                Value::HexColor(0xff - r, 0xff - g, 0xff - b, *a, None)
            } else {
                panic!(format!("Unexpected arguments to invert: ({:?})", color))
            }
        }),
    }
}

fn mix() -> SassFunction {
    SassFunction {
        args: formal_args(b"($color1, $color2, $weight:50%)")
            .unwrap()
            .1,
        body: Box::new(|s: &Scope| {
            let color1 = s.get("color1");
            let color2 = s.get("color2");
            let weight = s.get("weight");
            if let (&Some(Value::HexColor(ref r1,
                                          ref g1,
                                          ref b1,
                                          ref a1,
                                          _)),
                    &Some(Value::HexColor(ref r2,
                                          ref g2,
                                          ref b2,
                                          ref a2,
                                          _)),
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
                Value::HexColor(rb(mv(*r1, *r2, w2)),
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
        }),
    }
}

fn type_of() -> SassFunction {
    SassFunction {
        args: formal_args(b"($value)")
            .unwrap()
            .1,
        body: Box::new(|s: &Scope| {
            let value = s.get("value");
            Value::Literal(match value {
                    Some(Value::HexColor(..)) => "color",
                    Some(Value::Literal(..)) => "string",
                    Some(Value::Numeric(..)) => "number",
                    _ => "unknown",
                }
                .into())
        }),
    }
}

fn if_function() -> SassFunction {
    SassFunction {
        args: formal_args(b"($condition, $if-true, $if-false)")
            .unwrap()
            .1,
        body: Box::new(|s: &Scope| {
            if s.get("condition").map(|v| v.is_true()).unwrap_or(false) {
                s.get("if-true").unwrap()
            } else {
                s.get("if-false").unwrap()
            }
        }),
    }
}

#[test]
fn test_rgb() {
    use formalargs::call_args;
    use variablescope::ScopeImpl;
    assert_eq!(Value::HexColor(17, 0, 225, Rational::from_integer(1), None),
               rgb().call(&mut ScopeImpl::new(),
                          &call_args(b"(17, 0, 225)").unwrap().1))
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
