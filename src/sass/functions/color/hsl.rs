use super::channels::Channels;
use super::{
    check_alpha, check_pct, check_pct_range, check_rational, eval_inner,
    is_not, is_special, make_call, CallError, CheckedArg, FunctionMap,
    ResolvedArgs,
};
use crate::css::{CallArgs, Value};
use crate::output::Format;
use crate::sass::{ArgsError, FormalArgs, Name};
use crate::value::{Color, Hsla, Numeric, Rational, Unit};
use crate::Scope;
use num_traits::zero;

pub fn register(f: &mut Scope) {
    def_va!(f, _hsl(kwargs), |s| do_hsla(&name!(hsl), s));
    def_va!(f, _hsla(kwargs), |s| do_hsla(&name!(hsla), s));
    def!(f, complement(color), |s| {
        Ok(s.get::<Color>(name!(color))?.rotate_hue(180.into()).into())
    });
    def!(f, hue(color), |s| {
        let col = s.get::<Color>(name!(color))?;
        let hsla = col.to_hsla();
        Ok(Value::Numeric(Numeric::new(hsla.hue(), Unit::Deg), true))
    });
    def!(f, saturation(color), |s| {
        Ok(percentage(s.get::<Color>(name!(color))?.to_hsla().sat()))
    });
    def!(f, lightness(color), |s| {
        Ok(percentage(s.get::<Color>(name!(color))?.to_hsla().lum()))
    });
    def!(f, grayscale(color), |args| match args.get(name!(color))? {
        Value::Color(col, _) => {
            let col = col.to_hsla();
            Ok(Hsla::new(col.hue(), zero(), col.lum(), col.alpha(), false)
                .into())
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
        let col = s.get::<Color>(name!(color))?;
        if let Some(adj) = s.get_opt_map(name!(degrees), check_rational)? {
            Ok(col.rotate_hue(adj).into())
        } else {
            Ok(col.into())
        }
    });
    def!(f, darken(color, amount), |s| {
        let col = s.get::<Color>(name!(color))?;
        let col = col.to_hsla();
        let lum = col.lum() - s.get_map(name!(amount), check_pct_range)?;
        Ok(Hsla::new(col.hue(), col.sat(), lum, col.alpha(), false).into())
    });
    def!(f, desaturate(color, amount), |s| {
        let col = s.get::<Color>(name!(color))?;
        let col = col.to_hsla();
        let sat = col.sat() - s.get_map(name!(amount), check_pct_range)?;
        Ok(Hsla::new(col.hue(), sat, col.lum(), col.alpha(), false).into())
    });
    def_va!(f, saturate(kwargs), |s| {
        let a1 = FormalArgs::new(vec![one_arg!(color), one_arg!(amount)]);
        let a2 = FormalArgs::new(vec![one_arg!(amount)]);
        let args = s.get_map(name!(kwargs), CallArgs::from_value)?;
        match eval_inner(&name!(saturate), &a1, s, args.clone()) {
            Ok(s) => {
                let col = s.get::<Color>(name!(color))?;
                let sat = s.get_map(name!(amount), check_pct_range)?;
                let col = col.to_hsla();
                let sat = col.sat() + sat;
                Ok(Hsla::new(col.hue(), sat, col.lum(), col.alpha(), false)
                    .into())
            }
            Err(CallError::Args(ArgsError::Missing(_), _)) => {
                let s = eval_inner(&name!(saturate), &a2, s, args)?;
                let sat = s.get_map(name!(amount), |v| {
                    check_pct(v.clone()).map(|_| v) // validate only
                })?;
                Ok(make_call("saturate", vec![sat]))
            }
            Err(ae) => Err(ae),
        }
    });
    def!(f, lighten(color, amount), |s| {
        let col = s.get::<Color>(name!(color))?;
        let col = col.to_hsla();
        let lum = col.lum() + s.get_map(name!(amount), check_pct_range)?;
        Ok(Hsla::new(col.hue(), col.sat(), lum, col.alpha(), false).into())
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

fn do_hsla(fn_name: &Name, s: &ResolvedArgs) -> Result<Value, CallError> {
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
    let args = s.get_map(name!(kwargs), CallArgs::from_value)?;
    match eval_inner(fn_name, &a1, s, args.clone()) {
        Ok(s) => Channels::try_from(s.get::<Value>(name!(channels))?)
            .map_err(|e| e.conv(&["hue", "saturation", "lightness"]))
            .and_then(|c| match c {
                Channels::Data([h, s, l, a]) => {
                    hsla_from_values(fn_name, h, s, l, a)
                }
                Channels::Special(channels) => {
                    Ok(make_call(fn_name.as_ref(), vec![channels]))
                }
            }),
        Err(err @ CallError::Args(ArgsError::Missing(_), _)) => Err(err),
        Err(_) => {
            let s = eval_inner(fn_name, &a2_show, s, args.clone()).or_else(
                |e| eval_inner(fn_name, &a2, s, args).map_err(|_| e),
            )?;
            hsla_from_values(
                fn_name,
                s.get(name!(hue))?,
                s.get(name!(saturation))?,
                s.get(name!(lightness))?,
                s.get(name!(alpha))?,
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
) -> Result<Value, CallError> {
    if is_special(&h) || is_special(&s) || is_special(&l) || is_special(&a) {
        Ok(make_call(fn_name.as_ref(), vec![h, s, l, a]))
    } else if l == Value::Null {
        Err(CallError::msg("Missing argument $lightness."))
    } else {
        Ok(Hsla::new(
            check_rational(h).named(name!(hue))?,
            check_pct_opt(s).named(name!(saturation))?,
            check_pct_opt(l).named(name!(lightness))?,
            check_alpha(a).named(name!(alpha))?,
            true, // ??
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
    let v = Numeric::try_from(v)?;
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
    assert_eq!(
        do_evaluate(&[], b"hsl(17, 32%, 0%);"),
        "hsl(17deg, 32%, 0%)"
    )
}
#[test]
fn test_hsl_white() {
    assert_eq!(
        do_evaluate(&[], b"hsl(300, 82%, 100%);"),
        "hsl(300deg, 82%, 100%)"
    )
}
#[test]
fn test_hsl_gray() {
    assert_eq!(
        do_evaluate(&[], b"hsl(300, 0%, 50%);"),
        "hsl(300deg, 0%, 50%)"
    )
}
#[test]
fn test_hsl_red() {
    assert_eq!(
        do_evaluate(&[], b"hsl(0, 75%, 88%);"),
        "hsl(0deg, 75%, 88%)"
    )
}
#[test]
fn test_hsl_yellow() {
    assert_eq!(
        do_evaluate(&[], b"hsl(60, 100%, 63%);"),
        "hsl(60deg, 100%, 63%)"
    )
}

#[test]
fn test_hsl_blue_magenta() {
    assert_eq!(
        do_evaluate(&[], b"hsl(270, 75%, 38%);"),
        "hsl(270deg, 75%, 38%)"
    )
}

#[cfg(test)]
use crate::variablescope::test::do_evaluate;
