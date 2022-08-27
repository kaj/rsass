use super::{
    check_alpha_pm, check_alpha_range, check_channel_pm, check_channel_range,
    check_expl_pct, check_rational, expected_to, make_call, CallError,
    CheckedArg, FunctionMap, Name,
};
use crate::css::{CallArgs, Value};
use crate::value::{Color, Hsla, Hwba, Numeric, Rational, RgbFormat, Rgba};
use crate::Scope;
use num_traits::{one, zero, Signed};

pub fn register(f: &mut Scope) {
    def_va!(f, adjust(color, kwargs), |s| {
        fn opt_add(a: Rational, b: Option<Rational>) -> Rational {
            if let Some(b) = b {
                a + b
            } else {
                a
            }
        }
        let rgba: Color = s.get(name!(color))?;
        let mut args = s.get_map(name!(kwargs), CallArgs::from_value)?;
        if !args.positional.is_empty() {
            return Err(CallError::msg("Only one positional argument is allowed. \
                                       All other arguments must be passed by name."));
        }
        let red = take_opt(&mut args, name!(red), check_channel_pm)?;
        let gre = take_opt(&mut args, name!(green), check_channel_pm)?;
        let blu = take_opt(&mut args, name!(blue), check_channel_pm)?;
        let hue = take_opt(&mut args, name!(hue), check_rational)?;
        let sat = take_opt(&mut args, name!(saturation), check_pct_pm)?;
        let lig = take_opt(&mut args, name!(lightness), check_pct_pm)?;
        let bla = take_opt(&mut args, name!(blackness), check_pct_expl_pm)?;
        let whi = take_opt(&mut args, name!(whiteness), check_pct_expl_pm)?;
        let a_adj = take_opt(&mut args, name!(alpha), check_alpha_pm)?;
        args.check_no_named().map_err(CallError::msg)?;

        if red.is_some() || gre.is_some() || blu.is_some() {
            check_none(&[bla, whi], "RGB", "HWB")?;
            check_none(&[hue, sat, lig], "RGB", "HSL")?;
            let rgba = rgba.to_rgba();
            Ok(Rgba::new(
                opt_add(rgba.red(), red),
                opt_add(rgba.green(), gre),
                opt_add(rgba.blue(), blu),
                opt_add(rgba.alpha(), a_adj),
                rgba.source(),
            )
            .into())
        } else if bla.is_some() || whi.is_some() {
            check_none(&[sat, lig], "HSL", "HWB")?;
            let hwba = rgba.to_hwba();
            Ok(Hwba::new(
                opt_add(hwba.hue(), hue),
                opt_add(hwba.whiteness(), whi),
                opt_add(hwba.blackness(), bla),
                opt_add(hwba.alpha(), a_adj),
            )
            .into())
        } else {
            let hsla = rgba.to_hsla();
            Ok(Hsla::new(
                opt_add(hsla.hue(), hue),
                opt_add(hsla.sat(), sat),
                opt_add(hsla.lum(), lig),
                opt_add(hsla.alpha(), a_adj),
                hsla.hsla_format,
            )
            .into())
        }
    });
    def_va!(f, scale(color, kwargs), |s| {
        let cmb = |orig: Rational, x: Option<Rational>, max: Rational| match x
        {
            None => orig,
            Some(x) => {
                if x.is_positive() {
                    orig + (max - orig) * x
                } else {
                    orig + orig * x
                }
            }
        };
        let one: Rational = one();
        let ff = Rational::from_integer(255);

        let rgba: Color = s.get(name!(color))?;
        let mut args = s.get_map(name!(kwargs), CallArgs::from_value)?;
        if !args.positional.is_empty() {
            return Err(CallError::msg("Only one positional argument is allowed. \
                                       All other arguments must be passed by name."));
        }
        let red = take_opt(&mut args, name!(red), check_pct_expl_pm)?;
        let gre = take_opt(&mut args, name!(green), check_pct_expl_pm)?;
        let blu = take_opt(&mut args, name!(blue), check_pct_expl_pm)?;
        let sat = take_opt(&mut args, name!(saturation), check_pct_expl_pm)?;
        let lig = take_opt(&mut args, name!(lightness), check_pct_expl_pm)?;
        let bla = take_opt(&mut args, name!(blackness), check_pct_expl_pm)?;
        let whi = take_opt(&mut args, name!(whiteness), check_pct_expl_pm)?;
        let a_adj = take_opt(&mut args, name!(alpha), check_pct_expl_pm)?;
        args.check_no_named().map_err(CallError::msg)?;

        if red.is_some() || gre.is_some() || blu.is_some() {
            check_none(&[bla, whi], "RGB", "HWB")?;
            check_none(&[sat, lig], "RGB", "HSL")?;
            let rgba = rgba.to_rgba();
            Ok(Rgba::new(
                cmb(rgba.red(), red, ff),
                cmb(rgba.green(), gre, ff),
                cmb(rgba.blue(), blu, ff),
                cmb(rgba.alpha(), a_adj, one),
                RgbFormat::Name,
            )
            .into())
        } else if bla.is_none() && whi.is_none() {
            let hsla = rgba.to_hsla();
            Ok(Hsla::new(
                hsla.hue(),
                cmb(hsla.sat(), sat, one),
                cmb(hsla.lum(), lig, one),
                cmb(hsla.alpha(), a_adj, one),
                hsla.hsla_format,
            )
            .into())
        } else {
            check_none(&[sat, lig], "HSL", "HWB")?;
            let hwba = rgba.to_hwba();
            Ok(Hwba::new(
                hwba.hue(),
                cmb(hwba.whiteness(), whi, one),
                cmb(hwba.blackness(), bla, one),
                cmb(hwba.alpha(), a_adj, one),
            )
            .into())
        }
    });

    def!(f, opacity(color), |s| match s.get(name!(color))? {
        Value::Color(ref col, _) => Ok(Value::scalar(col.get_alpha())),
        v => Ok(make_call("opacity", vec![v])),
    });
    def!(f, alpha(color), |s| {
        let v = s.get(name!(color))?;
        if ok_as_filterarg(&v) {
            Ok(make_call("alpha", vec![v]))
        } else {
            let color = Color::try_from(v).named(name!(color))?;
            Ok(Value::scalar(color.get_alpha()))
        }
    });

    def_va!(f, change(color, kwargs), |s| {
        let rgba: Color = s.get(name!(color))?;
        let mut args = s.get_map(name!(kwargs), CallArgs::from_value)?;
        if !args.positional.is_empty() {
            return Err(CallError::msg("Only one positional argument is allowed. \
                                       All other arguments must be passed by name."));
        }
        let red = take_opt(&mut args, name!(red), check_channel_range)?;
        let gre = take_opt(&mut args, name!(green), check_channel_range)?;
        let blu = take_opt(&mut args, name!(blue), check_channel_range)?;
        let hue = take_opt(&mut args, name!(hue), check_rational)?;
        let sat =
            take_opt(&mut args, name!(saturation), check_pct_opt_range)?;
        let lig = take_opt(&mut args, name!(lightness), check_pct_opt_range)?;
        let bla = take_opt(&mut args, name!(blackness), check_expl_pct)?;
        let whi = take_opt(&mut args, name!(whiteness), check_expl_pct)?;
        let alp = take_opt(&mut args, name!(alpha), check_alpha_range)?;
        args.check_no_named().map_err(CallError::msg)?;

        if red.is_some() || gre.is_some() || blu.is_some() {
            check_none(&[hue, sat, lig], "RGB", "HSL")?;
            check_none(&[bla, whi], "RGB", "HWB")?;
            let rgba = rgba.to_rgba();
            Ok(Rgba::new(
                red.unwrap_or_else(|| rgba.red()),
                gre.unwrap_or_else(|| rgba.green()),
                blu.unwrap_or_else(|| rgba.blue()),
                alp.unwrap_or_else(|| rgba.alpha()),
                rgba.source(),
            )
            .into())
        } else if bla.is_none() && whi.is_none() {
            let hsla = rgba.to_hsla();
            Ok(Hsla::new(
                hue.unwrap_or_else(|| hsla.hue()),
                sat.unwrap_or_else(|| hsla.sat()),
                lig.unwrap_or_else(|| hsla.lum()),
                alp.unwrap_or_else(|| hsla.alpha()),
                false, // hsla.hsla_format,
            )
            .into())
        } else {
            check_none(&[sat, lig], "HSL", "HWB")?;
            let hwba = rgba.to_hwba();
            Ok(Hwba::new(
                hue.unwrap_or_else(|| hwba.hue()),
                whi.unwrap_or_else(|| hwba.whiteness()),
                bla.unwrap_or_else(|| hwba.blackness()),
                alp.unwrap_or_else(|| hwba.alpha()),
            )
            .into())
        }
    });
    def!(f, ie_hex_str(color), |s| {
        let (r, g, b, alpha) =
            Color::to_rgba(&s.get(name!(color))?).to_bytes();
        Ok(format!("#{:02X}{:02X}{:02X}{:02X}", alpha, r, g, b).into())
    });
}

