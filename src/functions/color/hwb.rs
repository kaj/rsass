use super::hsl::{percentage, to_rational2, to_rational_percent};
use super::rgb::values_from_list;
use super::{Error, Module, SassFunction};
use crate::css::Value;
use crate::value::{Hwba, NumValue, Unit};
use num_rational::Rational;
use num_traits::One;

pub fn register(f: &mut Module) {
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
    def!(f, blackness(color), |args| match args.get("color")? {
        Value::Color(col, _) => Ok(percentage(col.to_hwba().blackness())),
        v => Err(Error::badarg("color", &v)),
    });
    def!(f, whiteness(color), |args| match args.get("color")? {
        Value::Color(col, _) => Ok(percentage(col.to_hwba().whiteness())),
        v => Err(Error::badarg("color", &v)),
    });
}

fn badchannels(v: &Value) -> Error {
    Error::badarg("Expected channels list", v)
}

fn as_deg(v: &Value) -> Result<NumValue, Error> {
    match v {
        Value::Numeric(vv, u, ..) => {
            if u == &Unit::None {
                Ok(vv.value.clone())
            } else if let Some(scale) = u.scale_to(&Unit::Deg) {
                Ok(&vv.value * scale)
            } else {
                Err(Error::badarg("angle", &v))
            }
        }
        v => Err(Error::badarg("angle", &v)),
    }
}
