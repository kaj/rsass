use super::{get_color, make_call, Error, Module, SassFunction};
use crate::css::{CallArgs, Value};
use crate::output::Format;
use crate::value::{ListSeparator, Numeric, Quotes, Rgba, Unit};
use crate::variablescope::Scope;
use num_rational::Rational;
use num_traits::{one, One, Zero};

fn do_rgba(fn_name: &str, s: &dyn Scope) -> Result<Value, Error> {
    let a = s.get("alpha")?;
    let red = s.get("red")?;
    let red = if red.is_null() { s.get("color")? } else { red };
    if let Value::Color(rgba, _) = red {
        let rgba = rgba.to_rgba();
        let a = if a.is_null() { s.get("green")? } else { a };
        match a {
            Value::Numeric(a, ..) => Ok(Rgba::new(
                rgba.red(),
                rgba.green(),
                rgba.blue(),
                a.value.as_ratio()?,
            )
            .into()),
            _ => Ok(make_call(
                fn_name,
                vec![
                    int_value(rgba.red()),
                    int_value(rgba.green()),
                    int_value(rgba.blue()),
                    a,
                ],
            )),
        }
    } else if let Value::List(vec, sep, bracketed) = if red.is_null() {
        s.get("channels")?
    } else {
        red.clone()
    } {
        if let Some((r, g, b, a)) = values_from_list(&vec) {
            Ok(rgba_from_values(&r, &g, &b, &a)
                .unwrap_or_else(|| make_call(fn_name, vec![r, g, b, a])))
        } else {
            Ok(preserve_call(fn_name, vec, sep, bracketed))
        }
    } else {
        let green = s.get("green")?;
        let blue = s.get("blue")?;
        Ok(rgba_from_values(&red, &green, &blue, &a)
            .unwrap_or_else(|| make_call(fn_name, vec![red, green, blue, a])))
    }
}

pub fn values_from_list(
    vec: &[Value],
) -> Option<(Value, Value, Value, Value)> {
    use crate::value::Operator::Div;

    if vec.len() == 3 {
        if let Value::BinOp(a, _, Div, _, b) = &vec[2] {
            if let (Value::Numeric(..), Value::Numeric(..)) = (&**a, &**b) {
                Some((vec[0].clone(), vec[1].clone(), *a.clone(), *b.clone()))
            } else {
                None
            }
        } else {
            Some((
                vec[0].clone(),
                vec[1].clone(),
                vec[2].clone(),
                Value::Null,
            ))
        }
    } else {
        None
    }
}

fn rgba_from_values(
    r: &Value,
    g: &Value,
    b: &Value,
    a: &Value,
) -> Option<Value> {
    let r = to_int(r).ok()?;
    let g = to_int(g).ok()?;
    let b = to_int(b).ok()?;
    let a = if a.is_null() {
        Rational::one()
    } else {
        to_rational(a).ok()?
    };
    Some(Rgba::new(r, g, b, a).into())
}

pub fn preserve_call(
    fn_name: &str,
    vec: Vec<Value>,
    sep: ListSeparator,
    bracketed: bool,
) -> Value {
    Value::Call(
        fn_name.into(),
        CallArgs::new(vec![(
            None,
            Value::Literal(
                Value::List(vec, sep, bracketed)
                    .format(Default::default())
                    .to_string(),
                Quotes::None,
            ),
        )]),
    )
}

pub fn register(f: &mut Module) {
    def!(f, _rgb(red, green, blue, alpha, color, channels), |s| {
        do_rgba("rgb", s)
    });
    def!(f, _rgba(red, green, blue, alpha, color, channels), |s| {
        do_rgba("rgba", s)
    });
    fn num(v: Rational) -> Result<Value, Error> {
        Ok(Numeric::new(v, Unit::None).into())
    }
    def!(f, red(color), |s| {
        num(get_color(s, "color")?.to_rgba().red())
    });
    def!(f, green(color), |s| {
        num(get_color(s, "color")?.to_rgba().green())
    });
    def!(f, blue(color), |s| {
        num(get_color(s, "color")?.to_rgba().blue())
    });
    def!(f, mix(color1, color2, weight = b"50%"), |s| match (
        s.get("color1")?,
        s.get("color2")?,
        s.get("weight")?,
    ) {
        (Value::Color(a, _), Value::Color(b, _), w @ Value::Numeric(..)) => {
            let a = a.to_rgba();
            let b = b.to_rgba();
            let w = to_rational(&w)?;
            let one = Rational::one();

            let w_a = {
                let wa = a.alpha() - b.alpha();
                let w2 = w * 2 - 1;
                let divis = w2 * wa + 1;
                if divis.is_zero() {
                    w
                } else {
                    (((w2 + wa) / divis) + 1) / 2
                }
            };
            let w_b = one - w_a;

            let m_c = |c_a, c_b| w_a * c_a + w_b * c_b;
            Ok(Rgba::new(
                m_c(a.red(), b.red()),
                m_c(a.green(), b.green()),
                m_c(a.blue(), b.blue()),
                a.alpha() * w + b.alpha() * (one - w),
            )
            .into())
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
        (Value::Color(rgba, _), Value::Numeric(w, ..)) => {
            let rgba = rgba.to_rgba();
            let w = w
                .as_unit(Unit::None)
                .ok_or_else(|| {
                    Error::S(format!(
                        "Expected unitless or percent, got {}",
                        w.format(Format::introspect())
                    ))
                })?
                .as_ratio()?;
            let inv = |v: Rational| -(v - 255) * w + v * -(w - 1);
            Ok(Rgba::new(
                inv(rgba.red()),
                inv(rgba.green()),
                inv(rgba.blue()),
                rgba.alpha(),
            )
            .into())
        }
        (col, Value::Numeric(w, ..))
            if w.as_unit(Unit::None) == Some(one()) =>
        {
            Ok(make_call("invert", vec![col]))
        }
        (value, weight) => Ok(make_call("invert", vec![value, weight])),
    });
}

pub fn expose(m: &Module, global: &mut Module) {
    for &(gname, lname) in &[
        ("rgb", "_rgb"),
        ("rgba", "_rgba"),
        ("blue", "blue"),
        ("green", "green"),
        ("invert", "invert"),
        ("mix", "mix"),
        ("red", "red"),
    ] {
        global.expose(gname, m, lname);
    }
}

fn int_value(v: Rational) -> Value {
    Value::scalar(v.to_integer())
}

fn to_int(v: &Value) -> Result<Rational, Error> {
    match v {
        Value::Numeric(v, ..) => {
            if v.unit == Unit::Percent {
                Ok(v.value.as_ratio()? * 255 / 100)
            } else {
                v.value.as_ratio()
            }
        }
        v => Err(Error::badarg("number", &v)),
    }
}

fn to_rational(v: &Value) -> Result<Rational, Error> {
    match v {
        Value::Numeric(num, _) if num.unit == Unit::Percent => {
            Ok(num.value.as_ratio()? / 100)
        }
        Value::Numeric(num, _) if num.unit == Unit::None => {
            num.value.as_ratio()
        }
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
