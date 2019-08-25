use super::{make_call, Error, SassFunction};
use crate::css::Value;
use crate::value::{Number, Unit};
use crate::variablescope::Scope;
use num_rational::Rational;
use num_traits::{One, Zero};
use std::collections::BTreeMap;

pub fn register(f: &mut BTreeMap<&'static str, SassFunction>) {
    def!(f, hsl(hue, saturation, lightness), |s| {
        let hue = s.get("hue")?;
        let sat = s.get("saturation")?;
        let lig = s.get("lightness")?;
        if let (Ok(hue), Ok(sat), Ok(lig)) = (
            to_rational(&hue),
            to_rational_percent(&sat),
            to_rational_percent(&lig),
        ) {
            Ok(Value::hsla(hue, sat, lig, Rational::one()))
        } else {
            Ok(make_call("hsl", vec![hue, sat, lig]))
        }
    });
    def!(f, hsla(hue, saturation, lightness, alpha), |s| {
        let hue = s.get("hue")?;
        let sat = s.get("saturation")?;
        let lig = s.get("lightness")?;
        let a = s.get("alpha")?;
        if let (Ok(hue), Ok(sat), Ok(lig), Ok(a)) = (
            to_rational(&hue),
            to_rational_percent(&sat),
            to_rational_percent(&lig),
            if a.is_null() {
                Ok(Rational::one())
            } else {
                to_rational(&a)
            },
        ) {
            Ok(Value::hsla(hue, sat, lig, a))
        } else {
            Ok(make_call("hsla", vec![hue, sat, lig, a]))
        }
    });
    def!(f, adjust_hue(color, degrees), |s: &dyn Scope| match (
        s.get("color")?,
        s.get("degrees")?,
    ) {
        (c @ Value::Color(..), Value::Null) => Ok(c),
        (Value::Color(rgba, _), Value::Numeric(v, ..)) => {
            let (h, s, l, alpha) = rgba.to_hsla();
            Ok(Value::hsla(h + v.value, s, l, alpha))
        }
        (c, v) => Err(Error::badargs(&["color", "number"], &[&c, &v])),
    });
    def!(
        f,
        complement(color),
        |s: &dyn Scope| match &s.get("color")? {
            &Value::Color(ref rgba, _) => {
                let (h, s, l, alpha) = rgba.to_hsla();
                Ok(Value::hsla(h + 180, s, l, alpha))
            }
            v => Err(Error::badarg("color", v)),
        }
    );
    def!(f, saturate(color, amount), |s: &dyn Scope| match (
        s.get("color")?,
        s.get("amount")?
    ) {
        (Value::Color(c, _), Value::Null) => Ok(Value::Color(c, None)),
        (Value::Color(rgba, _), Value::Numeric(v, u, _)) => {
            let (h, s, l, alpha) = rgba.to_hsla();
            let v = v.value;
            let v = if u == Unit::Percent { v / 100 } else { v };
            Ok(Value::hsla(h, s + v, l, alpha))
        }
        (c, v) => Ok(make_call("saturate", vec![c, v])),
    });
    def!(f, lighten(color, amount), |args: &dyn Scope| match &args
        .get("color")?
    {
        &Value::Color(ref rgba, _) => {
            let (h, s, l, alpha) = rgba.to_hsla();
            let amount = to_rational_percent(&args.get("amount")?)?;
            Ok(Value::hsla(h, s, l + amount, alpha))
        }
        v => Err(Error::badarg("color", v)),
    });
    def!(f, darken(color, amount), |args: &dyn Scope| match &args
        .get("color")?
    {
        &Value::Color(ref rgba, _) => {
            let (h, s, l, alpha) = rgba.to_hsla();
            let amount = to_rational_percent(&args.get("amount")?)?;
            Ok(Value::hsla(h, s, l - amount, alpha))
        }
        v => Err(Error::badarg("color", v)),
    });
    def!(
        f,
        hue(color),
        |args: &dyn Scope| match &args.get("color")? {
            &Value::Color(ref rgba, _) => {
                let (h, _s, _l, _a) = rgba.to_hsla();
                Ok(Value::Numeric(Number::from(h), Unit::Deg, true))
            }
            v => Err(Error::badarg("color", v)),
        }
    );
    def!(f, saturation(color), |args| match &args.get("color")? {
        &Value::Color(ref rgba, _) => {
            let (_h, s, _l, _a) = rgba.to_hsla();
            Ok(percentage(s))
        }
        v => Err(Error::badarg("color", v)),
    });
    def!(f, lightness(color), |args| match &args.get("color")? {
        &Value::Color(ref rgba, _) => {
            let (_h, _s, l, _a) = rgba.to_hsla();
            Ok(percentage(l))
        }
        v => Err(Error::badarg("color", v)),
    });
    def!(f, desaturate(color, amount), |args: &dyn Scope| match &args
        .get("color")?
    {
        &Value::Color(ref rgba, _) => {
            let (h, s, l, alpha) = rgba.to_hsla();
            let amount = to_rational_percent(&args.get("amount")?)?;
            Ok(Value::hsla(h, s - amount, l, alpha))
        }
        v => Err(Error::badarg("color", v)),
    });
    def!(f, grayscale(color), |args| match args.get("color")? {
        Value::Color(ref rgba, _) => {
            let (h, _s, l, alpha) = rgba.to_hsla();
            Ok(Value::hsla(h, Zero::zero(), l, alpha))
        }
        v => Ok(make_call("grayscale", vec![v])),
    });
}

fn percentage(v: Rational) -> Value {
    Value::Numeric(Number::from(v * 100), Unit::Percent, true)
}

fn to_rational(v: &Value) -> Result<Rational, Error> {
    match v {
        Value::Numeric(v, ..) => Ok(v.value),
        v => Err(Error::badarg("number", v)),
    }
}

/// Gets a percentage as a fraction 0 .. 1.
/// If v is not a percentage, keep it as it is.
fn to_rational_percent(v: &Value) -> Result<Rational, Error> {
    match v {
        Value::Null => Ok(Rational::zero()),
        Value::Numeric(v, Unit::Percent, _) => Ok(v.value / 100),
        Value::Numeric(v, ..) => {
            let v = v.value;
            Ok(if v <= Rational::one() { v } else { v / 100 })
        }
        v => Err(Error::badarg("number", &v)),
    }
}

#[test]
fn test_hsl_black() {
    assert_eq!("black", do_evaluate(&[], b"hsl(17, 32%, 0%);"))
}
#[test]
fn test_hsl_white() {
    assert_eq!("white", do_evaluate(&[], b"hsl(300, 82%, 100%);"))
}
#[test]
fn test_hsl_gray() {
    assert_eq!("gray", do_evaluate(&[], b"hsl(300, 0%, 50%);"))
}
#[test]
fn test_hsl_red() {
    assert_eq!("#f7c9c9", do_evaluate(&[], b"hsl(0, 75%, 88%);"))
}
#[test]
fn test_hsl_yellow() {
    assert_eq!("#ffff42", do_evaluate(&[], b"hsl(60, 100%, 63%);"))
}
#[test]
fn test_hsl_blue_magenta() {
    assert_eq!("#6118aa", do_evaluate(&[], b"hsl(270, 75%, 38%);"))
}

#[cfg(test)]
use super::super::variablescope::test::do_evaluate;
