use super::{
    expected_to, is_not, is_special, CallError, CheckedArg, FunctionMap,
    ResolvedArgs,
};
use crate::css::{CallArgs, CssString, Value};
use crate::input::SourcePos;
use crate::output::Format;
use crate::sass::{FormalArgs, Name};
use crate::value::{ListSeparator, Number, Numeric, Quotes, Rational, Unit};
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
            let col = s.get(name!($arg1))?;
            let arg = s.get(name!($arg2))?;
            Err(not_in_module(&name!($name), &col, &name!($toarg), &arg))
        });
    }};
    ($f:expr, $name:ident($arg1:ident, $arg2:ident), - $toarg:ident) => {{
        def!($f, $name($arg1, $arg2), |s| {
            let col = s.get(name!($arg1))?;
            let arg = s.get(name!($arg2))?;
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

/// For the alpha parameter of rgba, hsla, hwba functions
/// Special perk: Defaults to 1.0 if the value is null.
fn check_alpha(v: Value) -> Result<Rational, String> {
    Ok(match v {
        Value::Null => one(),
        v => {
            let num = Numeric::try_from(v)?;
            num.as_unit(Unit::None)
                .ok_or_else(|| {
                    expected_to(&num, "have unit \"%\" or no units")
                })?
                .as_ratio()?
        }
    })
}
/// Get a rational number in the 0..1 range.
fn check_alpha_range(v: Value) -> Result<Rational, String> {
    let v = Numeric::try_from(v)?;
    let r = v.as_ratio()?;
    if r < zero() || r > one() {
        Err(expected_to(&v, "be within 0 and 1"))
    } else {
        Ok(r)
    }
}
fn check_alpha_pm(v: Value) -> Result<Rational, String> {
    let v = Numeric::try_from(v)?;
    let r = v.as_ratio()?;
    if r.abs() > one() {
        Err(expected_to(&v, "be within -1 and 1"))
    } else {
        Ok(r)
    }
}

fn check_hue(v: Value) -> Result<Rational, String> {
    let v = Numeric::try_from(v)?;
    let v = match v.as_unit_def(Unit::Deg) {
        Some(v) => v,
        None => {
            dep_warn!(
                "$hue: Passing a unit other than deg ({}) is deprecated.\
                 \nTo preserve current behavior: calc($hue / 1{})\
                 \nSee https://sass-lang.com/d/function-units",
                v.format(Format::introspect()),
                v.unit
            );
            v.value
        }
    };
    v.as_ratio().map_err(|e| e.to_string())
}

fn check_pct(v: Value) -> Result<Number, String> {
    let val = Numeric::try_from(v)?;
    match val.as_unit_def(Unit::Percent) {
        Some(v) => Ok(v),
        None => {
            dep_warn!(
                "Passing a number without unit % ({}) is deprecated.\
                 \nTo preserve current behavior: calc($weight / 1{} * 1%)\
                 \nMore info: https://sass-lang.com/d/function-units",
                val.format(Format::introspect()),
                val.unit
            );
            Ok(val.value)
        }
    }
}

fn check_expl_pct(v: Value) -> Result<Rational, String> {
    let val = Numeric::try_from(v)?;
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
        Err(expected_to(
            &Numeric::new(val, Unit::Percent),
            "be within 0% and 100%",
        ))
    } else {
        Ok(val.as_ratio()? / 100)
    }
}

fn check_amount(v: Value) -> Result<Rational, String> {
    let val = check_pct(v)?;
    if val < zero() || val > 100.into() {
        Err(expected_to(&val, "be within 0 and 100"))
    } else {
        Ok(val.as_ratio()? / 100)
    }
}

fn check_channel(v: Value) -> Result<Rational, String> {
    num2chan(&Numeric::try_from(v)?)
}
fn check_channel_range(v: Value) -> Result<Rational, String> {
    let v = Numeric::try_from(v)?;
    let r = num2chan(&v)?;
    if r > Rational::from_integer(255) || r < zero() {
        Err(expected_to(&v, "be within 0 and 255"))
    } else {
        Ok(r)
    }
}
fn check_channel_pm(v: Value) -> Result<Rational, String> {
    let v = Numeric::try_from(v)?;
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

fn make_call(name: &str, args: Vec<Value>) -> Value {
    Value::Call(
        name.into(),
        CallArgs::from_list(
            args.into_iter().filter(|v| v != &Value::Null).collect(),
        ),
    )
}

pub(crate) fn eval_inner(
    name: &Name,
    decl: &FormalArgs,
    outer: &ResolvedArgs,
    args: CallArgs,
) -> Result<ResolvedArgs, CallError> {
    Ok(ResolvedArgs::new(
        decl.eval(outer.raw(), args).map_err(|e| {
            e.declared_at(&SourcePos::mock_function(name, decl, ""))
        })?,
        outer.call_scope(),
    ))
}

fn not_in_module(nm: &Name, col: &Value, an: &Name, av: &Value) -> CallError {
    CallError::msg(format!(
        "The function {0}() isn\'t in the sass:color module.\n\
         \nRecommendation: color.adjust({1}, ${2}: {3})\n\
         \nMore info: https://sass-lang.com/documentation/functions/color#{0}",
        nm,
        col.format(Format::introspect()),
        an,
        av.format(Format::introspect()),
    ))
}

fn relative_color(args: &CallArgs) -> bool {
    fn is_from(s: &CssString) -> bool {
        s.quotes() == Quotes::None && s.value().eq_ignore_ascii_case("from")
    }
    fn inner(arg: &Value) -> bool {
        match arg {
            Value::List(l, Some(ListSeparator::Space), false) => {
                matches!(l.get(0), Some(Value::Literal(s)) if is_from(s))
            }
            Value::List(l, Some(ListSeparator::Slash), false) => {
                l.len() == 2 && inner(&l[0])
            }
            _ => false,
        }
    }
    args.get_single().map(inner).unwrap_or(false)
}