pub fn expose(m: &Scope, global: &mut FunctionMap) {
    for (gname, lname) in &[
        (name!(adjust_color), name!(adjust)),
        (name!(alpha), name!(alpha)),
        (name!(opacity), name!(opacity)),
        (name!(change_color), name!(change)),
        (name!(ie_hex_str), name!(ie_hex_str)),
        (name!(scale_color), name!(scale)),
    ] {
        global.insert(gname.clone(), m.get_lfunction(lname));
    }
    let mut f = Scope::builtin_module("sass:color");
    def!(f, fade_in(color, amount), |s| {
        let mut col: Color = s.get(name!(color))?;
        let amount = s.get_map(name!(amount), check_alpha_range)?;
        col.set_alpha(col.get_alpha() + amount);
        Ok(col.into())
    });
    def!(f, fade_out(color, amount), |s| {
        let mut col: Color = s.get(name!(color))?;
        let amount = s.get_map(name!(amount), check_alpha_range)?;
        col.set_alpha(col.get_alpha() - amount);
        Ok(col.into())
    });
    for (gname, lname) in &[
        (name!(fade_in), name!(fade_in)),
        (name!(fade_out), name!(fade_out)),
        (name!(opacify), name!(fade_in)),
        (name!(transparentize), name!(fade_out)),
    ] {
        global.insert(gname.clone(), f.get_lfunction(lname));
    }
}

