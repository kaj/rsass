use super::hsl::{percentage, to_rational2, to_rational_percent};
use super::rgb::values_from_list;
use super::{get_color, Error};
use crate::css::Value;
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
        let hue = as_deg(&hue)?;
        let w = to_rational_percent(&w)?;
        let b = to_rational_percent(&b)?;
        let a = if a.is_null() {
            Rational::one()
        } else {
            to_rational2(&a)?
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
    Error::badarg("Expected channels list", v)
}

fn as_deg(v: &Value) -> Result<Number, Error> {
    match v {
        Value::Numeric(vv, ..) => {
            if let Some(scaled) = vv.as_unit_def(Unit::Deg) {
                Ok(scaled)
            } else {
                Err(Error::badarg("angle", &v))
            }
        }
        v => Err(Error::badarg("angle", &v)),
    }
}
