use super::{make_call, Error, SassFunction};
use crate::css::Value;
use crate::value::{Quotes, Unit};
use crate::variablescope::Scope;
use num_rational::Rational;
use num_traits::{One, Signed, Zero};
use std::collections::BTreeMap;

pub fn register(f: &mut BTreeMap<&'static str, SassFunction>) {
    def!(
        f,
        adjust_color(
            color, red, green, blue, hue, saturation, lightness, alpha
        ),
        |s: &Scope| match &s.get("color")? {
            &Value::Color(ref rgba, _) => {
                let c_add = |orig: Rational, name: &str| match s.get(name)? {
                    Value::Null => Ok(orig),
                    x => to_rational(x).map(|x| orig + x),
                };
                let h_adj = s.get("hue")?;
                let s_adj = s.get("saturation")?;
                let l_adj = s.get("lightness")?;
                if h_adj.is_null() && s_adj.is_null() && l_adj.is_null() {
                    Ok(Value::rgba(
                        c_add(rgba.red, "red")?,
                        c_add(rgba.green, "green")?,
                        c_add(rgba.blue, "blue")?,
                        c_add(rgba.alpha, "alpha")?,
                    ))
                } else {
                    let (h, s, l, alpha) = rgba.to_hsla();
                    let sl_add = |orig: Rational, x: Value| match x {
                        Value::Null => Ok(orig),
                        x => to_rational_percent(x).map(|x| orig + x),
                    };
                    Ok(Value::hsla(
                        c_add(h, "hue")?,
                        sl_add(s, s_adj)?,
                        sl_add(l, l_adj)?,
                        c_add(alpha, "alpha")?,
                    ))
                }
            }
            v => Err(Error::badarg("color", v)),
        }
    );
    def!(
        f,
        scale_color(
            color, red, green, blue, hue, saturation, lightness, alpha
        ),
        |s: &Scope| match &s.get("color")? {
            &Value::Color(ref rgba, _) => {
                let h_adj = s.get("hue")?;
                let s_adj = s.get("saturation")?;
                let l_adj = s.get("lightness")?;
                let a_adj = s.get("alpha")?;

                let comb = |orig: Rational, x: Value, max: Rational| match x {
                    Value::Null => Ok(orig),
                    x => to_rational_percent(x).map(|x| {
                        if x.is_positive() {
                            orig + (max - orig) * x
                        } else {
                            orig + orig * x
                        }
                    }),
                };
                let one = Rational::one();
                let ff = Rational::from_integer(255);
                if h_adj.is_null() && s_adj.is_null() && l_adj.is_null() {
                    Ok(Value::rgba(
                        comb(rgba.red, s.get("red")?, ff)?,
                        comb(rgba.green, s.get("green")?, ff)?,
                        comb(rgba.blue, s.get("blue")?, ff)?,
                        comb(rgba.alpha, a_adj, one)?,
                    ))
                } else {
                    let (h, s, l, alpha) = rgba.to_hsla();
                    Ok(Value::hsla(
                        comb(h, h_adj, one)?,
                        comb(s, s_adj, one)?,
                        comb(l, l_adj, one)?,
                        comb(alpha, a_adj, one)?,
                    ))
                }
            }
            v => Err(Error::badarg("color", v)),
        }
    );

    def!(f, opacity(color), |args| match args.get("color")? {
        Value::Color(ref rgba, _) => Ok(Value::scalar(rgba.alpha)),
        v => Ok(make_call("opacity", vec![v])),
    });
    def!(f, alpha(color), |args| match args.get("color")? {
        Value::Color(ref rgba, _) => Ok(Value::scalar(rgba.alpha)),
        v => Ok(make_call("alpha", vec![v])),
    });

    fn fade_in(color: Value, amount: Value) -> Result<Value, Error> {
        match (color, amount) {
            (Value::Color(rgba, _), Value::Numeric(v, ..)) => {
                let a = rgba.alpha + v.value;
                Ok(Value::rgba(rgba.red, rgba.green, rgba.blue, a))
            }
            (c, v) => Err(Error::badargs(&["color", "number"], &[&c, &v])),
        }
    }
    f.insert("fade_in", func2!(fade_in(color, amount)));
    f.insert("opacify", func2!(fade_in(color, amount)));

    fn fade_out(color: Value, amount: Value) -> Result<Value, Error> {
        match (color, amount) {
            (Value::Color(rgba, _), Value::Numeric(v, ..)) => {
                let a = rgba.alpha - v.value;
                Ok(Value::rgba(rgba.red, rgba.green, rgba.blue, a))
            }
            (c, v) => Err(Error::badargs(&["color", "number"], &[&c, &v])),
        }
    }
    f.insert("fade_out", func2!(fade_out(color, amount)));
    f.insert("transparentize", func2!(fade_out(color, amount)));

    def!(
        f,
        change_color(
            color, red, green, blue, hue, saturation, lightness, alpha
        ),
        |s: &Scope| match s.get("color")? {
            Value::Color(rgba, _) => {
                let h_adj = s.get("hue")?;
                let s_adj = s.get("saturation")?;
                let l_adj = s.get("lightness")?;

                let c_or = |name: &str, orig: Rational| match s.get(name)? {
                    Value::Null => Ok(orig),
                    x => to_rational(x),
                };
                let a_or = |name: &str, orig: Rational| match s.get(name)? {
                    Value::Null => Ok(orig),
                    x => to_rational(x),
                };
                let sl_or = |x: Value, orig: Rational| match x {
                    Value::Null => Ok(orig),
                    x => to_rational_percent(x),
                };
                if h_adj.is_null() && s_adj.is_null() && l_adj.is_null() {
                    Ok(Value::rgba(
                        c_or("red", rgba.red)?,
                        c_or("green", rgba.green)?,
                        c_or("blue", rgba.blue)?,
                        a_or("alpha", rgba.alpha)?,
                    ))
                } else {
                    let (h, s, l, alpha) = rgba.to_hsla();
                    Ok(Value::hsla(
                        a_or("hue", h)?,
                        sl_or(s_adj, s)?,
                        sl_or(l_adj, l)?,
                        a_or("alpha", alpha)?,
                    ))
                }
            }
            v => Err(Error::badarg("color", &v)),
        }
    );
    def!(f, ie_hex_str(color), |s| match s.get("color")? {
        Value::Color(rgba, _) => {
            let (r, g, b, a) = rgba.to_bytes();
            Ok(Value::Literal(
                format!("#{:02X}{:02X}{:02X}{:02X}", a, r, g, b),
                Quotes::None,
            ))
        }
        v => Err(Error::badarg("color", &v)),
    });
}

