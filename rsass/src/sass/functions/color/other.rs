use super::{
    CallError, CheckedArg, FunctionMap, Name, check_alpha_num,
    check_alpha_pm, check_alpha_range, check_channel_pm, check_channel_range,
    check_expl_pct, check_hue, check_pct, check_pct_range, expected_to,
    is_special_not_none,
};
use crate::Scope;
use crate::css::{CallArgs, CssString, Value, is_not};
use crate::output::Format;
use crate::sass::ResolvedArgs;
use crate::sass::functions::color::is_special;
use crate::sass::functions::num_or_special::NumOrSpecial;
use crate::value::{
    Color, Hsla, Hwba, ListSeparator, Numeric, Operator, Quotes, RgbFormat,
    Rgba, Unit,
};
use std::collections::BTreeMap;
use std::sync::LazyLock;

/// The argument to the `color` function.
#[derive(Debug)]
struct ColDesc {
    data: WildChannels,
}

impl ColDesc {
    fn into_call_or_str(self) -> Value {
        self.data.into_call_or_str("color")
    }
}

impl TryFrom<Value> for ColDesc {
    type Error = ChannelErr;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let mut data = WildChannels::try_from(value)?;
        data.args = valid_col_arg_noslash(data.args)?;
        Ok(Self { data })
    }
}

#[derive(Debug)]
struct WildChannels {
    args: Vec<Value>,
    alpha: Option<Value>,
}

impl WildChannels {
    fn into_arg(self) -> Value {
        let inner_list =
            Value::List(self.args, Some(ListSeparator::Space), false);
        if let Some(alpha) = self.alpha {
            Value::List(
                vec![inner_list, alpha],
                Some(ListSeparator::Slash),
                false,
            )
        } else {
            inner_list
        }
    }

    fn into_call_or_str(self, fn_name: &str) -> Value {
        let is_from = self.args.first().is_some_and(is_from);
        if is_from
            || self.args.iter().any(is_special_not_none)
            || self.alpha.as_ref().is_some_and(is_special_not_none)
        {
            let inner =
                Value::List(self.args, Some(ListSeparator::Space), false);
            let inner = inner.format(Format::default());
            Value::from(if let Some(alpha) = self.alpha {
                let alpha = alpha.format(Format::default());
                if is_from {
                    format!("{fn_name}({inner} / {alpha})")
                } else {
                    format!("{fn_name}({inner}/{alpha})")
                }
            } else {
                format!("{fn_name}({inner})")
            })
        } else {
            Value::call(fn_name, [self.into_arg()])
        }
    }
}

/// An error in channel arguments.
///
/// It should either be reported on the parameter that holds the entire set
/// of channels or on a specific channel.
struct ChannelErr {
    err: String,
    channel: Option<Name>,
}
impl ChannelErr {
    fn named(err: impl Into<String>, channel: Name) -> Self {
        Self {
            err: err.into(),
            channel: Some(channel),
        }
    }
}

impl<T: Into<String>> From<T> for ChannelErr {
    fn from(value: T) -> Self {
        Self {
            err: value.into(),
            channel: None,
        }
    }
}
impl<T> CheckedArg<T> for Result<T, ChannelErr> {
    /// The `name` is applied if there isn't a channel name in the `ChannelErr`.
    fn named(self, name: Name) -> Result<T, CallError> {
        self.map_err(|e| {
            CallError::BadArgument(e.channel.unwrap_or(name), e.err)
        })
    }
}

impl TryFrom<CallArgs> for WildChannels {
    type Error = ChannelErr;

    fn try_from(value: CallArgs) -> Result<Self, Self::Error> {
        match value.get_single() {
            Ok(value) => value.clone().try_into(),
            Err(_) => Err("expected single argument".into()),
        }
    }
}

