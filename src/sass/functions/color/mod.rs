use super::{
    check, expected_to, get_checked, get_opt_check, CheckedArg, Error,
    FunctionMap,
};
use crate::css::{CallArgs, Value};
use crate::output::Format;
use crate::parser::SourcePos;
use crate::sass::{ArgsError, FormalArgs, Name};
use crate::value::{Color, Number, Quotes, Rational, Unit};
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
        v => {
            Err(format!("{} is not a color", v.format(Format::introspect())))
        }
    }
}

/// For the alpha parameter of rgba, hsla, hwba functions
/// Special perk: Defaults to 1.0 if the value is null.
fn check_alpha(v: Value) -> Result<Rational, String> {
    match v {
        Value::Null => Ok(one()),
        v => {
            let num = check::numeric(v)?;
            let r = num.value.as_ratio().map_err(|e| e.to_string())?;
            if num.unit.is_percent() {
                Ok(r / 100)
            } else if num.unit.is_none() {
                Ok(r)
            } else {
                Err(expected_to(&num, "have no units or \"%\""))
            }
        }
    }
}

fn check_pct(v: Value) -> Result<Number, String> {
    let val = check::numeric(v)?;
    val.as_unit_def(Unit::Percent)
        .ok_or_else(|| expected_to(&val, "have unit \"%\""))
}

fn check_pct_expl_rational_pm1(v: Value) -> Result<Rational, String> {
    let val = check::numeric(v)?;
    if !val.unit.is_percent() {
        return Err(expected_to(&val, "have unit \"%\""));
    }
    if val.value.clone().abs() > 100.into() {
        Err(expected_to(&val, "be within -100% and 100%"))
    } else {
        let r = val.value.as_ratio().map_err(|e| e.to_string())?;
        Ok(r / 100)
    }
}

fn check_expl_pct(v: Value) -> Result<Number, String> {
    let val = check::numeric(v)?;
    if !val.unit.is_percent() {
        return Err(expected_to(&val, "have unit \"%\""));
    }
    if val.value < 0.into() || val.value > 100.into() {
        Err(expected_to(&val, "be within 0% and 100%"))
    } else {
        Ok(val.value)
    }
}
fn check_expl_pct_r(v: Value) -> Result<Rational, String> {
    check_expl_pct(v)?
        .as_ratio()
        .map(|r| r / 100)
        .map_err(|e| e.to_string())
}

fn check_pct_rational_range(v: Value) -> Result<Rational, String> {
    let val = check_pct(v)?;
    if val < 0.into() || val > 100.into() {
        Err(expected_to(&val, "be within 0 and 100"))
    } else {
        val.as_ratio().map_err(|e| e.to_string()).map(|v| v / 100)
    }
}
fn check_rational(v: Value) -> Result<Rational, String> {
    check::numeric(v)?.as_ratio().map_err(|e| e.to_string())
}
fn check_rational_pm1(v: Value) -> Result<Rational, String> {
    let v = check_rational(v)?;
    if v.abs() > one() {
        Err(expected_to(&Number::from(v), "be within -1 and 1"))
    } else {
        Ok(v)
    }
}
fn check_rational_byte(v: Value) -> Result<Rational, String> {
    let v = check_rational(v)?;
    if v > Rational::from_integer(255) || v < zero() {
        Err(expected_to(&Number::from(v), "be within 0 and 255"))
    } else {
        Ok(v)
    }
}
fn check_rational_pmbyte(v: Value) -> Result<Rational, String> {
    let v = check_rational(v)?;
    if v > Rational::from_integer(255) || v < Rational::from_integer(-255) {
        Err(expected_to(&Number::from(v), "be within -255 and 255"))
    } else {
        Ok(v)
    }
}
/// Get a rational number in the 0..1 range.
fn check_rational_fract(v: Value) -> Result<Rational, String> {
    let val = check::numeric(v)?
        .as_unit_def(Unit::None)
        .ok_or_else(|| "xyzzy".to_string())?;
    if val < 0.into() || val > 1.into() {
        Err(expected_to(&val, "be within 0 and 1"))
    } else {
        val.as_ratio().map_err(|e| e.to_string())
    }
}

fn is_special(v: &Value) -> bool {
    match v {
        Value::Call(..) => true,
        Value::Literal(s, Quotes::None) if looks_like_call(s) => true,
        Value::BinOp(..) => true,
        _ => false,
    }
}

fn looks_like_call(s: &str) -> bool {
    s.contains('(') && s.ends_with(')')
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
