use super::{
    check_alpha_pm, check_alpha_range, check_channel_pm, check_channel_range,
    check_expl_pct, check_hue, expected_to, CallError, CheckedArg,
    FunctionMap, Name,
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
        let color: Color = s.get(name!(color))?;
        let mut args = s.get_map(name!(kwargs), CallArgs::from_value)?;
        no_more_positional(&args)?;
        let a_adj = take_opt(&mut args, name!(alpha), check_alpha_pm)?;

        let red = take_opt(&mut args, name!(red), check_channel_pm)?;
        let gre = take_opt(&mut args, name!(green), check_channel_pm)?;
        let blu = take_opt(&mut args, name!(blue), check_channel_pm)?;
        if red.is_some() || gre.is_some() || blu.is_some() {
            no_more_in_space(&args, "rgb")?;
            let rgba = color.to_rgba();
            Ok(Rgba::new(
                opt_add(rgba.red(), red),
                opt_add(rgba.green(), gre),
                opt_add(rgba.blue(), blu),
                opt_add(rgba.alpha(), a_adj),
                rgba.source(),
            )
            .into())
        } else {
            let hue = take_opt(&mut args, name!(hue), check_hue)?;
            let sat = take_opt(&mut args, name!(saturation), check_pct)?;
            let lig = take_opt(&mut args, name!(lightness), check_pct)?;
            if sat.is_some() || lig.is_some() {
                no_more_in_space(&args, "hsl")?;
            }
            let bla =
                take_opt(&mut args, name!(blackness), check_pct_expl_pm)?;
            let whi =
                take_opt(&mut args, name!(whiteness), check_pct_expl_pm)?;
            no_more_in_space(&args, "rgb")?;
            if bla.is_some() || whi.is_some() {
                let hwba = color.to_hwba();
                let hwba = Hwba::new(
                    opt_add(hwba.hue(), hue),
                    opt_add(hwba.whiteness(), whi),
                    opt_add(hwba.blackness(), bla),
                    opt_add(hwba.alpha(), a_adj),
                );
                Ok(Rgba::from(&hwba).into())
            } else if hue.is_some() || sat.is_some() || lig.is_some() {
                let hsla = color.to_hsla();
                let sat = opt_add(hsla.sat(), sat);
                let lum = opt_add(hsla.lum(), lig);
                Ok(Hsla::new(
                    opt_add(hsla.hue(), hue),
                    sat,
                    lum,
                    opt_add(hsla.alpha(), a_adj),
                    hsla.hsla_format
                        || sat > one()
                        || lum > one()
                        || lum < zero(),
                )
                .into())
            } else {
                let mut color = color.clone();
                color.set_alpha(opt_add(color.get_alpha(), a_adj));
                Ok(color.into())
            }
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

        let color: Color = s.get(name!(color))?;
        let mut args = s.get_map(name!(kwargs), CallArgs::from_value)?;
        no_more_positional(&args)?;
        let a_adj = take_opt(&mut args, name!(alpha), check_pct_expl_pm)?;

        take_opt(&mut args, name!(hue), check_none_scalable)?;

        let red = take_opt(&mut args, name!(red), check_pct_expl_pm)?;
        let gre = take_opt(&mut args, name!(green), check_pct_expl_pm)?;
        let blu = take_opt(&mut args, name!(blue), check_pct_expl_pm)?;
        if red.is_some() || gre.is_some() || blu.is_some() {
            no_more_in_space(&args, "rgb")?;
            let rgba = color.to_rgba();
            let rgba = Rgba::new(
                cmb(rgba.red(), red, ff),
                cmb(rgba.green(), gre, ff),
                cmb(rgba.blue(), blu, ff),
                cmb(rgba.alpha(), a_adj, one),
                RgbFormat::Name,
            );
            return Ok(rgba.into());
        }
        let sat = take_opt(&mut args, name!(saturation), check_pct_expl_pm)?;
        let lig = take_opt(&mut args, name!(lightness), check_pct_expl_pm)?;
        if sat.is_some() || lig.is_some() {
            no_more_in_space(&args, "hsl")?;
        }
        let bla = take_opt(&mut args, name!(blackness), check_pct_expl_pm)?;
        let whi = take_opt(&mut args, name!(whiteness), check_pct_expl_pm)?;
        no_more_in_space(&args, "rgb")?;

        if bla.is_none() && whi.is_none() {
            let hsla = color.to_hsla();
            Ok(Hsla::new(
                hsla.hue(),
                cmb(hsla.sat(), sat, one),
                cmb(hsla.lum(), lig, one),
                cmb(hsla.alpha(), a_adj, one),
                hsla.hsla_format,
            )
            .into())
        } else {
            let is_rgb = color.is_rgb();
            let hwba = color.to_hwba();
            let hwba = Hwba::new(
                hwba.hue(),
                cmb(hwba.whiteness(), whi, one),
                cmb(hwba.blackness(), bla, one),
                cmb(hwba.alpha(), a_adj, one),
            );
            if is_rgb {
                Ok(Rgba::from(&hwba).into())
            } else {
                Ok(hwba.into())
            }
        }
    });

    def!(f, opacity(color), |s| match s.get(name!(color))? {
        Value::Color(ref col, _) => Ok(Value::scalar(col.get_alpha())),
        v => Ok(Value::call("opacity", [v])),
    });
    def!(f, alpha(color), |s| {
        let v = s.get(name!(color))?;
        if ok_as_filterarg(&v) {
            Ok(Value::call("alpha", [v]))
        } else {
            let color = Color::try_from(v).named(name!(color))?;
            Ok(Value::scalar(color.get_alpha()))
        }
    });

    def_va!(f, change(color, kwargs), |s| {
        let rgba: Color = s.get(name!(color))?;
        let mut args = s.get_map(name!(kwargs), CallArgs::from_value)?;
        no_more_positional(&args)?;
        let alp = take_opt(&mut args, name!(alpha), check_alpha_range)?;

        let red = take_opt(&mut args, name!(red), check_channel_range)?;
        let gre = take_opt(&mut args, name!(green), check_channel_range)?;
        let blu = take_opt(&mut args, name!(blue), check_channel_range)?;
        if red.is_some() || gre.is_some() || blu.is_some() {
            no_more_in_space(&args, "rgb")?;
            let rgba = rgba.to_rgba();
            let rgba = Rgba::new(
                red.unwrap_or_else(|| rgba.red()),
                gre.unwrap_or_else(|| rgba.green()),
                blu.unwrap_or_else(|| rgba.blue()),
                alp.unwrap_or_else(|| rgba.alpha()),
                rgba.source(),
            );
            return Ok(rgba.into());
        }
        let hue = take_opt(&mut args, name!(hue), check_hue)?;
        let sat =
            take_opt(&mut args, name!(saturation), check_pct_opt_range)?;
        let lig = take_opt(&mut args, name!(lightness), check_pct_opt_range)?;
        if sat.is_some() || lig.is_some() {
            no_more_in_space(&args, "hsl")?;
        }
        let bla = take_opt(&mut args, name!(blackness), check_expl_pct)?;
        let whi = take_opt(&mut args, name!(whiteness), check_expl_pct)?;
        no_more_in_space(&args, "rgb")?;

        if bla.is_none() && whi.is_none() {
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
            let hwba = rgba.to_hwba();
            let hwba = Hwba::new(
                hue.unwrap_or_else(|| hwba.hue()),
                whi.unwrap_or_else(|| hwba.whiteness()),
                bla.unwrap_or_else(|| hwba.blackness()),
                alp.unwrap_or_else(|| hwba.alpha()),
            );
            Ok(Rgba::from(&hwba).into())
        }
    });
    def!(f, ie_hex_str(color), |s| {
        let (r, g, b, alpha) =
            Color::to_rgba(&s.get(name!(color))?).to_bytes();
        Ok(format!("#{alpha:02X}{r:02X}{g:02X}{b:02X}").into())
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

fn check_none_scalable(_: Value) -> Result<(), String> {
    Err("Channel isn't scalable.".into())
}

fn no_more_positional(args: &CallArgs) -> Result<(), CallError> {
    if !args.positional.is_empty() {
        Err(CallError::msg(
            "Only one positional argument is allowed. \
             All other arguments must be passed by name.",
        ))
    } else {
        Ok(())
    }
}

fn no_more_in_space(args: &CallArgs, space: &str) -> Result<(), CallError> {
    if let Some((name, _)) = args.named.iter().next() {
        Err(CallError::msg(format!("${name}: Color space {space} doesn\'t have a channel with this name.")))
    } else {
        Ok(())
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

fn check_pct(v: Value) -> Result<Rational, String> {
    Ok(Numeric::try_from(v)?.as_ratio()? / 100)
}
fn check_pct_expl_pm(v: Value) -> Result<Rational, String> {
    let val = Numeric::try_from(v)?;
    if !val.unit.is_percent() {
        return Err(expected_to(val, "have unit \"%\""));
    }
    if val.value.abs() > 100.into() {
        Err(expected_to(val, "be within -100% and 100%"))
    } else {
        Ok(val.as_ratio()? / 100)
    }
}

fn check_pct_opt_range(v: Value) -> Result<Rational, String> {
    let val = Numeric::try_from(v)?;
    if val.value < zero() || val.value > 100.into() {
        Err(expected_to(val, "be within 0% and 100%"))
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
            let b = tuple((unitname, opt_spacelike, tag("=")))(span.borrow())
                .is_ok();
            b
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