impl TryFrom<Value> for WildChannels {
    type Error = ChannelErr;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let (args, alpha) = match value {
            Value::BinOp(op) if op.op() == Operator::Div => {
                Ok((vec![op.a().clone()], Some(op.b().clone())))
            }
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
                [Value::List(inner, _, _), a] => {
                    Ok((inner.clone(), Some(a.clone())))
                }
                [item, a] => Ok((vec![item.clone()], Some(a.clone()))),
                other => {
                    let n = other.len();
                    Err(format!(
                        "Only 2 slash-separated elements allowed, but {n} {} passed.",
                        if n == 1 { "was" } else { "were" },
                    ))
                }
            },
            Value::List(mut vec, _, false) => {
                if vec.first().is_some_and(is_from) {
                    Ok((vec, None))
                } else {
                    match vec.pop() {
                        Some(Value::BinOp(op))
                            if op.op() == Operator::Div =>
                        {
                            vec.push(op.a().clone());
                            Ok((vec, Some(op.b().clone())))
                        }
                        Some(other) => {
                            vec.push(other);
                            Ok((vec, None))
                        }
                        None => Ok((vec, None)),
                    }
                }
            }
            item => Ok((vec![item], None)),
        }?;
        if args.is_empty() {
            return Err("Color component list may not be empty.".into());
        }

        let alpha = if let Some(alpha) = alpha {
            if args.first().is_some_and(is_from) {
                Some(alpha)
            } else {
                match NumOrSpecial::try_from(alpha)? {
                    NumOrSpecial::Num(n) => {
                        let alpha = check_alpha_num(n).map_err(|e| {
                            ChannelErr::named(e, name!(alpha))
                        })?;
                        if alpha >= 1.0 {
                            None
                        } else {
                            Some(Numeric::scalar(alpha.max(0.)).into())
                        }
                    }
                    NumOrSpecial::Special(x) => Some(x),
                }
            }
        } else {
            None
        };

        Ok(Self { args, alpha })
    }
}

#[derive(Debug)]
struct StrictChannels {
    channels: Vec<Option<Numeric>>,
    alpha: Option<Numeric>, // TODO: Or just f64, since dimension is known?
}

impl StrictChannels {
    fn from_value(
        space: &str,
        value: Value,
    ) -> Result<Result<Self, WildChannels>, ChannelErr> {
        let channels = WildChannels::try_from(value)?;
        Self::check(space, channels.args, channels.alpha)
    }

    fn check(
        space: &str,
        channels: Vec<Value>,
        alpha: Option<Value>,
    ) -> Result<Result<Self, WildChannels>, ChannelErr> {
        if channels.first().is_some_and(is_from) {
            return Ok(Err(WildChannels {
                args: channels,
                alpha,
            }));
        }

        let (_s, ch) = get_space(space)
            .ok_or_else(|| format!("Unknown color space {space:?}"))?;

        let ch_num_msg = {
            let l = channels.len();
            let ex = ch.len();

            let v = if channels.len() == 1 {
                channels.first().unwrap().introspect()
            } else {
                format!(
                    "({})",
                    Value::List(channels.clone(), None, false).introspect()
                )
            };
            format!(
                "The {space} color space has {ex} channels but {v} has {l}."
            )
        };

        let mut argiter = channels.into_iter();
        let mut channels = Vec::new();
        let mut wild = Vec::new();
        for ch in ch {
            if let Some(arg) = argiter.next() {
                match NumOrSpecial::try_from(arg).map_err(|e| {
                    format!(
                        "Expected {} channel to be a number, was {}.",
                        ch.name,
                        e.value().format(Format::default())
                    )
                })? {
                    NumOrSpecial::Num(n) if wild.is_empty() => {
                        channels.push(Some((ch.check)(n).map_err(|s| {
                            ChannelErr::named(s, ch.name.into())
                        })?));
                    }
                    NumOrSpecial::Special(s)
                        if wild.is_empty() && is_none(&s) =>
                    {
                        channels.push(None);
                    }
                    NumOrSpecial::Num(n) => {
                        wild.push(n.into());
                    }
                    NumOrSpecial::Special(s) => {
                        wild.extend(channels_to_value(channels.split_off(0)));
                        wild.push(s);
                    }
                }
            } else if wild.is_empty() {
                return Err(ch_num_msg.into());
            }
        }
        if argiter.next().is_some() {
            return Err(ch_num_msg.into());
        }

        let a = alpha.map(NumOrSpecial::try_from).transpose()?;
        if wild.is_empty()
            && let Some(NumOrSpecial::Num(alpha)) = a
        {
            Ok(Ok(Self {
                channels,
                alpha: Some(alpha),
            }))
        } else if wild.is_empty() && a.is_none() {
            Ok(Ok(Self {
                channels,
                alpha: None,
            }))
        } else if wild.is_empty() {
            Ok(Err(WildChannels {
                args: channels_to_value(channels),
                alpha: a.map(Value::from),
            }))
        } else {
            Ok(Err(WildChannels {
                args: wild,
                alpha: a.map(Value::from),
            }))
        }
    }

