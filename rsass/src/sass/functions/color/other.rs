use super::{
    CallError, CheckedArg, FunctionMap, Name, check_alpha_num,
    check_alpha_pm, check_alpha_range, check_channel_pm, check_channel_range,
    check_expl_pct, check_hue, check_pct, check_pct_range, expected_to,
};
use crate::Scope;
use crate::css::{CallArgs, CssString, Value, is_not};
use crate::output::Format;
use crate::sass::functions::color::is_special;
use crate::sass::functions::num_or_special::NumOrSpecial;
use crate::value::{
    Color, Hsla, Hwba, ListSeparator, Numeric, Operator, Quotes, RgbFormat,
    Rgba,
};
use std::collections::BTreeMap;

/// The argument to the `color` function.
struct ColDesc {
    args: Vec<Value>,
    alpha: Option<Value>,
}

impl ColDesc {
    fn into_arg(self) -> Result<Value, CallError> {
        let inner_list =
            Value::List(self.args, Some(ListSeparator::Space), false);
        let alpha = if let Some(alpha) = self.alpha {
            match NumOrSpecial::try_from(alpha).named(name!(description))? {
                NumOrSpecial::Num(n) => {
                    if check_alpha_num(n.clone()).named(name!(alpha))? >= 1.0
                    {
                        None
                    } else if n > Numeric::scalar(0.) {
                        Some(n.clone().into())
                    } else {
                        Some(Numeric::scalar(0).into())
                    }
                }
                NumOrSpecial::Special(x) => Some(x),
            }
        } else {
            None
        };
        if let Some(alpha) = alpha {
            Ok(Value::List(
                vec![inner_list, alpha],
                Some(ListSeparator::Slash),
                false,
            ))
        } else {
            Ok(inner_list)
        }
    }
}

impl TryFrom<Value> for ColDesc {
    type Error = String;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let (args, alpha) = match value {
            c if is_special(&c) => Ok((vec![c], None)),
            l @ Value::List(_, _, true) => Err(format!(
                "Expected an unbracketed list, was {}",
                l.introspect()
            )),
            l @ Value::List(_, Some(ListSeparator::Comma), _) => {
                Err(format!(
                    "Expected a space- or slash-separated list, was ({})",
                    l.introspect()
                ))
            }
            Value::List(v, Some(ListSeparator::Slash), _) => match &v[..] {
                [Value::List(_, _, true), _] => {
                    Err("Expected an unbracketed list.".to_string())
                }
                [l @ Value::List(_, Some(i_s), _), _]
                    if *i_s != ListSeparator::Space =>
                {
                    Err(format!(
                        "Expected a space-separated list, was ({})",
                        l.introspect()
                    ))
                }
                [Value::List(inner, _, _), a] => Ok((
                    valid_col_arg_noslash(inner.clone())?,
                    Some(a.clone()),
                )),
                other => {
                    let n = other.len();
                    Err(format!(
                        "Only 2 slash-separated elements allowed, but {n} {} passed.",
                        if n == 1 { "was" } else { "were" },
                    ))
                }
            },
            Value::List(vec, _, false) => match &vec[..] {
                [kind, r, g, Value::BinOp(op)]
                    if op.op() == Operator::Div =>
                {
                    Ok((
                        valid_col_arg_noslash(vec![
                            kind.clone(),
                            r.clone(),
                            g.clone(),
                            op.a().clone(),
                        ])?,
                        Some(op.b().clone()),
                    ))
                }
                other => Ok((valid_col_arg_noslash(other.to_vec())?, None)),
            },
            item => Ok((valid_col_arg_noslash(vec![item])?, None)),
        }?;
        Ok(Self { args, alpha })
    }
}

enum ColorOrCall {
    Color(Color),
    Call(String, CallArgs),
}
impl TryFrom<Value> for ColorOrCall {
    type Error = String;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Call(name, args) => Ok(Self::Call(name, args)),
            Value::Color(color, _x) => Ok(Self::Color(color)),
            other => Err(is_not(&other, "a color")),
        }
    }
}

