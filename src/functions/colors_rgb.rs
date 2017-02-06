use super::{Error, SassFunction, badarg, badargs};
use formalargs::FormalArgs;
use num_rational::Rational;
use num_traits::{One, Signed, Zero};
use std::collections::BTreeMap;
use valueexpression::{Unit, Value};
use variablescope::Scope;

pub fn register(f: &mut BTreeMap<&'static str, SassFunction>) {
    f.insert("rgb",
             func!((red, green, blue), |s| {
                 Ok(Value::rgba(to_int(s.get("red"))?,
                                to_int(s.get("green"))?,
                                to_int(s.get("blue"))?,
                                Rational::one()))
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
            Ok(Value::rgba(r, g, b, to_rational(a)?))
        } else {
            Ok(Value::rgba(to_int(red)?,
                           to_int(s.get("green"))?,
                           to_int(s.get("blue"))?,
                           to_rational(alpha)?))
        }
    }));
    f.insert("red",
             func!((color), |s: &Scope| match &s.get("color") {
                 &Value::Color(ref red, _, _, _, _) => {
                     Ok(Value::Numeric(*red, Unit::None, true))
                 }
                 value => Err(badarg("color", value)),
             }));
    f.insert("green",
             func!((color), |s: &Scope| match &s.get("color") {
                 &Value::Color(_, ref green, _, _, _) => {
                     Ok(Value::Numeric(*green, Unit::None, true))
                 }
                 value => Err(badarg("color", value)),
             }));
    f.insert("blue",
             func!((color), |s: &Scope| match &s.get("color") {
                 &Value::Color(_, _, ref blue, _, _) => {
                     Ok(Value::Numeric(*blue, Unit::None, true))
                 }
                 value => Err(badarg("color", value)),
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
            let one = Rational::one();
            let w2 = one - (one - w * a1) * a2;
            fn m(v1: &Rational, v2: &Rational, w: Rational) -> Rational {
                *v1 * w + *v2 * (Rational::one() - w)
            }
            Ok(Value::rgba(m(r1, r2, w2),
                           m(g1, g2, w2),
                           m(b1, b2, w2),
                           m(a1, a2, w)))
        } else {
            Err(badargs(&["color", "color", "number"],
                        &[&color1, &color2, &weight]))
        }
    }));
    f.insert("invert",
             func!((color), |s: &Scope| match &s.get("color") {
                 &Value::Color(ref r, ref g, ref b, ref a, _) => {
                     let ff = Rational::new(255, 1);
                     Ok(Value::rgba(ff - r, ff - g, ff - b, *a))
                 }
                 value => Err(badarg("color", value)),
             }));
}

fn to_int(v: Value) -> Result<Rational, Error> {
    Ok(cap_u8(match v {
        Value::Numeric(v, Unit::Percent, _) => Rational::new(255, 100) * v,
        Value::Numeric(v, _, _) => v,
        v => return Err(badarg("number", &v)),
    }))
}

fn cap_u8(n: Rational) -> Rational {
    let ff = Rational::new(255, 1);
    if n > ff {
        ff
    } else if n.is_negative() {
        Rational::zero()
    } else {
        n
    }
}

#[test]
fn percent_to_int_0() {
    assert_eq!(to_int(percent(0)), Ok(Rational::zero()))
}
#[test]
fn percent_to_int_50() {
    assert_eq!(to_int(percent(50)), Ok(Rational::new(255, 2)))
}
#[test]
fn percent_to_int_100() {
    assert_eq!(to_int(percent(100)), Ok(Rational::new(255, 1)))
}
#[test]
fn percent_to_int_overflow() {
    assert_eq!(to_int(percent(130)), Ok(Rational::new(255, 1)))
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
