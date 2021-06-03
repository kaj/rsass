use super::{
    check, expected_to, get_checked, get_opt_check, CheckedArg, Error,
    FunctionMap,
};
use crate::css::{CallArgs, Value};
use crate::output::Format;
use crate::sass::Name;
use crate::value::{Color, Number, Quotes, Rational, Unit};
use crate::Scope;
use num_traits::{one, zero};
mod hsl;
mod hwb;
mod other;
mod rgb;

pub fn create_module() -> Scope {
    let mut f = Scope::builtin_module("sass:color");
    hsl::register(&mut f);
    hwb::register(&mut f);
    rgb::register(&mut f);
    other::register(&mut f);
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

fn check_pct(v: Value) -> Result<Number, String> {
    let val = check::numeric(v)?;
    val.as_unit_def(Unit::Percent)
        .ok_or_else(|| expected_to(&val, "have unit \"%\""))
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

/// Gets a percentage as a fraction 0 .. 1.
/// If v is not a percentage, keep it as it is.
pub fn to_rational_percent(v: Value) -> Result<Rational, String> {
    match v {
        Value::Null => Ok(zero()),
        Value::Numeric(v, ..) => {
            let r = v.value.as_ratio().map_err(|e| e.to_string())?;
            if v.unit.is_percent() || r > one() {
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

fn check_pct_rational(v: Value) -> Result<Rational, String> {
    Ok(check_pct(v)?.as_ratio().map_err(|e| e.to_string())? / 100)
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

fn get_opt_rational(
    s: &Scope,
    name: &'static str,
) -> Result<Option<Rational>, Error> {
    get_opt_check(s, Name::from_static(name), check_rational)
}

fn nospecial_value<F>(
    v: &Value,
    name: Name,
    f: F,
) -> Result<Option<Rational>, Error>
where
    F: Fn(Value) -> Result<Rational, String>,
{
    match v {
        Value::Call(..) => Ok(None),
        Value::Literal(s, Quotes::None) if looks_like_call(s) => Ok(None),
        Value::BinOp(..) => Ok(None),
        v => f(v.clone()).named(name).map(Some),
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