    fn into_call(self, fn_name: &str) -> Value {
        Value::call(fn_name, [self.into_arg()])
    }
    fn into_arg(self) -> Value {
        let inner_list = Value::List(
            channels_to_value(self.channels),
            Some(ListSeparator::Space),
            false,
        );
        if let Some(alpha) = self.alpha {
            Value::List(
                vec![inner_list, Value::from(alpha)],
                Some(ListSeparator::Slash),
                false,
            )
        } else {
            inner_list
        }
    }
}
fn channels_to_value(ch: Vec<Option<Numeric>>) -> Vec<Value> {
    ch.into_iter()
        .map(|v| v.map_or_else(|| Value::from("none"), Value::from))
        .collect()
}

#[derive(Debug, Clone)]
pub enum ColorOrCall {
    Color(Color),
    Call(String, CallArgs),
}
impl ColorOrCall {
    fn space(&self) -> Result<String, CallError> {
        match self {
            Self::Color(color) => Ok(color.space().into()),
            Self::Call(name, args) => {
                if name == "color" {
                    match WildChannels::try_from(args.clone())
                        .named(name!(color))?
                        .args
                        .first()
                    {
                        Some(Value::Literal(s)) => {
                            Ok(s.value().to_ascii_lowercase())
                        }
                        _ => Err("unexpected color arg").named(name!(color)),
                    }
                } else if let Some((space, _)) = get_space(name) {
                    Ok(space)
                } else {
                    Err("not a color").named(name!(color))
                }
            }
        }
    }
    fn get_channel(&self, channel: &str) -> Option<f64> {
        if let Self::Color(color) = self {
            color.get_channel(channel).map(|c| c.value.into()).ok()
        } else {
            match find_channel(self.clone(), channel) {
                Ok(Value::Numeric(n, _)) => Some(n.value.into()),
                _ => None,
            }
        }
    }
    pub fn check_legacy(self, name: &str) -> Result<Color, CallError> {
        self.check_legacy_inner(name, "")
    }
    pub fn check_legacy_w(self, name: &str) -> Result<Color, CallError> {
        self.check_legacy_inner(name, " with an explicit $space argument")
    }
    fn check_legacy_inner(
        self,
        name: &str,
        extra: &str,
    ) -> Result<Color, CallError> {
        match self {
            ColorOrCall::Color(c) => Ok(c),
            ColorOrCall::Call(..) => Err(CallError::msg(format!(
                "{name}() is only supported for legacy colors. \
                 Please use color.channel() instead{extra}."
            ))),
        }
    }
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
impl From<ColorOrCall> for Value {
    fn from(value: ColorOrCall) -> Self {
        match value {
            ColorOrCall::Color(color) => color.into(),
            ColorOrCall::Call(name, args) => Value::Call(name, args),
        }
    }
}

fn num_or_pct_of(num: Numeric, total: f64) -> Result<f64, String> {
    if num.is_no_unit() {
        Ok(f64::from(num.value))
    } else if num.unit.is_percent() {
        Ok(f64::from(num.value) / 100. * total)
    } else {
        Err(expected_to(num, "have unit \"%\" or no units"))
    }
}

pub fn register(f: &mut Scope) {
    def!(f, color(description), |s| {
        let description = s.get::<ColDesc>(name!(description))?;
        Ok(description.into_call_or_str())
    });
    def!(f, lab(channels), |s| col_space_fn("lab", s));
    def!(f, oklab(channels), |s| col_space_fn("oklab", s));
    def!(f, lch(channels), |s| col_space_fn("lch", s));
    def!(f, oklch(channels), |s| col_space_fn("oklch", s));

    def!(f, space(color), |s| {
        Ok(s.get::<ColorOrCall>(name!(color))?.space()?.into())
    });
    def!(f, channel(color, channel), |s| {
        let color = s.get::<ColorOrCall>(name!(color))?;
        let channel = s.get_map(name!(channel), ch_channel)?;
        let channel = find_channel(color, &channel)?;
        if is_none(&channel) {
            Ok(Value::scalar(0))
        } else {
            Ok(channel)
        }
    });
    def!(f, is_missing(color, channel), |s| {
        let color = s.get::<ColorOrCall>(name!(color))?;
        let channel = s.get_map(name!(channel), ch_channel)?;
        Ok(is_none(&find_channel(color, &channel)?).into())
    });
    def!(f, is_powerless(color, channel), |s| {
        let color = s.get::<ColorOrCall>(name!(color))?;
        let channel = s.get_map(name!(channel), ch_channel)?;
        Ok(match (color.space()?.as_str(), channel.as_str()) {
            ("hsl", "hue") => color
                .get_channel("saturation")
                .is_some_and(|s| s.abs() < 1e-10)
                .into(),
            ("hsl", "saturation" | "lightness") => false.into(),
            ("hwb", "hue") => {
                ((color.get_channel("blackness").unwrap_or(0.)
                    + color.get_channel("whiteness").unwrap_or(0.))
                    > (100. - 1e-10))
                    .into()
            }
            ("lch" | "oklch", "chroma") => false.into(),
            ("lch" | "oklch", "hue") => color
                .get_channel("chroma")
                .is_some_and(|s| s.abs() < 1e-10)
                .into(),
            ("lab" | "oklab", "l" | "a" | "b") => false.into(),
            (_, ch) => {
                return Err(format!(
                    "Color {} doesn't have a channel named \"{ch}\".",
                    Value::from(color).format(Format::default())
                ))
                .named(name!(channel));
            }
        })
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
                if hwba.blackness() >= 0.
                    && hwba.blackness() <= 1.
                    && hwba.whiteness() >= 0.
                    && hwba.whiteness() <= 1.
                    && hwba.hue().is_finite()
                {
                    Ok(Rgba::from(&hwba).into())
                } else {
                    Ok(hwba.into())
                }
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
            let color = ColorOrCall::try_from(v)
                .named(name!(color))?
                .check_legacy("color.alpha")?;
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
    def!(f, is_legacy(color), |s| {
        let color = s.get::<ColorOrCall>(name!(color))?;
        Ok(color.check_legacy("color.is_legacy").is_ok().into())
    });
    def!(f, ie_hex_str(color), |s| {
        let (r, g, b, alpha) =
            Color::to_rgba(&s.get(name!(color))?).to_bytes();
        Ok(format!("#{alpha:02X}{r:02X}{g:02X}{b:02X}").into())
    });
}

fn col_space_fn(space: &str, s: &ResolvedArgs) -> Result<Value, CallError> {
    let channels =
        s.get_map(name!(channels), |v| StrictChannels::from_value(space, v))?;
    match channels {
        Ok(strict) => Ok(strict.into_call(space)),
        Err(wild) => Ok(wild.into_call_or_str(space)),
    }
}

fn find_channel(
    color: ColorOrCall,
    channel: &str,
) -> Result<Value, CallError> {
    match color {
        ColorOrCall::Call(f, args) if f == "color" => {
            let (ch, alpha) = arg_channels(&args).named(name!(color))?;
            if channel == "alpha" {
                return Ok(alpha.unwrap_or(Value::scalar(1)));
            }
            match ch.split_first() {
                Some((Value::Literal(kind), args)) => {
                    let n = channel_index(kind.value(), channel)
                        .named(name!(color))?;
                    let v = args.get(n);
                    Ok(v.ok_or({
                        format!(
                            "Color (..) doesn\'t have a channel named {channel:?}.",
                        )
                    }).named(name!(channel))?.clone())
                }
                _ => Err(format!("Strange color arg {ch:?}"))
                    .named(name!(color)),
            }
        }
        ColorOrCall::Call(f, args) => {
            let (ch, alpha) = arg_channels(&args).named(name!(color))?;
            if channel == "alpha" {
                Ok(alpha.unwrap_or(Value::scalar(1)))
            } else {
                let n = channel_index(&f, channel).named(name!(color))?;
                Ok(ch
                    .get(n)
                    .cloned()
                    .ok_or_else(|| "missing".to_string())
                    .named(name!(color))?)
            }
        }
        ColorOrCall::Color(color) => {
            color
                .get_channel(channel)
                .map(Into::into)
                .map_err(|_| {
                    format!(
                        "Color {} doesn\'t have a channel named {channel:?}.",
                        color.format(Format::default())
                    )
                })
                .named(name!(channel))
            //}
        }
    }
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

fn get_space(name: &str) -> Option<(String, &[ChannelDesc])> {
    let name = if name.eq_ignore_ascii_case("xyz-d65") {
        String::from("xyz")
    } else {
        name.to_ascii_lowercase()
    };
    SPACES.get(name.as_str()).map(|c| (name, *c))
}

static SPACES: LazyLock<BTreeMap<&str, &[ChannelDesc]>> =
    LazyLock::new(|| {
        macro_rules! cd {
            ($name:ident) => {
                ChannelDesc {
                    name: stringify!($name),
                    check: Ok,
                }
            };
            ($name:ident, $check:tt) => {
                ChannelDesc {
                    name: stringify!($name),
                    check: $check,
                }
            };
        }
        let rgb = &[
            cd!(red, scalar_or_pct),
            cd!(green, scalar_or_pct),
            cd!(blue, scalar_or_pct),
        ][..];
        let xyz = &[
            cd!(x, scalar_or_pct),
            cd!(y, scalar_or_pct),
            cd!(z, scalar_or_pct),
        ][..];
        BTreeMap::from_iter([
            (
                "lab",
                &[
                    cd!(lightness, ch_bound_percent),
                    cd!(a, ch_ab),
                    cd!(b, ch_ab),
                ][..],
            ),
            (
                "oklab",
                &[
                    cd!(lightness, ch_pct_or_scalar),
                    cd!(a, ch_ok_ab),
                    cd!(b, ch_ok_ab),
                ][..],
            ),
            (
                "lch",
                &[
                    cd!(lightness, ch_bound_percent),
                    cd!(chroma, ch_chroma),
                    cd!(hue, ch_hue),
                ],
            ),
            (
                "oklch",
                &[
                    cd!(lightness, ch_pct_or_scalar),
                    cd!(chroma, ch_ok_chroma),
                    cd!(hue, ch_hue),
                ],
            ),
            ("rgb", rgb),
            ("srgb", rgb),
            ("srgb-linear", rgb),
            ("prophoto-rgb", rgb),
            ("a98-rgb", rgb),
            ("display-p3", rgb),
            ("display-p3-linear", rgb),
            ("rec2020", rgb),
            ("xyz", xyz),
            ("xyz-d50", xyz),
        ])
    });

struct ChannelDesc {
    name: &'static str,
    check: fn(Numeric) -> Result<Numeric, String>,
}

fn scalar_or_pct(v: Numeric) -> Result<Numeric, String> {
    num_or_pct_of(v, 1.).map(Numeric::scalar)
}
fn ch_pct_or_scalar(v: Numeric) -> Result<Numeric, String> {
    #[allow(clippy::manual_clamp, reason = "NaN behavior")]
    let t = num_or_pct_of(v, 1.)?.max(0.).min(1.);
    Ok(Numeric::percentage(t))
}

fn ch_bound_percent(v: Numeric) -> Result<Numeric, String> {
    #[allow(clippy::manual_clamp, reason = "NaN behavior")]
    let t = num_or_pct_of(v, 100.)?.max(0.).min(100.);
    Ok(Numeric::percentage(t / 100.))
}

fn ch_chroma(v: Numeric) -> Result<Numeric, String> {
    let t = num_or_pct_of(v, 150.)?.max(0.);
    Ok(Numeric::scalar(t))
}
fn ch_ok_chroma(v: Numeric) -> Result<Numeric, String> {
    let t = num_or_pct_of(v, 0.4)?.max(0.);
    Ok(Numeric::scalar(t))
}

fn ch_hue(v: Numeric) -> Result<Numeric, String> {
    let hue = v.as_unit_def(Unit::Deg).ok_or_else(|| {
        expected_to(v, "have an angle unit (deg, grad, rad, turn)")
    })?;
    let hue = f64::from(hue).rem_euclid(360.);
    Ok(Numeric::new(hue, crate::value::Unit::Deg))
}

/// The a and b values of a `lab` color.
fn ch_ab(v: Numeric) -> Result<Numeric, String> {
    Ok(Numeric::scalar(num_or_pct_of(v, 125.)?))
}
/// The a and b values of a `oklab` color.
fn ch_ok_ab(v: Numeric) -> Result<Numeric, String> {
    Ok(Numeric::scalar(num_or_pct_of(v, 0.4)?))
}

fn ch_channel(value: Value) -> Result<String, String> {
    let channel = CssString::try_from(value)?;
    if channel.quotes() == Quotes::None {
        Err(expected_to(channel, "be a quoted string"))
    } else {
        Ok(channel.take_value())
    }
}
fn ch_unquoted(value: Value) -> Result<String, String> {
    let channel = CssString::try_from(value)?;
    if channel.quotes() == Quotes::None {
        Ok(channel.take_value())
    } else {
        Err(expected_to(channel, "be an unquoted string"))
    }
}

fn channel_index(space: &str, channel: &str) -> Result<usize, String> {
    if let Some((_, channels)) = get_space(space) {
        channels
            .iter()
            .enumerate()
            .find(|(_, c)| c.name == channel)
            .map(|(i, _)| i)
            .ok_or_else(|| format!("Color space {space:?} doesn\'t have a channel named {channel}."))
    } else {
        Err(format!("Color space {space:?} not handled yet"))
    }
}

fn channels_from_value(
    value: &Value,
) -> Result<(Vec<Value>, Option<Value>), String> {
    match value {
        c if is_special(c) => Ok((vec![c.clone()], None)),
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
        (name!(lab), name!(lab)),
        (name!(oklab), name!(oklab)),
        (name!(lch), name!(lch)),
        (name!(oklch), name!(oklch)),
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

fn valid_col_arg_noslash(
    mut args: Vec<Value>,
) -> Result<Vec<Value>, ChannelErr> {
    let s = ch_unquoted(args.first().ok_or("empty??")?.clone())?;
    let s = if s.eq_ignore_ascii_case("from") {
        s.clone()
    } else if let Some((s, ch)) = get_space(&s) {
        check_n_channels(&args, &s, ch.len(), 1)?;
        if args.len() == ch.len() + 1 {
            let mut argiter = args.into_iter();
            args = if let Some(ch) = argiter.next() {
                vec![ch]
            } else {
                vec![]
            };
            let argiter = ch.iter().zip(argiter);
            for (ch, arg) in argiter {
                args.push(match NumOrSpecial::try_from(arg) {
                    Ok(NumOrSpecial::Num(n)) => {
                        Value::from((ch.check)(n).map_err(|s| {
                            ChannelErr::named(s, ch.name.into())
                        })?)
                    }
                    Ok(NumOrSpecial::Special(s)) => s,
                    Err(e) => {
                        return Err(format!(
                            "Expected {} channel to be a number, was {}.",
                            ch.name,
                            e.value().format(Format::default())
                        )
                        .into());
                    }
                });
            }
        }
        s
    } else {
        return Err(format!("Unknown color space {s:?}.").into());
    };
    args[0] = Value::Literal(s.into());
    Ok(args)
}

fn check_n_channels(
    args: &[Value],
    s: &str,
    ex: usize,
    extra: usize,
) -> Result<(), String> {
    let l = args.len() - extra;
    if l != ex {
        if l < ex && args.iter().any(is_special) {
            return Ok(());
        }
        let s = s.to_owned();
        let v = if args.len() == 1 {
            args.first().unwrap().introspect()
        } else {
            format!(
                "({})",
                Value::List(args.to_vec(), None, false).introspect()
            )
        };
        return Err(format!(
            "The {s} color space has {ex} channels but {v} has {l}."
        ));
    }
    Ok(())
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

fn is_from(v: &Value) -> bool {
    matches!(v, Value::Literal(s)
             if s.value().eq_ignore_ascii_case("from")
             && s.quotes() == Quotes::None
    )
}

fn is_none(v: &Value) -> bool {
    matches!(v, Value::Literal(s)
             if s.value().eq_ignore_ascii_case("none")
             && s.quotes() == Quotes::None
    )
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
