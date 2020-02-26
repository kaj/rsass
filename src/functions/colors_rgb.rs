use super::{make_call, Error, SassFunction};
use crate::css::{CallArgs, Value};
use crate::value::{Number, Quotes, Unit};
use crate::variablescope::Scope;
use num_rational::Rational;
use num_traits::{One, Zero};
use std::collections::BTreeMap;

fn do_rgba(fn_name: &str, s: &dyn Scope) -> Result<Value, Error> {
    let a = s.get("alpha")?;
    let red = s.get("red")?;
    let red = if red.is_null() { s.get("color")? } else { red };
    if let Value::Color(rgba, _) = red {
        let a = if a.is_null() { s.get("green")? } else { a };
        match a {
            Value::Numeric(a, ..) => {
                Ok(Value::rgba(rgba.red, rgba.green, rgba.blue, a.value))
            }
            _ => Ok(make_call(
                fn_name,
                vec![
                    int_value(rgba.red),
                    int_value(rgba.green),
                    int_value(rgba.blue),
                    a,
                ],
            )),
        }
    } else if let Value::List(vec, sep, bracketed) = red {
        if vec.len() > 3 {
            Err(Error::badarg(
                "3 elements",
                &Value::List(vec, sep, bracketed),
            ))
        } else {
            // If less than 3 arguments are passed, one of the arguments could be a variable containing
            // multiple values, so we want to preserve the original separator.
            let call_args = if vec.len() < 3 {
                CallArgs::new(vec![(
                    None,
                    Value::Literal(
                        Value::List(vec, sep, bracketed)
                            .format(Default::default())
                            .to_string(),
                        Quotes::None,
                    ),
                )])
            } else {
                CallArgs::new(vec.into_iter().map(|v| (None, v)).collect())
            };

            Ok(Value::Call(fn_name.into(), call_args))
        }
    } else {
        let green = s.get("green")?;
        let blue = s.get("blue")?;
        if let (Ok(red), Ok(green), Ok(blue), Ok(alpha)) = (
            to_int(&red),
            to_int(&green),
            to_int(&blue),
            if a.is_null() {
                Ok(Rational::one())
            } else {
                to_rational(&a)
            },
        ) {
            Ok(Value::rgba(red, green, blue, alpha))
        } else {
            Ok(make_call(fn_name, vec![red, green, blue, a]))
        }
    }
}

pub fn register(f: &mut BTreeMap<&'static str, SassFunction>) {
    def!(f, rgb(red, green, blue, alpha, color), |s| do_rgba(
        "rgb", s
    ));
    def!(f, rgba(red, green, blue, alpha, color), |s| do_rgba(
        "rgba", s
    ));
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
            let p = if wu == Unit::Percent {
                w.value / 100
            } else {
                w.value
            };
            let one = Rational::one();
            let w = p * 2 - one;
            let wa = a.alpha - b.alpha;

            let divis = w * wa + 1;
            let w1 =
                (if divis.is_zero() { w } else { (w + wa) / divis } + 1) / 2;
            let w2 = one - w1;

            let m_c = |c1, c2| w1 * c1 + w2 * c2;
            Ok(Value::rgba(
                m_c(a.red, b.red),
                m_c(a.green, b.green),
                m_c(a.blue, b.blue),
                a.alpha * p + b.alpha * (one - p),
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
        (ref by, Value::Numeric(Number { ref value, .. }, ref wu, ..))
            if value == &Rational::from_integer(100)
                && wu == &Unit::Percent =>
        {
            Ok(make_call("invert", vec![by.clone()]))
        }
        (value, weight) => Ok(make_call("invert", vec![value, weight])),
    });
}

fn int_value(v: Rational) -> Value {
    Value::scalar(v.to_integer())
}

fn to_int(v: &Value) -> Result<Rational, Error> {
    match v {
        Value::Numeric(v, Unit::Percent, _) => Ok(v.value * 255 / 100),
        Value::Numeric(v, ..) => Ok(v.value),
        v => Err(Error::badarg("number", &v)),
    }
}

fn to_rational(v: &Value) -> Result<Rational, Error> {
    match v {
        Value::Numeric(num, Unit::Percent, _) => Ok(num.value / 100),
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
