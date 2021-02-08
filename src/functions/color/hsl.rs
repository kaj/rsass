use super::rgb::{preserve_call, values_from_list};
use super::{get_color, make_call, Error, Module, SassFunction};
use crate::css::Value;
use crate::value::{Hsla, Number, Unit};
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
    Some(Hsla::new(h, s, l, a).into())
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
        (Value::Color(col, _), Value::Numeric(v, ..)) => {
            Ok(col.rotate_hue(v.as_ratio()?).into())
        }
        (c, v) => Err(Error::badargs(&["color", "number"], &[&c, &v])),
    });
    def!(f, complement(color), |s| {
        Ok(get_color(s, "color")?.rotate_hue(180.into()).into())
    });
    def!(f, _saturate(color, amount), |s: &dyn Scope| match (
        s.get("color")?,
        s.get("amount")?
    ) {
        (Value::Color(c, _), Value::Null) => Ok(Value::Color(c, None)),
        (Value::Color(col, _), v @ Value::Numeric(..)) => {
            let hsla = col.to_hsla();
            Ok(Hsla::new(
                hsla.hue(),
                hsla.sat() + to_rational2(&v)?,
                hsla.lum(),
                hsla.alpha(),
            )
            .into())
        }
        (c, v) => Ok(make_call("saturate", vec![c, v])),
    });
    def!(f, _lighten(color, amount), |s| {
        let col = get_color(s, "color")?;
        let hsla = col.to_hsla();
        let lum = hsla.lum() + to_rational_percent(&s.get("amount")?)?;
        Ok(Hsla::new(hsla.hue(), hsla.sat(), lum, hsla.alpha()).into())
    });
    def!(f, _darken(color, amount), |s| {
        let col = get_color(s, "color")?;
        let hsla = col.to_hsla();
        let lum = hsla.lum() - to_rational_percent(&s.get("amount")?)?;
        Ok(Hsla::new(hsla.hue(), hsla.sat(), lum, hsla.alpha()).into())
    });
    def!(f, hue(color), |s| {
        let col = get_color(s, "color")?;
        let hsla = col.to_hsla();
        Ok(Value::Numeric(Number::from(hsla.hue()), Unit::Deg, true))
    });
    def!(f, saturation(color), |s| {
        Ok(percentage(get_color(s, "color")?.to_hsla().sat()))
    });
    def!(f, lightness(color), |s| {
        Ok(percentage(get_color(s, "color")?.to_hsla().lum()))
    });
    def!(f, _desaturate(color, amount), |s| {
        let col = get_color(s, "color")?;
        let hsla = col.to_hsla();
        let sat = hsla.sat() - to_rational_percent(&s.get("amount")?)?;
        Ok(Hsla::new(hsla.hue(), sat, hsla.lum(), hsla.alpha()).into())
    });
    def!(f, grayscale(color), |args| match args.get("color")? {
        Value::Color(col, _) => {
            let hsla = col.to_hsla();
            Ok(
                Hsla::new(hsla.hue(), Zero::zero(), hsla.lum(), hsla.alpha())
                    .into(),
            )
        }
        v => Ok(make_call("grayscale", vec![v])),
    });
}

pub fn expose(m: &Module, global: &mut Module) {
    for &(gname, lname) in &[
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
        global.expose(gname, m, lname);
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
