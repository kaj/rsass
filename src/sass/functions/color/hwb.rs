use super::hsl::{percentage, to_rational2, to_rational_percent};
use super::rgb::values_from_list;
use super::{get_color, Error};
use crate::css::Value;
use crate::output::Format;
use crate::value::{Hwba, Number, Unit};
use crate::Scope;
use num_rational::Rational;
use num_traits::One;

pub fn register(f: &mut Scope) {
    def!(f, hwb(hue, whiteness, blackness, alpha, channels), |s| {
        let (hue, w, b, a) = match s.get("hue")? {
            Value::List(vec, s, p) => values_from_list(&vec)
                .ok_or_else(|| badchannels(&Value::List(vec, s, p)))?,
            Value::Null => match s.get("channels")? {
                Value::List(vec, s, p) => values_from_list(&vec)
                    .ok_or_else(|| badchannels(&Value::List(vec, s, p)))?,
                v => return Err(badchannels(&v)),
            },
            hue => (
                hue,
                s.get("whiteness")?,
                s.get("blackness")?,
                s.get("alpha")?,
            ),
        };
        let hue = as_hue(&hue)?;
        let w = to_rational_percent(&w, name!(whiteness))?;
        let b = to_rational_percent(&b, name!(blackness))?;
        let a = if a.is_null() {
            Rational::one()
        } else {
            to_rational2(&a, name!(alpha))?
        };
        Ok(Hwba::new(hue.as_ratio()?, w, b, a).into())
    });
    def!(f, blackness(color), |s| {
        Ok(percentage(get_color(s, "color")?.to_hwba().blackness()))
    });
    def!(f, whiteness(color), |s| {
        Ok(percentage(get_color(s, "color")?.to_hwba().whiteness()))
    });
}

fn badchannels(v: &Value) -> Error {
    Error::S(format!(
        "Error: Expected numeric channels, got \"hwb({})\".",
        v.format(Format::introspect()),
    ))
}

fn as_hue(v: &Value) -> Result<Number, Error> {
    match v {
        Value::Numeric(vv, ..) => {
            if let Some(scaled) = vv.as_unit_def(Unit::Deg) {
                Ok(scaled)
            } else {
                Err(Error::bad_arg(name!(hue), &v, "is not an angle"))
            }
        }
        v => Err(Error::bad_arg(name!(hue), &v, "is not a number")),
    }
}