fn check_none(
    args: &[Option<Rational>],
    kind: &str,
    with_kind: &str,
) -> Result<(), CallError> {
    if args.iter().all(|v| v.is_none()) {
        Ok(())
    } else {
        Err(CallError::msg(format!(
            "{} parameters may not be passed along with {} parameters.",
            kind, with_kind
        )))
    }
}

fn take_opt<T, F>(
    args: &mut CallArgs,
    name: Name,
    check: F,
) -> Result<Option<T>, CallError>
where
    F: Fn(Value) -> Result<T, String>,
{
    args.named.remove(&name).map(check).transpose().named(name)
}

fn check_pct_pm(v: Value) -> Result<Rational, String> {
    let val = Numeric::try_from(v)?;
    if val.value.clone().abs() > 100.into() {
        Err(expected_to(&val, "be within -100% and 100%"))
    } else {
        Ok(val.as_ratio()? / 100)
    }
}
fn check_pct_expl_pm(v: Value) -> Result<Rational, String> {
    let val = Numeric::try_from(v)?;
    if !val.unit.is_percent() {
        return Err(expected_to(&val, "have unit \"%\""));
    }
    if val.value.clone().abs() > 100.into() {
        Err(expected_to(&val, "be within -100% and 100%"))
    } else {
        Ok(val.as_ratio()? / 100)
    }
}

fn check_pct_opt_range(v: Value) -> Result<Rational, String> {
    let val = Numeric::try_from(v)?;
    if val.value < zero() || val.value > 100.into() {
        Err(expected_to(&val, "be within 0% and 100%"))
    } else {
        Ok(val.as_ratio()? / 100)
    }
}

fn ok_as_filterarg(v: &Value) -> bool {
    match v {
        Value::Literal(ref s) if s.quotes().is_none() => {
            use crate::parser::strings::unitname;
            use crate::parser::{code_span, util::opt_spacelike};
            use nom::bytes::complete::tag;
            use nom::sequence::tuple;
            let span = code_span(s.value().as_ref());
            tuple((unitname, opt_spacelike, tag("=")))(span).is_ok()
        }
        Value::List(..) => true,
        _ => false,
    }
}

#[cfg(test)]
mod test {
    use crate::variablescope::test::do_evaluate;

    #[test]
    fn ie_hex_str_a() {
        assert_eq!(do_evaluate(&[], b"ie-hex-str(#abc);"), "#FFAABBCC")
    }
    #[test]
    fn ie_hex_str_b() {
        assert_eq!(do_evaluate(&[], b"ie-hex-str(#3322BB);"), "#FF3322BB")
    }
    #[test]
    fn ie_hex_str_c() {
        assert_eq!(
            do_evaluate(&[], b"ie-hex-str(rgba(0, 255, 0, 0.5));"),
            "#8000FF00"
        )
    }
}
