use super::rgb::{preserve_call, values_from_list};
use super::{make_call, Error, Module, SassFunction};
use crate::css::Value;
use crate::value::{Number, Rgba, Unit};
use crate::variablescope::Scope;
use num_rational::Rational;
use num_traits::{One, Zero};

fn do_hsla(fn_name: &str, s: &dyn Scope) -> Result<Value, Error> {
    let a = s.get("alpha")?;
    let hue = s.get("hue")?;
    if let Value::List(vec, sep, bracketed) = if hue.is_null() {
        s.get("channels")?
    } else {
        hue.clone()
    } {
        if let Some((h, s, v, a)) = values_from_list(&vec) {
            Ok(hsla_from_values(&h, &s, &v, &a)
                .unwrap_or_else(|| make_call(fn_name, vec![h, s, v, a])))
        } else {
            Ok(preserve_call(fn_name, vec, sep, bracketed))
        }
    } else {
        let sat = s.get("saturation")?;
        let lig = s.get("lightness")?;
        Ok(hsla_from_values(&hue, &sat, &lig, &a)
            .unwrap_or_else(|| make_call(fn_name, vec![hue, sat, lig, a])))
    }
}

fn hsla_from_values(
    h: &Value,
    s: &Value,
    l: &Value,
    a: &Value,
) -> Option<Value> {
    let h = to_rational(h).ok()?;
    let s = to_rational_percent(s).ok()?;
    let l = to_rational_percent(l).ok()?;
    let a = if a.is_null() {
        Rational::one()
    } else {
        to_rational2(a).ok()?
    };
    Some(Rgba::from_hsla(h, s, l, a).into())
}

pub fn register(f: &mut Module) {
    def!(f, _hsl(hue, saturation, lightness, alpha, channels), |s| {
        do_hsla("hsl", s)
    });
    def!(f, _hsla(hue, saturation, lightness, alpha, channels), |s| {
        do_hsla("hsla", s)
    });
    def!(f, _adjust_hue(color, degrees), |s: &dyn Scope| match (
        s.get("color")?,
        s.get("degrees")?,
    ) {
        (c @ Value::Color(..), Value::Null) => Ok(c),
        (Value::Color(rgba, _), Value::Numeric(v, ..)) => {
            let (h, s, l, alpha) = rgba.to_hsla();
            Ok(Rgba::from_hsla(h + v.as_ratio()?, s, l, alpha).into())
        }
        (c, v) => Err(Error::badargs(&["color", "number"], &[&c, &v])),
    });
    def!(
        f,
        complement(color),
        |s: &dyn Scope| match &s.get("color")? {
            &Value::Color(ref rgba, _) => {
                let (h, s, l, alpha) = rgba.to_hsla();
                Ok(Rgba::from_hsla(h + 180, s, l, alpha).into())
            }
            v => Err(Error::badarg("color", v)),
        }
    );
    def!(f, _saturate(color, amount), |s: &dyn Scope| match (
        s.get("color")?,
        s.get("amount")?
    ) {
        (Value::Color(c, _), Value::Null) => Ok(Value::Color(c, None)),
        (Value::Color(rgba, _), Value::Numeric(v, u, _)) => {
            let (h, s, l, alpha) = rgba.to_hsla();
            let v = v.as_ratio()?;
            let v = if u == Unit::Percent { v / 100 } else { v };
            Ok(Rgba::from_hsla(h, s + v, l, alpha).into())
        }
        (c, v) => Ok(make_call("saturate", vec![c, v])),
    });
    def!(f, _lighten(color, amount), |args: &dyn Scope| match &args
        .get("color")?
    {
        &Value::Color(ref rgba, _) => {
            let (h, s, l, alpha) = rgba.to_hsla();
            let amount = to_rational_percent(&args.get("amount")?)?;
            Ok(Rgba::from_hsla(h, s, l + amount, alpha).into())
        }
        v => Err(Error::badarg("color", v)),
    });
    def!(f, _darken(color, amount), |args: &dyn Scope| match &args
        .get("color")?
    {
        &Value::Color(ref rgba, _) => {
            let (h, s, l, alpha) = rgba.to_hsla();
            let amount = to_rational_percent(&args.get("amount")?)?;
            Ok(Rgba::from_hsla(h, s, l - amount, alpha).into())
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
    def!(
        f,
        _desaturate(color, amount),
        |args: &dyn Scope| match &args.get("color")? {
            &Value::Color(ref rgba, _) => {
                let (h, s, l, alpha) = rgba.to_hsla();
                let amount = to_rational_percent(&args.get("amount")?)?;
                Ok(Rgba::from_hsla(h, s - amount, l, alpha).into())
            }
            v => Err(Error::badarg("color", v)),
        }
    );
    def!(f, grayscale(color), |args| match args.get("color")? {
        Value::Color(ref rgba, _) => {
            let (h, _s, l, alpha) = rgba.to_hsla();
            Ok(Rgba::from_hsla(h, Zero::zero(), l, alpha).into())
        }
        v => Ok(make_call("grayscale", vec![v])),
    });
}

pub fn expose(meta: &Module, global: &mut Module) {
    for (gname, lname) in &[
        ("hsl", "_hsl"),
        ("hsla", "_hsla"),
        ("adjust_hue", "_adjust_hue"),
        ("complement", "complement"),
        ("darken", "_darken"),
        ("desaturate", "_desaturate"),
        ("grayscale", "grayscale"),
        ("hue", "hue"),
        ("lighten", "_lighten"),
        ("lightness", "lightness"),
        ("saturate", "_saturate"),
        ("saturation", "saturation"),
    ] {
        global.insert_function(gname, meta.get_function(lname).unwrap().clone());
    }
}

pub fn percentage(v: Rational) -> Value {
    Value::Numeric(Number::from(v * 100), Unit::Percent, true)
}

fn to_rational(v: &Value) -> Result<Rational, Error> {
    match v {
        Value::Numeric(v, ..) => v.as_ratio(),
        v => Err(Error::badarg("number", v)),
    }
}

/// Gets a percentage as a fraction 0 .. 1.
/// If v is not a percentage, keep it as it is.
pub fn to_rational_percent(v: &Value) -> Result<Rational, Error> {
    match v {
        Value::Null => Ok(Rational::zero()),
        Value::Numeric(v, Unit::Percent, _) => Ok(v.as_ratio()? / 100),
        Value::Numeric(v, ..) => {
            let v = v.as_ratio()?;
            Ok(if v <= Rational::one() { v } else { v / 100 })
        }
        v => Err(Error::badarg("number", &v)),
    }
}
/// Gets a percentage as a fraction 0 .. 1.
/// If v is not a percentage, keep it as it is.
pub fn to_rational2(v: &Value) -> Result<Rational, Error> {
    match v {
        Value::Null => Ok(Rational::zero()),
        Value::Numeric(v, Unit::Percent, _) => Ok(v.as_ratio()? / 100),
        Value::Numeric(v, ..) => Ok(v.as_ratio()?),
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
use super::super::super::variablescope::test::do_evaluate;