fn to_rational(v: Value) -> Result<Rational, Error> {
    match v {
        Value::Numeric(v, ..) => Ok(v.value),
        v => Err(Error::badarg("number", &v)),
    }
}

/// Try to get percentage part (i.e. a number -1 .. 1) from a value.
///
/// If the value is a percentage, get it as part (e.g. 50% => 0.5).
/// If the value is in the correct range, return it as is.
/// Otherwise, guesstimate that it is a percenatage anyway and divide by 100.
fn to_rational_percent(v: Value) -> Result<Rational, Error> {
    match v {
        Value::Null => Ok(Rational::zero()),
        Value::Numeric(v, Unit::Percent, _) => Ok(v.value / 100),
        Value::Numeric(v, ..) => Ok(if v.value.abs() < Rational::one() {
            v.value
        } else {
            v.value / 100
        }),
        v => Err(Error::badarg("number", &v)),
    }
}

#[cfg(test)]
mod test {
    use crate::variablescope::test::do_evaluate;

    #[test]
    fn ie_hex_str_a() {
        assert_eq!(do_evaluate(&[], b"ie-hex-str(#abc);"), "#FFAABBCC")
    }
    #[test]
    fn ie_hex_str_b() {
        assert_eq!(do_evaluate(&[], b"ie-hex-str(#3322BB);"), "#FF3322BB")
    }
    #[test]
    fn ie_hex_str_c() {
        assert_eq!(
            do_evaluate(&[], b"ie-hex-str(rgba(0, 255, 0, 0.5));"),
            "#8000FF00"
        )
    }
}
