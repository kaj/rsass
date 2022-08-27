use super::hsl::percentage;
use super::{
    check_alpha, check_expl_pct, eval_inner, is_not, CallError, CheckedArg,
    ResolvedArgs,
};
use crate::css::{CallArgs, Value};
use crate::output::Format;
use crate::sass::FormalArgs;
use crate::value::{Color, Hwba, ListSeparator, Numeric, Rational, Unit};
use crate::Scope;

pub fn register(f: &mut Scope) {
    def_va!(f, hwb(kwargs), hwb);
    def!(f, blackness(color), |s| {
        // Blackness of the rgb approximation that can be represented in css.
        let (r, g, b, _a) = Color::to_rgba(&s.get(name!(color))?).to_bytes();
        let max_c = *[r, g, b].iter().max().unwrap();
        Ok(percentage(Rational::new((255 - max_c).into(), 255)))
    });
    def!(f, whiteness(color), |s| {
        // Whiteness of the rgb approximation that can be represented in css.
        let (r, g, b, _a) = Color::to_rgba(&s.get(name!(color))?).to_bytes();
        let min_c = *[r, g, b].iter().min().unwrap();
        Ok(percentage(Rational::new(min_c.into(), 255)))
    });
}

fn hwb(s: &ResolvedArgs) -> Result<Value, CallError> {
    let args = s.get_map(name!(kwargs), CallArgs::from_value)?;
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
    let w = check_expl_pct(w).named(name!(whiteness))?;
    let b = check_expl_pct(b).named(name!(blackness))?;
    let a = check_alpha(a).named(name!(alpha))?;
    Ok(Hwba::new(hue, w, b, a).into())
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
                    if let Value::BinOp(a, _, Div, _, b) = &vec[2] {
                        if let (Value::Numeric(..), Value::Numeric(..)) =
                            (&**a, &**b)
                        {
                            Ok((
                                vec[0].clone(),
                                vec[1].clone(),
                                *a.clone(),
                                *b.clone(),
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
                    "Only 3 elements allowed, but {} were passed.",
                    n
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

fn check_hue(v: Value) -> Result<Rational, String> {
    let vv = Numeric::try_from(v)?;
    if let Some(scaled) = vv.as_unit_def(Unit::Deg) {
        Ok(scaled.as_ratio()?)
    } else {
        Err(is_not(&vv, "an angle"))
    }
}
