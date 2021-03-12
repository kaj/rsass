use super::hsl::{percentage, to_rational2};
use super::rgb::values_from_list;
use super::{check, check_pct_rational_range, get_color, CheckedArg, Error};
use crate::css::Value;
use crate::output::Format;
use crate::value::{Hwba, Number, Unit};
use crate::Scope;
use num_rational::Rational;
use num_traits::One;

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
                Value::List(vec, s, p) => values_from_list(&vec)
                    .ok_or_else(|| badchannels(&Value::List(vec, s, p)))?,
                Value::Null => match s.get("channels")? {
                    Value::List(vec, s, p) => values_from_list(&vec)
                        .ok_or_else(|| {
                            badchannels(&Value::List(vec, s, p))
                        })?,
                    v => return Err(badchannels(&v)),
                },
                hue => (
                    hue,
                    s.get("whiteness")?,
                    s.get("blackness")?,
                    s.get("alpha")?,
                ),
            };
            let hue = as_hue(hue).named(name!(hue))?;
            let w = check_pct_rational_range(w).named(name!(whiteness))?;
            let b = check_pct_rational_range(b).named(name!(blackness))?;
            let a = match a {
                Value::Null => Rational::one(),
                a => to_rational2(a).named(name!(alpha))?,
            };
            Ok(Hwba::new(hue.as_ratio()?, w, b, a).into())
        }
    );
    def!(f, blackness(color), |s| {
        Ok(percentage(get_color(s, "color")?.to_hwba().blackness()))
    });
    def!(f, whiteness(color), |s| {
        Ok(percentage(get_color(s, "color")?.to_hwba().whiteness()))
    });
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