pub fn register(f: &mut Scope) {
    def!(f, color(description), |s| {
        let description = s.get::<ColDesc>(name!(description))?;
        Ok(Value::call("color", [description.into_arg()?]))
    });
    def!(f, space(color), |s| {
        let color = s.get::<ColorOrCall>(name!(color))?;
        match color {
            ColorOrCall::Color(color) => Ok(color.space().into()),
            ColorOrCall::Call(name, args) => match name.as_str() {
                "lab" | "lch" => Ok(name.into()),
                "color" => match args.get_single() {
                    Ok(Value::List(v, _, _)) => match v.get(0) {
                        Some(s @ Value::Literal(_)) => Ok(s.clone()),
                        _ => Err("unexpected color arg").named(name!(color)),
                    },
                    _ => Err("unexpected color arg").named(name!(color)),
                },
                _ => Err("not a color").named(name!(color)),
            },
        }
    });
    def!(f, is_missing(color, channel), |s| {
        let color = s.get::<ColorOrCall>(name!(color))?;
        let channel = s.get::<CssString>(name!(channel))?;
        if channel.quotes() == Quotes::None {
            return Err(format!("Expected {channel} to be a quoted string."))
                .named(name!(channel));
        }
        match color {
            ColorOrCall::Call(f, args) if f == "color" => {
                Err(format!("Proper color({args}) not handled yet"))
                    .named(name!(color))
            }
            ColorOrCall::Call(f, args) => {
                let channels = BTreeMap::from_iter([
                    ("lch", &["lightness", "chroma", "hue"]),
                    ("lab", &["l", "a", "b"]),
                    ("rgb", &["red", "green", "blue"]),
                ]);
                if let Some(channels) = channels.get(&f.as_str()) {
                    if let Some(n) = channels
                        .iter()
                        .enumerate()
                        .find(|(_, c)| **c == channel.value())
                        .map(|(i, _)| i)
                    {
                        let is_none = dbg!(arg_channels(&dbg!(args)))
                            .named(name!(color))?
                            .0
                            .get(dbg!(n))
                            == Some(&Value::Literal(CssString::from("none")));
                        Ok(is_none.into()) // Can it be missing?  Explicit "none"?
                    } else {
                        Err(format!("Color fn {f} doesn\'t have a channel named {channel}."))
                            .named(name!(channel))
                    }
                } else {
                    Err(format!("Color fn {f} not handled yet"))
                        .named(name!(color))
                }
            }
            ColorOrCall::Color(color) => {
                // This kind of color don't have missing components, either false or error.
                let existing = match &color {
                    Color::Rgba(_) => ["red", "green", "blue"],
                    Color::Hsla(_) => ["hue", "sat", "val"],
                    Color::Hwba(_) => ["hue", "white", "black"],
                };
                if existing.iter().any(|c| *c == channel.value()) {
                    Ok(false.into())
                } else {
                    Err(format!(
                        "Color {} doesn\'t have a channel named {channel}.",
                        color.format(Format::default())
                    ))
                    .named(name!(channel))
                }
            }
        }
        //Ok(false.into())
    });
    def_va!(f, adjust(color, kwargs), |s| {
        fn opt_add(a: f64, b: Option<f64>) -> f64 {
            if let Some(b) = b { a + b } else { a }
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
                        || sat > 1.
                        || !(0. ..=1.).contains(&lum),
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
        let cmb = |orig: f64, x: Option<f64>, max: f64| match x {
            None => orig,
            Some(x) => {
                if x.is_sign_positive() {
                    orig + (max - orig) * x
                } else {
                    orig + orig * x
                }
            }
        };
        let one = 1.0;
        let ff = 255.0;

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
        let sat = take_opt(&mut args, name!(saturation), check_pct_range)?;
        let lig = take_opt(&mut args, name!(lightness), check_pct_range)?;
        if sat.is_some() || lig.is_some() {
            no_more_in_space(&args, "hsl")?;
        }
        let bla = take_opt(&mut args, name!(blackness), check_expl_pct)?;
        let whi = take_opt(&mut args, name!(whiteness), check_expl_pct)?;
        no_more_in_space(&args, "rgb")?;

        if bla.is_some() || whi.is_some() {
            let hwba = rgba.to_hwba();
            let hwba = Hwba::new(
                hue.unwrap_or_else(|| hwba.hue()),
                whi.unwrap_or_else(|| hwba.whiteness()),
                bla.unwrap_or_else(|| hwba.blackness()),
                alp.unwrap_or_else(|| hwba.alpha()),
            );
            Ok(Rgba::from(&hwba).into())
        } else if hue.is_some() || sat.is_some() || lig.is_some() {
            let hsla = rgba.to_hsla();
            Ok(Hsla::new(
                hue.unwrap_or_else(|| hsla.hue()),
                sat.unwrap_or_else(|| hsla.sat()),
                lig.unwrap_or_else(|| hsla.lum()),
                alp.unwrap_or_else(|| hsla.alpha()),
                hsla.hsla_format,
            )
            .into())
        } else if let Some(alp) = alp {
            let mut col = rgba;
            col.set_alpha(alp);
            Ok(col.into())
        } else {
            Ok(rgba.into())
        }
    });
    def!(f, ie_hex_str(color), |s| {
        let (r, g, b, alpha) =
            Color::to_rgba(&s.get(name!(color))?).to_bytes();
        Ok(format!("#{alpha:02X}{r:02X}{g:02X}{b:02X}").into())
    });
}

fn arg_channels(
    value: &CallArgs,
) -> Result<(Vec<Value>, Option<Value>), String> {
    if let Ok(arg) = value.get_single() {
        channels_from_value(arg)
    } else {
        Ok((value.positional.clone(), None))
    }
}

fn channels_from_value(
    value: &Value,
) -> Result<(Vec<Value>, Option<Value>), String> {
    match value {
        c if is_special(&c) => Ok((vec![c.clone()], None)),
        l @ Value::List(_, _, true) => Err(format!(
            "Expected an unbracketed list, was {}",
            l.introspect()
        )),
        l @ Value::List(_, Some(ListSeparator::Comma), _) => Err(format!(
            "Expected a space- or slash-separated list, was ({})",
            l.introspect()
        )),
        Value::List(v, Some(ListSeparator::Slash), _) => match &v[..] {
            [Value::List(_, _, true), _] => {
                Err(format!("Expected an unbracketed list."))
            }
            [l @ Value::List(_, Some(i_s), _), _]
                if *i_s != ListSeparator::Space =>
            {
                Err(format!(
                    "Expected a space-separated list, was ({})",
                    l.introspect()
                ))
            }
            [Value::List(inner, _, _), a] => {
                Ok((inner.clone(), Some(a.clone())))
            }
            other => {
                let n = other.len();
                Err(format!(
                    "Only 2 slash-separated elements allowed, but {n} {} passed.",
                    if n == 1 { "was" } else { "were" },
                ))
            }
        },
        Value::List(vec, _, false) => match &vec[..] {
            [kind, r, g, Value::BinOp(op)] if op.op() == Operator::Div => {
                Ok((
                    vec![kind.clone(), r.clone(), g.clone(), op.a().clone()],
                    Some(op.b().clone()),
                ))
            }
            other => Ok((other.to_vec(), None)),
        },
        item => Ok((vec![item.clone()], None)),
    }
}

pub fn expose(m: &Scope, global: &mut FunctionMap) {
    for (gname, lname) in &[
        (name!(color), name!(color)),
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

fn valid_col_arg_noslash(args: Vec<Value>) -> Result<Vec<Value>, String> {
    match args.get(0) {
        Some(Value::Literal(s)) => {
            if s.quotes() != Quotes::None {
                return Err(expected_to(s.clone(), "be an unquoted string"));
            }
            let s = s.value();
            // TODO: A much longer list is needed!
            //let known = ["from", "srgb", "srgb-linear", "prophoto-rgb"];
            let expected_len = BTreeMap::from_iter([
                ("a98-rgb", 3),
                ("srgb", 3),
                ("srgb-linear", 3),
                ("prophoto-rgb", 3),
            ]);
            if !s.eq_ignore_ascii_case("from") {
                if let Some(ex) = expected_len.get(&s).copied() {
                    let l = args.len() - 1;
                    if l != ex {
                        let s = s.to_owned();
                        let v = if args.len() == 1 {
                            args.get(0).unwrap().introspect()
                        } else {
                            format!(
                                "({})",
                                Value::List(args, None, false).introspect()
                            )
                        };
                        return Err(format!(
                            "The {s} color space has {ex} channels but {v} has {l}."
                        ));
                    }
                } else {
                    return Err(format!("Unknown color space {s:?}."));
                }
            }
        }
        Some(x) => {
            return Err(format!("{} is not a string.", x.introspect()));
        }
        None => return Err(format!("empty??")),
    }
    Ok(args)
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
        Err(CallError::msg(format!(
            "${name}: Color space {space} doesn\'t have a channel with this name."
        )))
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

fn check_pct_expl_pm(v: Value) -> Result<f64, String> {
    let val = Numeric::try_from(v)?;
    if !val.unit.is_percent() {
        return Err(expected_to(val, "have unit \"%\""));
    }
    if f64::from(val.value.abs()) > 100. {
        Err(expected_to(val, "be within -100% and 100%"))
    } else {
        Ok(f64::from(val.value) / 100.)
    }
}

fn ok_as_filterarg(v: &Value) -> bool {
    match v {
        Value::Literal(s) if s.quotes().is_none() => {
            use crate::parser::strings::unitname;
            use crate::parser::{code_span, util::opt_spacelike};
            use nom::Parser as _;
            use nom::bytes::complete::tag;
            let span = code_span(s.value().as_ref());
            (unitname, opt_spacelike, tag("="))
                .parse(span.borrow())
                .is_ok()
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
