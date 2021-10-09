use super::{
    check, expected_to, get_checked, get_opt_check, is_not, CheckedArg,
    Error, FunctionMap,
};
use crate::css::{CallArgs, CssString, Value};
use crate::output::Format;
use crate::parser::SourcePos;
use crate::sass::{ArgsError, FormalArgs, Name};
use crate::value::{Color, Number, Numeric, Rational, Unit};
use crate::Scope;
use num_traits::{one, zero, Signed};
mod channels;
mod hsl;
mod hwb;
mod other;
mod rgb;

macro_rules! def_adj {
    ($f:expr, $name:ident($arg1:ident, $arg2:ident), $toarg:ident) => {{
        def!($f, $name($arg1, $arg2), |s| {
            let col = s.get(stringify!($arg1))?;
            let arg = s.get(stringify!($arg2))?;
            Err(not_in_module(&name!($name), &col, &name!($toarg), &arg))
        });
    }};
    ($f:expr, $name:ident($arg1:ident, $arg2:ident), - $toarg:ident) => {{
        def!($f, $name($arg1, $arg2), |s| {
            let col = s.get(stringify!($arg1))?;
            let arg = s.get(stringify!($arg2))?;
            let arg =
                Value::UnaryOp(crate::value::Operator::Minus, Box::new(arg));
            Err(not_in_module(&name!($name), &col, &name!($toarg), &arg))
        });
    }};
}

pub fn create_module() -> Scope {
    let mut f = Scope::builtin_module("sass:color");
    hsl::register(&mut f);
    hwb::register(&mut f);
    rgb::register(&mut f);
    other::register(&mut f);
    def_adj!(f, adjust_hue(color, degrees), hue);
    def_adj!(f, lighten(color, amount), lightness);
    def_adj!(f, darken(color, amount), -lightness);
    def_adj!(f, desaturate(color, amount), -saturation);
    def_adj!(f, saturate(color, amount), saturation);
    def_adj!(f, fade_in(color, amount), alpha);
    def_adj!(f, opacify(color, amount), alpha);
    def_adj!(f, fade_out(color, amount), -alpha);
    def_adj!(f, transparentize(color, amount), -alpha);
    f
}

pub fn expose(m: &Scope, global: &mut FunctionMap) {
    rgb::expose(m, global);
    hsl::expose(m, global);
    other::expose(m, global);
}

fn get_color(s: &Scope, name: &'static str) -> Result<Color, Error> {
    get_checked(s, Name::from_static(name), check_color)
}
fn check_color(v: Value) -> Result<Color, String> {
    match v {
        Value::Color(col, _) => Ok(col),
        v => Err(is_not(&v, "a color")),
    }
}

/// For the alpha parameter of rgba, hsla, hwba functions
/// Special perk: Defaults to 1.0 if the value is null.
fn check_alpha(v: Value) -> Result<Rational, String> {
    Ok(match v {
        Value::Null => one(),
        v => {
            let num = check::numeric(v)?;
            num.as_unit(Unit::None)
                .ok_or_else(|| expected_to(&num, "have no units or \"%\""))?
                .as_ratio()?
        }
    })
}
/// Get a rational number in the 0..1 range.
fn check_alpha_range(v: Value) -> Result<Rational, String> {
    let val = check::numeric(v)?;
    let a = val
        .as_unit_def(Unit::None)
        .ok_or_else(|| expected_to(&val, "have no units or \"%\""))?
        .as_ratio()?;
    if a < zero() || a > one() {
        Err(expected_to(&val, "be within 0 and 1"))
    } else {
        Ok(a)
    }
}
fn check_alpha_pm(v: Value) -> Result<Rational, String> {
    let v = check_rational(v)?;
    if v.abs() > one() {
        Err(expected_to(&Number::from(v), "be within -1 and 1"))
    } else {
        Ok(v)
    }
}

fn check_pct(v: Value) -> Result<Number, String> {
    let val = check::numeric(v)?;
    val.as_unit_def(Unit::Percent)
        .ok_or_else(|| expected_to(&val, "have unit \"%\""))
}

fn check_expl_pct(v: Value) -> Result<Rational, String> {
    let val = check::numeric(v)?;
    if !val.unit.is_percent() {
        return Err(expected_to(&val, "have unit \"%\""));
    }
    if val.value < zero() || val.value > 100.into() {
        Err(expected_to(&val, "be within 0% and 100%"))
    } else {
        Ok(val.as_ratio()? / 100)
    }
}

fn check_pct_range(v: Value) -> Result<Rational, String> {
    let val = check_pct(v)?;
    if val < zero() || val > 100.into() {
        Err(expected_to(&val, "be within 0 and 100"))
    } else {
        Ok(val.as_ratio()? / 100)
    }
}
fn check_rational(v: Value) -> Result<Rational, String> {
    Ok(check::numeric(v)?.as_ratio()?)
}

fn check_channel(v: Value) -> Result<Rational, String> {
    num2chan(&check::numeric(v)?)
}
fn check_channel_range(v: Value) -> Result<Rational, String> {
    let v = check::numeric(v)?;
    let r = num2chan(&v)?;
    if r > Rational::from_integer(255) || r < zero() {
        Err(expected_to(&v, "be within 0 and 255"))
    } else {
        Ok(r)
    }
}
fn check_channel_pm(v: Value) -> Result<Rational, String> {
    let v = check::numeric(v)?;
    let r = num2chan(&v)?;
    if r.abs() > Rational::from_integer(255) {
        Err(expected_to(&v, "be within -255 and 255"))
    } else {
        Ok(r)
    }
}
fn num2chan(v: &Numeric) -> Result<Rational, String> {
    let r = v.as_ratio()?;
    if v.unit.is_percent() {
        Ok(r * 255 / 100)
    } else {
        Ok(r)
    }
}

fn is_special(v: &Value) -> bool {
    match v {
        Value::Call(..) => true,
        Value::Literal(s) if looks_like_call(s) => true,
        Value::BinOp(..) => true,
        _ => false,
    }
}

fn looks_like_call(s: &CssString) -> bool {
    s.quotes().is_none()
        && s.value().contains('(')
        && s.value().ends_with(')')
}

fn make_call(name: &str, args: Vec<Value>) -> Value {
    Value::Call(
        name.into(),
        CallArgs::from_list(
            args.into_iter().filter(|v| v != &Value::Null).collect(),
        ),
    )
}

fn bad_arg(err: ArgsError, name: &Name, args: &FormalArgs) -> Error {
    match err {
        ArgsError::Eval(e) => e,
        ae => Error::BadArguments(
            ae.to_string(),
            SourcePos::mock_function(name, args, ""),
        ),
    }
}

fn not_in_module(nm: &Name, col: &Value, an: &Name, av: &Value) -> Error {
    Error::S(format!(
        "Error: The function {0}() isn\'t in the sass:color module.\n\
         \nRecommendation: color.adjust({1}, ${2}: {3})\n\
         \nMore info: https://sass-lang.com/documentation/functions/color#{0}",
        nm,
        col.format(Format::introspect()),
        an,
        av.format(Format::introspect()),
    ))
}
