use super::rgb::{preserve_call, values_from_list};
use super::{
    check, check_pct_rational_range, get_checked, get_color, get_opt_check,
    make_call, nospecial_value, to_rational_percent, Error, FunctionMap,
};
use crate::css::Value;
use crate::output::Format;
use crate::value::{Hsla, Numeric, Rational, Unit};
use crate::Scope;
use num_traits::{one, zero};

fn do_hsla(fn_name: &str, s: &Scope) -> Result<Value, Error> {
    let a = s.get("alpha")?;
    let hue = s.get("hue")?;
    if let Value::List(vec, sep, bracketed) = if hue.is_null() {
        s.get("channels")?
    } else {
        hue.clone()
    } {
        if bracketed {
            return Err(Error::BadValue(
                "Error: $channels must be an unbracketed list.".into(),
            ));
        }
        if let Some((h, s, v, a)) = values_from_list(&vec, sep) {
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
    let a = match a {
        Value::Null => Some(one()),
        a => nospecial_value(a, name!(alpha), to_rational2)?,
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
    def!(f, adjust_hue(color, degrees), |s| {
        let col = get_color(s, "color")?;
        let adj = get_opt_check(s, name!(degrees), to_rational)?;
        if let Some(adj) = adj {
            Ok(col.rotate_hue(adj).into())
        } else {
            Ok(col.into())
        }
    });
    def!(f, complement(color), |s| {
        Ok(get_color(s, "color")?.rotate_hue(180.into()).into())
    });
    def!(f, saturate(color = b"null", amount = b"null"), |s| {
        let col = match s.get("color")? {
            Value::Color(col, _) => col,
            c => return Ok(make_call("saturate", vec![c, s.get("amount")?])),
        };
        let hsla = col.to_hsla();
        let sat = hsla.sat()
            + get_opt_check(s, name!(amount), check_pct_rational_range)?
                .unwrap_or_else(zero);
        Ok(Hsla::new(hsla.hue(), sat, hsla.lum(), hsla.alpha()).into())
    });
    def!(f, lighten(color, amount), |s| {
        let col = get_color(s, "color")?;
        let hsla = col.to_hsla();
        let lum = hsla.lum()
            + get_checked(s, name!(amount), check_pct_rational_range)?;
        Ok(Hsla::new(hsla.hue(), hsla.sat(), lum, hsla.alpha()).into())
    });
    def!(f, darken(color, amount), |s| {
        let col = get_color(s, "color")?;
        let hsla = col.to_hsla();
        let lum = hsla.lum()
            - get_checked(s, name!(amount), check_pct_rational_range)?;
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
    def!(f, desaturate(color, amount), |s| {
        let col = get_color(s, "color")?;
        let hsla = col.to_hsla();
        let sat = hsla.sat()
            - get_checked(s, name!(amount), check_pct_rational_range)?;
        Ok(Hsla::new(hsla.hue(), sat, hsla.lum(), hsla.alpha()).into())
    });
    def!(f, grayscale(color), |args| match args.get("color")? {
        Value::Color(col, _) => {
            let hsla = col.to_hsla();
            Ok(
                Hsla::new(hsla.hue(), zero(), hsla.lum(), hsla.alpha())
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
        (name!(adjust_hue), name!(adjust_hue)),
        (name!(complement), name!(complement)),
        (name!(darken), name!(darken)),
        (name!(desaturate), name!(desaturate)),
        (name!(grayscale), name!(grayscale)),
        (name!(hue), name!(hue)),
        (name!(lighten), name!(lighten)),
        (name!(lightness), name!(lightness)),
        (name!(saturate), name!(saturate)),
        (name!(saturation), name!(saturation)),
    ] {
        global.insert(gname.clone(), m.get_lfunction(&lname));
    }
}

pub fn percentage(v: Rational) -> Value {
    Numeric::new(v * 100, Unit::Percent).into()
}

fn to_rational(v: Value) -> Result<Rational, String> {
    check::numeric(v).and_then(|v| v.as_ratio().map_err(|e| e.to_string()))
}

/// Gets a percentage as a fraction 0 .. 1.
/// If v is not a percentage, keep it as it is.
pub fn to_rational2(v: Value) -> Result<Rational, String> {
    match v {
        Value::Null => Ok(zero()),
        Value::Numeric(v, ..) => {
            let r = v.value.as_ratio().map_err(|e| e.to_string())?;
            if v.unit.is_percent() {
                Ok(r / 100)
            } else {
                Ok(r)
            }
        }
        v => Err(format!(
            "{} is not a number",
            v.format(Format::introspect())
        )),
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
