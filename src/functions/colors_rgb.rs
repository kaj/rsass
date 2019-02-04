use super::{Error, SassFunction};
use crate::css::{CallArgs, Value};
use crate::value::{ListSeparator, Number, Unit};
use num_rational::Rational;
use num_traits::One;
use std::collections::BTreeMap;

pub fn register(f: &mut BTreeMap<&'static str, SassFunction>) {
    def!(f, rgb(red, green, blue), |s| Ok(Value::rgba(
        to_int(s.get("red")?)?,
        to_int(s.get("green")?)?,
        to_int(s.get("blue")?)?,
        Rational::one()
    )));
    def!(f, rgba(red, green, blue, alpha, color), |s| {
        let a = s.get("alpha")?;
        let red = s.get("red")?;
        let red = if red.is_null() { s.get("color")? } else { red };
        if let Value::Color(rgba, _) = red {
            let a = if a.is_null() { s.get("green")? } else { a };
            match a {
                Value::Numeric(a, ..) => {
                    Ok(Value::rgba(rgba.red, rgba.green, rgba.blue, a.value))
                }
                _ => Ok(Value::Call(
                    "rgba".into(),
                    CallArgs::from_value(Value::List(
                        vec![
                            int_value(rgba.red),
                            int_value(rgba.green),
                            int_value(rgba.blue),
                            a,
                        ],
                        ListSeparator::Space,
                        false,
                    )),
                )),
            }
        } else {
            Ok(Value::rgba(
                to_int(red)?,
                to_int(s.get("green")?)?,
                to_int(s.get("blue")?)?,
                to_rational(a)?,
            ))
        }
    });
    fn num(v: &Rational) -> Result<Value, Error> {
        Ok(Value::Numeric(Number::from(*v), Unit::None, true))
    }
    def!(f, red(color), |s| match &s.get("color")? {
        &Value::Color(ref rgba, _) => num(&rgba.red),
        value => Err(Error::badarg("color", value)),
    });
    def!(f, green(color), |s| match &s.get("color")? {
        &Value::Color(ref rgba, _) => num(&rgba.green),
        value => Err(Error::badarg("color", value)),
    });
    def!(f, blue(color), |s| match &s.get("color")? {
        &Value::Color(ref rgba, _) => num(&rgba.blue),
        value => Err(Error::badarg("color", value)),
    });
    def!(f, mix(color1, color2, weight = b"50%"), |s| match (
        s.get("color1")?,
        s.get("color2")?,
        s.get("weight")?,
    ) {
        (
            Value::Color(a, _),
            Value::Color(b, _),
            Value::Numeric(w, wu, ..),
        ) => {
            let w = if wu == Unit::Percent {
                w.value / 100
            } else {
                w.value
            };
            let one = Rational::one();
            let w2 = one - (one - w * a.alpha) * b.alpha;
            let m_c = |a, b| a * w2 + b * (one - w2);;
            let m_a = |a, b| a * w + b * (one - w);
            Ok(Value::rgba(
                m_c(a.red, b.red),
                m_c(a.green, b.green),
                m_c(a.blue, b.blue),
                m_a(a.alpha, b.alpha),
            ))
        }
        (color1, color2, weight) => Err(Error::badargs(
            &["color", "color", "number"],
            &[&color1, &color2, &weight],
        )),
    });
    def!(f, invert(color, weight = b"100%"), |s| match (
        s.get("color")?,
        s.get("weight")?,
    ) {
        (Value::Color(rgba, _), Value::Numeric(w, wu, ..)) => {
            let w = if wu == Unit::Percent {
                w.value / 100
            } else {
                w.value
            };
            let inv = |v: Rational| -(v - 255) * w + v * -(w - 1);
            Ok(Value::rgba(
                inv(rgba.red),
                inv(rgba.green),
                inv(rgba.blue),
                rgba.alpha,
            ))
        }
        (value, weight) => {
            Err(Error::badargs(&["color", "percentage"], &[&value, &weight]))
        }
    });
}

fn int_value(v: Rational) -> Value {
    Value::scalar(v.to_integer())
}

fn to_int(v: Value) -> Result<Rational, Error> {
    match v {
        Value::Numeric(v, Unit::Percent, _) => Ok(v.value * 255 / 100),
        Value::Numeric(v, ..) => Ok(v.value),
        v => Err(Error::badarg("number", &v)),
    }
}

fn to_rational(v: Value) -> Result<Rational, Error> {
    match v {
        Value::Numeric(num, ..) => Ok(num.value),
        v => Err(Error::badarg("number", &v)),
    }
}

#[cfg(test)]
mod test {
    use crate::variablescope::test::do_evaluate;

    #[test]
    fn test_high() {
        assert_eq!("white", do_evaluate(&[], b"rgb(150%, 300, 256);"));
    }

    #[test]
    fn test_low() {
        assert_eq!("black", do_evaluate(&[], b"rgb(-3, -2%, 0);"));
    }
    #[test]
    fn test_mid() {
        assert_eq!("gray", do_evaluate(&[], b"rgb(50%, 255/2, 25% + 25);"));
    }
    #[test]
    fn test_named() {
        assert_eq!("gray", do_evaluate(&[], b"rgb(50%, 255/2, 25% + 25);"));
    }
}
