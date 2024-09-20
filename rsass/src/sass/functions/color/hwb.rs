use super::super::FunctionMap;
use super::hsl::percentage;
use super::{
    check_alpha, check_expl_pct_norange, eval_inner, is_not, relative_color,
    CallError, CheckedArg, ResolvedArgs,
};
use crate::css::{CallArgs, Value};
use crate::output::Format;
use crate::sass::FormalArgs;
use crate::value::{Color, Hwba, ListSeparator, Numeric, Rgba, Unit};
use crate::Scope;

pub fn register(f: &mut Scope) {
    def_va!(f, hwb(kwargs), hwb);
    def!(f, blackness(color), |s| {
        let color: Color = s.get(name!(color))?;
        let hwb = color.to_hwba();
        Ok(percentage(hwb.blackness()))
    });
    def!(f, whiteness(color), |s| {
        let color: Color = s.get(name!(color))?;
        let hwb = color.to_hwba();
        Ok(percentage(hwb.whiteness()))
    });
}

pub fn expose(m: &Scope, global: &mut FunctionMap) {
    global.insert(name!(hwb), m.get_lfunction(&name!(hwb)));
}

fn hwb(s: &ResolvedArgs) -> Result<Value, CallError> {
    let args = s.get_map(name!(kwargs), CallArgs::from_value)?;
    if relative_color(&args) {
        return Ok(Value::Call("hwb".to_string(), args));
    }
    let (hue, w, b, a) = if args.len() < 3 {
        let a1 = FormalArgs::new(vec![one_arg!(channels)]);
        eval_inner(&name!(hwb), &a1, s, args)
            .and_then(|s| hwb_from_channels(s.get(name!(channels))?))?
    } else {
        let a2 = FormalArgs::new(vec![
            one_arg!(hue),
            one_arg!(whiteness),
            one_arg!(blackness),
            one_arg!(alpha = b"1"),
        ]);
        eval_inner(&name!(hwb), &a2, s, args).and_then(|s| {
            Ok((
                s.get(name!(hue))?,
                s.get(name!(whiteness))?,
                s.get(name!(blackness))?,
                s.get(name!(alpha))?,
            ))
        })?
    };
    let hue = check_hue(hue).named(name!(hue))?;
    let w = check_expl_pct_norange(w).named(name!(whiteness))?;
    let b = check_expl_pct_norange(b).named(name!(blackness))?;
    let a = check_alpha(a).named(name!(alpha))?;
    // I don't really agree with this, but it makes tests pass.
    let hue = if w + b >= 1. { 0. } else { hue };
    let hwba = Hwba::new(hue, w, b, a);
    let rgba = Rgba::from(&hwba);
    if rgba.is_integer() && w >= 0. {
        Ok(rgba.into())
    } else {
        Ok(hwba.into())
    }
}

fn hwb_from_channels(
    v: Value,
) -> Result<(Value, Value, Value, Value), CallError> {
    match v {
        Value::List(_, _, true) => {
            Err(CallError::msg("$channels must be an unbracketed list."))
        }
        Value::List(_, Some(ListSeparator::Comma), _) => {
            Err(CallError::msg("$channels must be a space-separated list."))
        }
        Value::List(vec, s, p) => {
            use crate::value::Operator::Div;
            match vec.len() {
                0 => Err(CallError::msg("Missing element $hue.")),
                1 => Err(CallError::msg("Missing element $whiteness.")),
                2 => Err(CallError::msg("Missing element $blackness.")),
                3 => {
                    if let Value::BinOp(op) = &vec[2] {
                        if let (
                            Div,
                            a @ Value::Numeric(..),
                            b @ Value::Numeric(..),
                        ) = (op.op(), op.a(), op.b())
                        {
                            Ok((
                                vec[0].clone(),
                                vec[1].clone(),
                                a.clone(),
                                b.clone(),
                            ))
                        } else {
                            Err(badchannels(&Value::List(vec, s, p)))
                        }
                    } else {
                        Ok((
                            vec[0].clone(),
                            vec[1].clone(),
                            vec[2].clone(),
                            Value::Null,
                        ))
                    }
                }
                n => Err(CallError::msg(format!(
                    "Only 3 elements allowed, but {n} were passed.",
                ))),
            }
        }
        _hue => Err(CallError::msg("Missing element $whiteness.")),
    }
}

fn badchannels(v: &Value) -> CallError {
    CallError::msg(format!(
        "Expected numeric channels, got \"hwb({})\".",
        v.format(Format::introspect()),
    ))
}

fn check_hue(v: Value) -> Result<f64, String> {
    let vv = Numeric::try_from(v)?;
    if let Some(scaled) = vv.as_unit_def(Unit::Deg) {
        Ok(scaled.into())
    } else {
        Err(is_not(&vv, "an angle"))
    }
}
