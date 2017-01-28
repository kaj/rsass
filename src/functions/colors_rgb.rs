use super::{Error, SassFunction, badarg, badargs};
use formalargs::FormalArgs;
use num_rational::Rational;
use num_traits::One;
use std::collections::BTreeMap;
use valueexpression::{Unit, Value};
use variablescope::Scope;

pub fn register(f: &mut BTreeMap<&'static str, SassFunction>) {
    f.insert("rgb",
             func!((red, green, blue), |s| {
        Ok(Value::Color(to_int(s.get("red")),
                        to_int(s.get("green")),
                        to_int(s.get("blue")),
                        Rational::one(),
                        None))
    }));
    f.insert("rgba",
             func!((red, green, blue, alpha), |s: &Scope| {
        let red = s.get("red");
        let alpha = s.get("alpha");
        if let Value::Color(r, g, b, _, _) = red {
            let a = if alpha != Value::Null {
                alpha
            } else {
                s.get("green")
            };
            let a = try!(to_rational(a));
            Ok(Value::Color(r, g, b, a, None))
        } else {
            Ok(Value::Color(to_int(red),
                            to_int(s.get("green")),
                            to_int(s.get("blue")),
                            try!(to_rational(alpha)),
                            None))
        }
    }));
    f.insert("red",
             func!((color), |s: &Scope| {
        match &s.get("color") {
            &Value::Color(ref red, _, _, _, _) => {
                Ok(Value::Numeric(b2rat(*red), Unit::None, true))
            }
            value => Err(badarg("color", value)),
        }
    }));
    f.insert("green",
             func!((color), |s: &Scope| {
        match &s.get("color") {
            &Value::Color(_, ref green, _, _, _) => {
                Ok(Value::Numeric(b2rat(*green), Unit::None, true))
            }
            value => Err(badarg("color", value)),
        }
    }));
    f.insert("blue",
             func!((color), |s: &Scope| {
        match &s.get("color") {
            &Value::Color(_, _, ref blue, _, _) => {
                Ok(Value::Numeric(b2rat(*blue), Unit::None, true))
            }
            value => Err(badarg("color", value)),
        }
    }));
    f.insert("mix",
             func!((color1, color2, weight = b"50%"), |s| {
        let color1 = s.get("color1");
        let color2 = s.get("color2");
        let weight = s.get("weight");
        if let (&Value::Color(ref r1, ref g1, ref b1, ref a1, _),
                &Value::Color(ref r2, ref g2, ref b2, ref a2, _),
                &Value::Numeric(ref w, ref wu, _)) =
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
                cap_u8(r.round().to_integer())
            }
            Ok(Value::Color(rb(mv(*r1, *r2, w2)),
                            rb(mv(*g1, *g2, w2)),
                            rb(mv(*b1, *b2, w2)),
                            mr(*a1, *a2, w),
                            None))
        } else {
            Err(badargs(&["color", "color", "number"],
                        &[&color1, &color2, &weight]))
        }
    }));
    f.insert("invert",
             func!((color), |s: &Scope| {
        match &s.get("color") {
            &Value::Color(ref r, ref g, ref b, ref a, _) => {
                Ok(Value::Color(0xff - r, 0xff - g, 0xff - b, *a, None))
            }
            value => Err(badarg("color", value)),
        }
    }));
}

fn b2rat(byte: u8) -> Rational {
    Rational::from_integer(byte as isize)
}

fn to_int(v: Value) -> u8 {
    cap_u8(match v {
        Value::Numeric(v, Unit::Percent, _) => {
            (Rational::new(255, 100) * v).to_integer()
        }
        Value::Numeric(v, _, _) => v.round().to_integer(),
        v => format!("{}", v).parse().unwrap(),
    })
}

fn cap_u8(n: isize) -> u8 {
    if n > 255 {
        255
    } else if n < 0 {
        0
    } else {
        n as u8
    }
}

#[test]
fn percent_to_int_0() {
    assert_eq!(to_int(percent(0)), 0)
}
#[test]
fn percent_to_int_50() {
    assert_eq!(to_int(percent(50)), 127)
}
#[test]
fn percent_to_int_100() {
    assert_eq!(to_int(percent(100)), 255)
}
#[test]
fn percent_to_int_overflow() {
    assert_eq!(to_int(percent(130)), 255)
}

#[cfg(test)]
fn percent(n: isize) -> Value {
    Value::Numeric(Rational::from_integer(n), Unit::Percent, false)
}


fn to_rational(v: Value) -> Result<Rational, Error> {
    match v {
        Value::Numeric(v, _, _) => Ok(v),
        v => Err(badarg("number", &v)),
    }
}

#[test]
fn test_nth() {
    assert_eq!("foo", do_evaluate(&[("x", "foo, bar")], b"nth($x, 1);"))
}

#[cfg(test)]
use super::super::variablescope::test::do_evaluate;
