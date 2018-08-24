use super::{Error, SassFunction};
use css::Value;
use num_rational::Rational;
use num_traits::{One, Signed, Zero};
use std::collections::BTreeMap;
use value::{Number, Quotes, Rgba, Unit};
use variablescope::Scope;

pub fn register(f: &mut BTreeMap<&'static str, SassFunction>) {
    def!(
        f,
        adjust_color(
            color, red, green, blue, hue, saturation, lightness, alpha
        ),
        |s: &Scope| match &s.get("color") {
            &Value::Color(ref rgba, _) => {
                let h_adj = s.get("hue");
                let s_adj = s.get("saturation");
                let l_adj = s.get("lightness");
                let a_adj = s.get("alpha");
                if h_adj.is_null() && s_adj.is_null() && l_adj.is_null() {
                    Ok(Value::rgba(
                        c_add(rgba.red, s.get("red"))?,
                        c_add(rgba.green, s.get("green"))?,
                        c_add(rgba.blue, s.get("blue"))?,
                        c_add(rgba.alpha, a_adj)?,
                    ))
                } else {
                    let (h, s, l, alpha) = rgba.to_hsla();
                    Ok(Value::Color(
                        Rgba::from_hsla(
                            c_add(h, h_adj)? * Rational::new(1, 360),
                            sl_add(s, s_adj)?,
                            sl_add(l, l_adj)?,
                            c_add(alpha, a_adj)?,
                        ),
                        None,
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
        |s: &Scope| match &s.get("color") {
            &Value::Color(ref rgba, _) => {
                let h_adj = s.get("hue");
                let s_adj = s.get("saturation");
                let l_adj = s.get("lightness");
                let a_adj = s.get("alpha");
                if h_adj.is_null() && s_adj.is_null() && l_adj.is_null() {
                    Ok(Value::rgba(
                        c_comb(rgba.red, s.get("red"))?,
                        c_comb(rgba.green, s.get("green"))?,
                        c_comb(rgba.blue, s.get("blue"))?,
                        comb(rgba.alpha, a_adj)?,
                    ))
                } else {
                    let (h, s, l, alpha) = rgba.to_hsla();
                    Ok(Value::Color(
                        Rgba::from_hsla(
                            comb(h, h_adj)? * Rational::new(1, 360),
                            comb(s, s_adj)?,
                            comb(l, l_adj)?,
                            comb(alpha, a_adj)?,
                        ),
                        None,
                    ))
                }
            }
            v => Err(Error::badarg("color", v)),
        }
    );

    fn opacity(color: Value) -> Result<Value, Error> {
        match color {
            Value::Color(ref rgba, _) => {
                Ok(Value::Numeric(Number::new(rgba.alpha), Unit::None, true))
            }
            v => Err(Error::badarg("color", &v)),
        }
    }
    f.insert("opacity", func2!(opacity(color)));
    f.insert("alpha", func2!(opacity(color)));

    fn fade_in(color: Value, amount: Value) -> Result<Value, Error> {
        match color {
            Value::Color(rgba, _) => Ok(Value::rgba(
                rgba.red,
                rgba.green,
                rgba.blue,
                rgba.alpha + to_rational(amount)?,
            )),
            v => Err(Error::badarg("color", &v)),
        }
    }
    f.insert("fade_in", func2!(fade_in(color, amount)));
    f.insert("opacify", func2!(fade_in(color, amount)));

    fn fade_out(color: Value, amount: Value) -> Result<Value, Error> {
        match color {
            Value::Color(rgba, _) => Ok(Value::rgba(
                rgba.red,
                rgba.green,
                rgba.blue,
                rgba.alpha - to_rational(amount)?,
            )),
            v => Err(Error::badarg("color", &v)),
        }
    }
    f.insert("fade_out", func2!(fade_out(color, amount)));
    f.insert("transparentize", func2!(fade_out(color, amount)));

    def!(
        f,
        change_color(
            color, red, green, blue, hue, saturation, lightness, alpha
        ),
        |s: &Scope| match s.get("color") {
            Value::Color(rgba, _) => {
                let h_adj = s.get("hue");
                let s_adj = s.get("saturation");
                let l_adj = s.get("lightness");
                let a_adj = s.get("alpha");
                if h_adj.is_null() && s_adj.is_null() && l_adj.is_null() {
                    Ok(Value::rgba(
                        c_or(rgba.red, s.get("red"))?,
                        c_or(rgba.green, s.get("green"))?,
                        c_or(rgba.blue, s.get("blue"))?,
                        a_or(rgba.alpha, s.get("alpha"))?,
                    ))
                } else {
                    let (h, s, l, alpha) = rgba.to_hsla();
                    Ok(Value::Color(
                        Rgba::from_hsla(
                            a_or(h, h_adj)? * Rational::new(1, 360),
                            sl_or(s, s_adj)?,
                            sl_or(l, l_adj)?,
                            a_or(alpha, a_adj)?,
                        ),
                        None,
                    ))
                }
            }
            v => Err(Error::badarg("color", &v)),
        }
    );
    def!(f, ie_hex_str(color), |s| match s.get("color") {
        Value::Color(rgba, _) => Ok(Value::Literal(
            {
                let (r, g, b, a) = rgba.to_bytes();
                format!("#{:02X}{:02X}{:02X}{:02X}", a, r, g, b)
            },
            Quotes::None,
        )),
        v => Err(Error::badarg("color", &v)),
    });
}

fn c_add(orig: Rational, x: Value) -> Result<Rational, Error> {
    match x {
        Value::Null => Ok(orig),
        x => Ok(orig + to_rational(x)?),
    }
}
fn sl_add(orig: Rational, x: Value) -> Result<Rational, Error> {
    match x {
        Value::Null => Ok(orig),
        x => Ok(orig + to_rational_percent(x)?),
    }
}

fn c_or(orig: Rational, x: Value) -> Result<Rational, Error> {
    match x {
        Value::Null => Ok(orig),
        x => Ok(cap_u8(to_rational(x)?)),
    }
}
fn a_or(orig: Rational, x: Value) -> Result<Rational, Error> {
    match x {
        Value::Null => Ok(orig),
        x => to_rational(x),
    }
}
fn sl_or(orig: Rational, x: Value) -> Result<Rational, Error> {
    match x {
        Value::Null => Ok(orig),
        x => to_rational_percent(x),
    }
}

fn comb(orig: Rational, x: Value) -> Result<Rational, Error> {
    match x {
        Value::Null => Ok(orig),
        x => {
            let x = to_rational_percent(x)?;
            Ok(if x.is_positive() {
                orig + (Rational::one() - orig) * x
            } else {
                orig - orig * x.abs()
            })
        }
    }
}
fn c_comb(orig: Rational, x: Value) -> Result<Rational, Error> {
    let ff = Rational::new(255, 1);
    Ok(ff * comb(orig / ff, x)?)
}

fn cap_u8(n: Rational) -> Rational {
    let ff = Rational::new(255, 1);
    if n > ff {
        ff
    } else if n.is_negative() {
        Rational::zero()
    } else {
        n
    }
}

fn to_rational(v: Value) -> Result<Rational, Error> {
    match v {
        Value::Numeric(v, ..) => Ok(v.value),
        v => Err(Error::badarg("number", &v)),
    }
}

/// Gets a percentage as a fraction 0 .. 1.
/// If v is not a percentage, keep it as it is.
fn to_rational_percent(v: Value) -> Result<Rational, Error> {
    match v {
        Value::Numeric(v, Unit::Percent, _) => {
            Ok(v.value * Rational::new(1, 100))
        }
        Value::Numeric(v, ..) => Ok(v.value),
        v => Err(Error::badarg("number", &v)),
    }
}

#[cfg(test)]
mod test {
    use variablescope::test::do_evaluate;

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
