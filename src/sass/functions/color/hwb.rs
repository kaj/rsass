use super::hsl::percentage;
use super::{
    bad_arg, check, check_alpha, check_expl_pct, get_color, CheckedArg,
};
use crate::css::{CallArgs, Value};
use crate::output::Format;
use crate::sass::FormalArgs;
use crate::value::{Hwba, ListSeparator, Number, Unit};
use crate::{Error, Scope, ScopeRef};

pub fn register(f: &mut Scope) {
    def_va!(f, hwb(kwargs), hwb);
    def!(f, blackness(color), |s| {
        Ok(percentage(get_color(s, "color")?.to_hwba().blackness()))
    });
    def!(f, whiteness(color), |s| {
        Ok(percentage(get_color(s, "color")?.to_hwba().whiteness()))
    });
}

fn hwb(s: &ScopeRef) -> Result<Value, Error> {
    let args = CallArgs::from_value(s.get("kwargs")?)?;
    let (hue, w, b, a) = if args.len() < 3 {
        let a1 = FormalArgs::new(vec![one_arg!(channels)]);
        a1.eval(s.clone(), args)
            .map_err(|e| bad_arg(e, &name!(hwb), &a1))
            .and_then(|s| hwb_from_channels(s.get("channels")?))?
    } else {
        let a2 = FormalArgs::new(vec![
            one_arg!(hue),
            one_arg!(whiteness),
            one_arg!(blackness),
            one_arg!(alpha = b"1"),
        ]);
        a2.eval(s.clone(), args)
            .map_err(|e| bad_arg(e, &name!(hwb), &a2))
            .and_then(|s| {
                Ok((
                    s.get("hue")?,
                    s.get("whiteness")?,
                    s.get("blackness")?,
                    s.get("alpha")?,
                ))
            })?
    };
    let hue = as_hue(hue).named(name!(hue))?.as_ratio()?;
    let w = check_expl_pct(w).named(name!(whiteness))?.as_ratio()?;
    let b = check_expl_pct(b).named(name!(blackness))?.as_ratio()?;
    let a = check_alpha(a).named(name!(alpha))?;
    Ok(Hwba::new(hue, w / 100, b / 100, a).into())
}

fn hwb_from_channels(
    v: Value,
) -> Result<(Value, Value, Value, Value), Error> {
    match v {
        Value::List(_, _, true) => {
            Err(Error::error("$channels must be an unbracketed list"))
        }
        Value::List(_, Some(ListSeparator::Comma), _) => {
            Err(Error::error("$channels must be a space-separated list"))
        }
        Value::List(vec, s, p) => {
            use crate::value::Operator::Div;
            match vec.len() {
                0 => Err(Error::error("Missing element $hue")),
                1 => Err(Error::error("Missing element $whiteness")),
                2 => Err(Error::error("Missing element $blackness")),
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
                n => Err(Error::error(format!(
                    "Only 3 elements allowed, but {} were passed",
                    n
                ))),
            }
        }
        _hue => Err(Error::error("Missing element $whiteness")),
    }
}

fn badchannels(v: &Value) -> Error {
    Error::error(format!(
        "Expected numeric channels, got \"hwb({})\"",
        v.format(Format::introspect()),
    ))
}

fn as_hue(v: Value) -> Result<Number, String> {
    let vv = check::numeric(v)?;
    if let Some(scaled) = vv.as_unit_def(Unit::Deg) {
        Ok(scaled)
    } else {
        Err(format!(
            "{} is not an angle",
            vv.format(Format::introspect()),
        ))
    }
}
