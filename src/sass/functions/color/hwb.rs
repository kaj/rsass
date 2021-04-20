use super::hsl::percentage;
use super::rgb::to_rational;
use super::{check, check_expl_pct, get_color, CheckedArg, Error};
use crate::css::Value;
use crate::output::Format;
use crate::value::{Hwba, ListSeparator, Number, Unit};
use crate::Scope;
use num_traits::one;

pub fn register(f: &mut Scope) {
    def!(
        f,
        hwb(
            hue = b"null",
            whiteness = b"null",
            blackness = b"null",
            alpha = b"null",
            channels = b"null"
        ),
        |s| {
            let (hue, w, b, a) = match s.get("hue")? {
                list @ Value::List(..) => hwb_from_channels(list)?,
                Value::Null => hwb_from_channels(s.get("channels")?)?,
                hue => (
                    hue,
                    s.get("whiteness")?,
                    s.get("blackness")?,
                    s.get("alpha")?,
                ),
            };
            let hue = as_hue(hue).named(name!(hue))?;
            let w = match w {
                Value::Null => {
                    return Err(Error::error("Missing element $whiteness"))
                }
                w => check_expl_pct(w).named(name!(whiteness))?,
            };
            let b = check_expl_pct(b).named(name!(blackness))?;
            let a = match a {
                Value::Null => one(),
                a => to_rational(a).named(name!(alpha))?,
            };
            Ok(Hwba::new(
                hue.as_ratio()?,
                w.as_ratio()? / 100,
                b.as_ratio()? / 100,
                a,
            )
            .into())
        }
    );
    def!(f, blackness(color), |s| {
        Ok(percentage(get_color(s, "color")?.to_hwba().blackness()))
    });
    def!(f, whiteness(color), |s| {
        Ok(percentage(get_color(s, "color")?.to_hwba().whiteness()))
    });
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
        v => Err(badchannels(&v)),
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
