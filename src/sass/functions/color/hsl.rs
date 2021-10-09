use super::channels::Channels;
use super::{
    bad_arg, check_alpha, check_pct, check_pct_range, check_rational,
    get_checked, get_color, get_opt_check, is_not, is_special, make_call,
    CheckedArg, FunctionMap,
};
use crate::css::{CallArgs, Value};
use crate::output::Format;
use crate::sass::{ArgsError, FormalArgs, Name};
use crate::value::{Hsla, Numeric, Rational, Unit};
use crate::{Error, Scope, ScopeRef};
use num_traits::zero;
use std::convert::TryFrom;

pub fn register(f: &mut Scope) {
    def_va!(f, _hsl(kwargs), |s| do_hsla(&name!(hsl), s));
    def_va!(f, _hsla(kwargs), |s| do_hsla(&name!(hsla), s));
    def!(f, complement(color), |s| {
        Ok(get_color(s, "color")?.rotate_hue(180.into()).into())
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
    def!(f, grayscale(color), |args| match args.get("color")? {
        Value::Color(col, _) => {
            let hsla = col.to_hsla();
            Ok(
                Hsla::new(hsla.hue(), zero(), hsla.lum(), hsla.alpha())
                    .into(),
            )
        }
        v @ Value::Numeric(..) => Ok(make_call("grayscale", vec![v])),
        v => Err(is_not(&v, "a color")).named(name!(color)),
    });
}

pub fn expose(m: &Scope, global: &mut FunctionMap) {
    for (gname, lname) in &[
        (name!(hsl), name!(_hsl)),
        (name!(hsla), name!(_hsla)),
        (name!(complement), name!(complement)),
        (name!(grayscale), name!(grayscale)),
        (name!(hue), name!(hue)),
        (name!(lightness), name!(lightness)),
        (name!(saturation), name!(saturation)),
    ] {
        global.insert(gname.clone(), m.get_lfunction(lname));
    }
    let mut f = Scope::builtin_module("sass:color");
    def!(f, adjust_hue(color, degrees), |s| {
        let col = get_color(s, "color")?;
        let adj = get_opt_check(s, name!(degrees), check_rational)?;
        if let Some(adj) = adj {
            Ok(col.rotate_hue(adj).into())
        } else {
            Ok(col.into())
        }
    });
    def!(f, darken(color, amount), |s| {
        let col = get_color(s, "color")?;
        let hsla = col.to_hsla();
        let lum =
            hsla.lum() - get_checked(s, name!(amount), check_pct_range)?;
        Ok(Hsla::new(hsla.hue(), hsla.sat(), lum, hsla.alpha()).into())
    });
    def!(f, desaturate(color, amount), |s| {
        let col = get_color(s, "color")?;
        let hsla = col.to_hsla();
        let sat =
            hsla.sat() - get_checked(s, name!(amount), check_pct_range)?;
        Ok(Hsla::new(hsla.hue(), sat, hsla.lum(), hsla.alpha()).into())
    });
    def_va!(f, saturate(kwargs), |s| {
        let a1 = FormalArgs::new(vec![one_arg!(color), one_arg!(amount)]);
        let a2 = FormalArgs::new(vec![one_arg!(amount)]);
        let args = CallArgs::from_value(s.get("kwargs")?)?;
        match a1.eval(s.clone(), args.clone()) {
            Ok(s) => {
                let col = get_color(&s, "color")?;
                let sat = get_checked(&s, name!(amount), check_pct_range)?;
                let hsla = col.to_hsla();
                let sat = hsla.sat() + sat;
                Ok(Hsla::new(hsla.hue(), sat, hsla.lum(), hsla.alpha())
                    .into())
            }
            Err(ArgsError::Missing(_)) => {
                let s = a2
                    .eval(s.clone(), args)
                    .map_err(|e| bad_arg(e, &name!(saturate), &a2))?;
                let sat = s.get("amount")?;
                check_pct(sat.clone()).named(name!(amount))?;
                Ok(make_call("saturate", vec![sat]))
            }
            Err(ae) => Err(bad_arg(ae, &name!(saturate), &a1)),
        }
    });
    def!(f, lighten(color, amount), |s| {
        let col = get_color(s, "color")?;
        let hsla = col.to_hsla();
        let lum =
            hsla.lum() + get_checked(s, name!(amount), check_pct_range)?;
        Ok(Hsla::new(hsla.hue(), hsla.sat(), lum, hsla.alpha()).into())
    });
    for (gname, lname) in &[
        (name!(adjust_hue), name!(adjust_hue)),
        (name!(darken), name!(darken)),
        (name!(desaturate), name!(desaturate)),
        (name!(lighten), name!(lighten)),
        (name!(saturate), name!(saturate)),
    ] {
        global.insert(gname.clone(), f.get_lfunction(lname));
    }
}

fn do_hsla(fn_name: &Name, s: &ScopeRef) -> Result<Value, Error> {
    let a1 = FormalArgs::new(vec![one_arg!(channels)]);
    let a2 = FormalArgs::new(vec![
        one_arg!(hue),
        one_arg!(saturation),
        one_arg!(lightness = b"null"),
        one_arg!(alpha = b"null"),
    ]);
    let a2_show = FormalArgs::new(vec![
        one_arg!(hue),
        one_arg!(saturation),
        one_arg!(lightness),
        one_arg!(alpha),
    ]);
    let args = CallArgs::from_value(s.get("kwargs")?)?;
    match a1.eval(s.clone(), args.clone()) {
        Ok(s) => Channels::try_from(s.get("channels")?)
            .map_err(|e| e.conv(&["hue", "saturation", "lightness"]))
            .and_then(|c| match c {
                Channels::Data([h, s, l, a]) => {
                    hsla_from_values(fn_name, h, s, l, a)
                }
                Channels::Special(channels) => {
                    Ok(make_call(fn_name.as_ref(), vec![channels]))
                }
            }),
        Err(err @ ArgsError::Missing(_)) => Err(bad_arg(err, fn_name, &a1)),
        Err(_) => {
            let s = a2
                .eval(s.clone(), args)
                .map_err(|e| bad_arg(e, fn_name, &a2_show))?;

            hsla_from_values(
                fn_name,
                s.get("hue")?,
                s.get("saturation")?,
                s.get("lightness")?,
                s.get("alpha")?,
            )
        }
    }
}

fn hsla_from_values(
    fn_name: &Name,
    h: Value,
    s: Value,
    l: Value,
    a: Value,
) -> Result<Value, Error> {
    if is_special(&h) || is_special(&s) || is_special(&l) || is_special(&a) {
        Ok(make_call(fn_name.as_ref(), vec![h, s, l, a]))
    } else if l == Value::Null {
        Err(Error::error("Missing argument $lightness."))
    } else {
        Ok(Hsla::new(
            check_rational(h).named(name!(hue))?,
            check_pct_opt(s).named(name!(saturation))?,
            check_pct_opt(l).named(name!(lightness))?,
            check_alpha(a).named(name!(alpha))?,
        )
        .into())
    }
}

pub fn percentage(v: Rational) -> Value {
    Numeric::new(v * 100, Unit::Percent).into()
}

/// Gets a percentage as a fraction 0 .. 1.
/// If v is not a percentage, keep it as it is.
fn check_pct_opt(v: Value) -> Result<Rational, String> {
    let v = super::check::numeric(v)?;
    if !v.unit.is_percent() {
        // Note: The deprecation warning should include the parameter name
        // and line of call, but we don't have that here.
        dep_warn!(
            "Passing a number without unit % ({}) is deprecated.",
            v.format(Format::introspect())
        );
    }
    Ok(v.as_ratio()? / 100)
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
