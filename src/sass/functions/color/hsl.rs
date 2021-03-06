use super::rgb::{preserve_call, values_from_list};
use super::{
    get_color, get_opt_rational, get_rational_pct, make_call,
    nospecial_value, Error, FunctionMap, Name,
};
use crate::css::Value;
use crate::value::{Hsla, Numeric, Unit};
use crate::Scope;
use num_rational::Rational;
use num_traits::{one, One, Zero};

fn do_hsla(fn_name: &str, s: &Scope) -> Result<Value, Error> {
    let a = s.get("alpha")?;
    let hue = s.get("hue")?;
    if let Value::List(vec, sep, bracketed) = if hue.is_null() {
        s.get("channels")?
    } else {
        hue.clone()
    } {
        if let Some((h, s, v, a)) = values_from_list(&vec) {
            Ok(hsla_from_values(&h, &s, &v, &a)?
                .unwrap_or_else(|| make_call(fn_name, vec![h, s, v, a])))
        } else {
            Ok(preserve_call(fn_name, vec, sep, bracketed))
        }
    } else {
        let sat = s.get("saturation")?;
        let lig = s.get("lightness")?;
        Ok(hsla_from_values(&hue, &sat, &lig, &a)?
            .unwrap_or_else(|| make_call(fn_name, vec![hue, sat, lig, a])))
    }
}

fn hsla_from_values(
    h: &Value,
    s: &Value,
    l: &Value,
    a: &Value,
) -> Result<Option<Value>, Error> {
    let h = nospecial_value(h, name!(hue), to_rational)?;
    let s = nospecial_value(s, name!(saturation), to_rational_percent)?;
    let l = nospecial_value(l, name!(lightness), to_rational_percent)?;
    let a = if a.is_null() {
        Some(Rational::one())
    } else {
        nospecial_value(a, name!(alpha), to_rational2)?
    };
    if let (Some(h), Some(s), Some(l), Some(a)) = (h, s, l, a) {
        Ok(Some(Hsla::new(h, s, l, a).into()))
    } else {
        Ok(None)
    }
}

pub fn register(f: &mut Scope) {
    def!(
        f,
        _hsl(
            hue = b"null",
            saturation = b"null",
            lightness = b"null",
            alpha = b"null",
            channels = b"()"
        ),
        |s| { do_hsla("hsl", s) }
    );
    def!(
        f,
        _hsla(
            hue = b"null",
            saturation = b"null",
            lightness = b"null",
            alpha = b"null",
            channels = b"null"
        ),
        |s| { do_hsla("hsla", s) }
    );
    def!(f, _adjust_hue(color, degrees), |s| {
        let col = get_color(s, "color")?;
        let adj = get_opt_rational(s, "degrees")?;
        if let Some(adj) = adj {
            Ok(col.rotate_hue(adj).into())
        } else {
            Ok(col.into())
        }
    });
    def!(f, complement(color), |s| {
        Ok(get_color(s, "color")?.rotate_hue(180.into()).into())
    });
    def!(f, _saturate(color = b"null", amount = b"null"), |s| match (
        s.get("color")?,
        s.get("amount")?
    ) {
        (Value::Color(c, _), Value::Null) => Ok(Value::Color(c, None)),
        (Value::Color(col, _), v @ Value::Numeric(..)) => {
            let hsla = col.to_hsla();
            let amount = to_rational2(&v, name!(amount))?;
            Ok(Hsla::new(
                hsla.hue(),
                hsla.sat() + amount,
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
        let lum = hsla.lum() + get_rational_pct(s, "amount")?;
        Ok(Hsla::new(hsla.hue(), hsla.sat(), lum, hsla.alpha()).into())
    });
    def!(f, _darken(color, amount), |s| {
        let col = get_color(s, "color")?;
        let hsla = col.to_hsla();
        let lum = hsla.lum() - get_rational_pct(s, "amount")?;
        Ok(Hsla::new(hsla.hue(), hsla.sat(), lum, hsla.alpha()).into())
    });
    def!(f, hue(color), |s| {
        let col = get_color(s, "color")?;
        let hsla = col.to_hsla();
        Ok(Value::Numeric(Numeric::new(hsla.hue(), Unit::Deg), true))
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
        let sat = hsla.sat() - get_rational_pct(s, "amount")?;
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

pub fn expose(m: &Scope, global: &mut FunctionMap) {
    for (gname, lname) in &[
        (name!(hsl), name!(_hsl)),
        (name!(hsla), name!(_hsla)),
        (name!(adjust_hue), name!(_adjust_hue)),
        (name!(complement), name!(complement)),
        (name!(darken), name!(_darken)),
        (name!(desaturate), name!(_desaturate)),
        (name!(grayscale), name!(grayscale)),
        (name!(hue), name!(hue)),
        (name!(lighten), name!(_lighten)),
        (name!(lightness), name!(lightness)),
        (name!(saturate), name!(_saturate)),
        (name!(saturation), name!(saturation)),
    ] {
        global.insert(gname.clone(), m.get_function(&lname).unwrap().clone());
    }
}

pub fn percentage(v: Rational) -> Value {
    Numeric::new(v * 100, Unit::Percent).into()
}

fn to_rational(v: &Value, name: Name) -> Result<Rational, Error> {
    match v {
        Value::Numeric(v, ..) => v.as_ratio(),
        v => Err(Error::bad_arg(name, v, "is not a number")),
    }
}

/// Gets a percentage as a fraction 0 .. 1.
/// If v is not a percentage, keep it as it is.
pub fn to_rational_percent(v: &Value, name: Name) -> Result<Rational, Error> {
    match v {
        Value::Null => Ok(Rational::zero()),
        Value::Numeric(v, ..) => {
            if v.unit.is_percent() || v.value > one() {
                Ok(v.as_ratio()? / 100)
            } else {
                Ok(v.as_ratio()?)
            }
        }
        v => Err(Error::bad_arg(name, v, "is not a number")),
    }
}
/// Gets a percentage as a fraction 0 .. 1.
/// If v is not a percentage, keep it as it is.
pub fn to_rational2(v: &Value, name: Name) -> Result<Rational, Error> {
    match v {
        Value::Null => Ok(Rational::zero()),
        Value::Numeric(v, ..) => {
            if v.unit.is_percent() {
                Ok(v.as_ratio()? / 100)
            } else {
                Ok(v.as_ratio()?)
            }
        }
        v => Err(Error::bad_arg(name, v, "is not a number")),
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
use crate::variablescope::test::do_evaluate;
