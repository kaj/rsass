use super::colors_hsl::{hsla_to_rgba, rgb_to_hsl};
use super::{Error, SassFunction};
use css::Value;
use num_rational::Rational;
use num_traits::{One, Signed, Zero};
use std::collections::BTreeMap;
use value::{Number, Quotes, Unit};
use variablescope::Scope;

pub fn register(f: &mut BTreeMap<&'static str, SassFunction>) {
    def!(
        f,
        adjust_color(
            color, red, green, blue, hue, saturation, lightness, alpha
        ),
        |s: &Scope| match &s.get("color") {
            &Value::Color(ref red, ref green, ref blue, ref alpha, _) => {
                let h_adj = s.get("hue");
                let s_adj = s.get("saturation");
                let l_adj = s.get("lightness");
                let a_adj = s.get("alpha");
                if h_adj.is_null() && s_adj.is_null() && l_adj.is_null() {
                    Ok(Value::rgba(
                        c_add(*red, s.get("red"))?,
                        c_add(*green, s.get("green"))?,
                        c_add(*blue, s.get("blue"))?,
                        c_add(*alpha, a_adj)?,
                    ))
                } else {
                    let (h, s, l) = rgb_to_hsl(red, green, blue);
                    Ok(hsla_to_rgba(
                        c_add(h, h_adj)? * Rational::new(1, 360),
                        sl_add(s, s_adj)?,
                        sl_add(l, l_adj)?,
                        c_add(*alpha, a_adj)?,
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
            &Value::Color(ref red, ref green, ref blue, ref alpha, _) => {
                let h_adj = s.get("hue");
                let s_adj = s.get("saturation");
                let l_adj = s.get("lightness");
                let a_adj = s.get("alpha");
                if h_adj.is_null() && s_adj.is_null() && l_adj.is_null() {
                    Ok(Value::rgba(
                        c_comb(*red, s.get("red"))?,
                        c_comb(*green, s.get("green"))?,
                        c_comb(*blue, s.get("blue"))?,
                        comb(*alpha, a_adj)?,
                    ))
                } else {
                    let (h, s, l) = rgb_to_hsl(red, green, blue);
                    Ok(hsla_to_rgba(
                        comb(h, h_adj)? * Rational::new(1, 360),
                        comb(s, s_adj)?,
                        comb(l, l_adj)?,
                        comb(*alpha, a_adj)?,
                    ))
                }
            }
            v => Err(Error::badarg("color", v)),
        }
    );

    fn opacity(color: Value) -> Result<Value, Error> {
        match color {
            Value::Color(_r, _g, _b, a, _) => Ok(Value::Numeric(
                Number::new(a, false),
                Unit::None,
                true,
            )),
            v => Err(Error::badarg("color", &v)),
        }
    }
    f.insert("opacity", func2!(opacity(color)));
    f.insert("alpha", func2!(opacity(color)));

    fn fade_in(color: Value, amount: Value) -> Result<Value, Error> {
        match color {
            Value::Color(red, green, blue, alpha, _) => Ok(Value::rgba(
                red,
                green,
                blue,
                alpha + to_rational(amount)?,
            )),
            v => Err(Error::badarg("color", &v)),
        }
    }
    f.insert("fade_in", func2!(fade_in(color, amount)));
    f.insert("opacify", func2!(fade_in(color, amount)));

    fn fade_out(color: Value, amount: Value) -> Result<Value, Error> {
        match color {
            Value::Color(red, green, blue, alpha, _) => Ok(Value::rgba(
                red,
                green,
                blue,
                alpha - to_rational(amount)?,
            )),
            v => Err(Error::badarg("color", &v)),
        }
    }
    f.insert("fade_out", func2!(fade_out(color, amount)));
    f.insert(
        "transparentize",
        func2!(fade_out(color, amount)),
    );

    def!(
        f,
        change_color(
            color, red, green, blue, hue, saturation, lightness, alpha
        ),
        |s: &Scope| match s.get("color") {
            Value::Color(red, green, blue, alpha, _) => {
                let h_adj = s.get("hue");
                let s_adj = s.get("saturation");
                let l_adj = s.get("lightness");
                let a_adj = s.get("alpha");
                if h_adj.is_null() && s_adj.is_null() && l_adj.is_null() {
                    Ok(Value::rgba(
                        c_or(red, s.get("red"))?,
                        c_or(green, s.get("green"))?,
                        c_or(blue, s.get("blue"))?,
                        a_or(alpha, s.get("alpha"))?,
                    ))
                } else {
                    let (h, s, l) = rgb_to_hsl(&red, &green, &blue);
                    Ok(hsla_to_rgba(
                        a_or(h, h_adj)? * Rational::new(1, 360),
                        sl_or(s, s_adj)?,
                        sl_or(l, l_adj)?,
                        a_or(alpha, a_adj)?,
                    ))
                }
            }
            v => Err(Error::badarg("color", &v)),
        }
    );
    def!(f, ie_hex_str(color), |s| match s.get("color") {
        Value::Color(r, g, b, alpha, _) => {
            fn to_byte(v: Rational) -> u8 {
                v.round().to_integer() as u8
            }
            Ok(Value::Literal(
                format!(
                    "#{:02X}{:02X}{:02X}{:02X}",
                    to_byte(alpha * Rational::new(255, 1)),
                    to_byte(r),
                    to_byte(g),
                    to_byte(b)
                ),
                Quotes::None,
            ))
        }
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
        assert_eq!(
            do_evaluate(&[], b"ie-hex-str(#abc);"),
            "#FFAABBCC"
        )
    }
    #[test]
    fn ie_hex_str_b() {
        assert_eq!(
            do_evaluate(&[], b"ie-hex-str(#3322BB);"),
            "#FF3322BB"
        )
    }
    #[test]
    fn ie_hex_str_c() {
        assert_eq!(
            do_evaluate(&[], b"ie-hex-str(rgba(0, 255, 0, 0.5));"),
            "#8000FF00"
        )
    }
}
