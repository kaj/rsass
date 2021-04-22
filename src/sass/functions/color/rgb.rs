use super::{
    check, check_pct_rational_range, expected_to, get_checked, get_color,
    make_call, nospecial_value, CheckedArg, Error, FunctionMap, Name,
};
use crate::css::{CallArgs, Value};
use crate::output::Format;
use crate::value::{ListSeparator, Quotes, Rational, Rgba};
use crate::Scope;
use num_traits::{one, Zero};

fn do_rgba(fn_name: &str, s: &Scope) -> Result<Value, Error> {
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
                a.as_ratio()?,
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
            Ok(rgba_from_values(&r, &g, &b, &a)?
                .unwrap_or_else(|| make_call(fn_name, vec![r, g, b, a])))
        } else {
            Ok(preserve_call(fn_name, vec, sep, bracketed))
        }
    } else {
        let green = s.get("green")?;
        let blue = s.get("blue")?;
        Ok(rgba_from_values(&red, &green, &blue, &a)?
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
) -> Result<Option<Value>, Error> {
    let r = to_channel(r, name!(red))?;
    let g = to_channel(g, name!(green))?;
    let b = to_channel(b, name!(blue))?;
    let a = match a {
        Value::Null => Some(one()),
        a => nospecial_value(a, name!(alpha), to_rational)?,
    };
    if let (Some(r), Some(g), Some(b), Some(a)) = (r, g, b, a) {
        Ok(Some(Rgba::new(r, g, b, a).into()))
    } else {
        Ok(None)
    }
}

fn to_channel(v: &Value, name: Name) -> Result<Option<Rational>, Error> {
    // Note: This null check is not quite correct, it should kind of
    // belong in values_from_list.
    if v.is_null() {
        Ok(None)
    } else {
        nospecial_value(v, name, to_int)
    }
}

pub fn preserve_call(
    fn_name: &str,
    vec: Vec<Value>,
    sep: Option<ListSeparator>,
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

pub fn register(f: &mut Scope) {
    def!(
        f,
        _rgb(
            red = b"null",
            green = b"null",
            blue = b"null",
            alpha = b"null",
            color = b"null",
            channels = b"null"
        ),
        |s| { do_rgba("rgb", s) }
    );
    def!(
        f,
        _rgba(
            red = b"null",
            green = b"null",
            blue = b"null",
            alpha = b"null",
            color = b"null",
            channels = b"null"
        ),
        |s| { do_rgba("rgba", s) }
    );
    def!(f, red(color), |s| {
        Ok(Value::scalar(get_color(s, "color")?.to_rgba().red()))
    });
    def!(f, green(color), |s| {
        Ok(Value::scalar(get_color(s, "color")?.to_rgba().green()))
    });
    def!(f, blue(color), |s| {
        Ok(Value::scalar(get_color(s, "color")?.to_rgba().blue()))
    });
    def!(f, mix(color1, color2, weight = b"50%"), |s| {
        let a = get_color(s, "color1")?;
        let a = a.to_rgba();
        let b = get_color(s, "color2")?;
        let b = b.to_rgba();
        let w = get_checked(s, name!(weight), check_pct_rational_range)?;
        let one: Rational = one();

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
    });
    def!(f, invert(color, weight = b"100%"), |s| {
        match s.get("color")? {
            Value::Color(col, _) => {
                let rgba = col.to_rgba();
                let w =
                    get_checked(s, name!(weight), check_pct_rational_range)?;
                let inv = |v: Rational| -(v - 255) * w + v * -(w - 1);
                Ok(Rgba::new(
                    inv(rgba.red()),
                    inv(rgba.green()),
                    inv(rgba.blue()),
                    rgba.alpha(),
                )
                .into())
            }
            col => {
                let w =
                    get_checked(s, name!(weight), check_pct_rational_range)?;
                if w == one() {
                    match col {
                        v @ Value::Numeric(..) => {
                            Ok(make_call("invert", vec![v]))
                        }
                        v => Err(format!(
                            "{} is not a color",
                            v.format(Format::introspect())
                        ))
                        .named(name!(color)),
                    }
                } else {
                    Err(Error::error("Only one argument may be passed to the plain-CSS invert() function"))
                }
            }
        }
    });
}

pub fn expose(m: &Scope, global: &mut FunctionMap) {
    for (gname, lname) in &[
        (name!(rgb), name!(_rgb)),
        (name!(rgba), name!(_rgba)),
        (name!(blue), name!(blue)),
        (name!(green), name!(green)),
        (name!(invert), name!(invert)),
        (name!(mix), name!(mix)),
        (name!(red), name!(red)),
    ] {
        global.insert(gname.clone(), m.get_function(&lname).unwrap().clone());
    }
}

fn int_value(v: Rational) -> Value {
    Value::scalar(v.to_integer())
}

fn to_int(v: Value) -> Result<Rational, String> {
    let v = check::numeric(v)?;
    let r = v.value.as_ratio().map_err(|e| e.to_string())?;
    if v.unit.is_percent() {
        Ok(r * 255 / 100)
    } else {
        Ok(r)
    }
}

pub fn to_rational(v: Value) -> Result<Rational, String> {
    let num = check::numeric(v)?;
    let r = num.value.as_ratio().map_err(|e| e.to_string())?;
    if num.unit.is_percent() {
        Ok(r / 100)
    } else if num.unit.is_none() {
        Ok(r)
    } else {
        Err(expected_to(&num, "have no units or \"%\""))
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
