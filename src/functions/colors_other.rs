use super::{Error, SassFunction, badarg};
use super::colors_hsl::hsla_to_rgba;
use formalargs::FormalArgs;
use num_rational::Rational;
use num_traits::{Signed, Zero};
use std::collections::BTreeMap;
use valueexpression::{Unit, Value};
use variablescope::Scope;

pub fn register(f: &mut BTreeMap<&'static str, SassFunction>) {
    f.insert("adjust_color",
             func!((color,
                    red,
                    green,
                    blue,
                    hue,
                    saturation,
                    lightness,
                    alpha),
                   |s: &Scope| {
        fn c_comb(orig: Rational, x: Value) -> Result<Rational, Error> {
            match x {
                Value::Null => Ok(orig),
                x => Ok(orig + to_rational(x)?),
            }
        }
        fn a_comb(orig: Rational, x: Value) -> Result<Rational, Error> {
            match x {
                Value::Null => Ok(orig),
                x => Ok(orig + to_rational(x)?),
            }
        }
        fn sl_comb(orig: Rational, x: Value) -> Result<Rational, Error> {
            match x {
                Value::Null => Ok(orig),
                x => Ok(orig + to_rational_percent(x)?),
            }
        }
        match s.get("color") {
            Value::Color(red, green, blue, alpha, _) => {
                let h_adj = s.get("hue");
                let s_adj = s.get("saturation");
                let l_adj = s.get("lightness");
                let a_adj = s.get("alpha");
                if h_adj != Value::Null || s_adj != Value::Null ||
                   l_adj != Value::Null {
                    let (h, s, l) = rgb_to_hsl(u8_to_frac(red),
                                               u8_to_frac(green),
                                               u8_to_frac(blue));
                    Ok(hsla_to_rgba(a_comb(h, h_adj)? * Rational::new(1, 360),
                                    sl_comb(s, s_adj)?,
                                    sl_comb(l, l_adj)?,
                                    a_comb(alpha, a_adj)?))
                } else {
                    Ok(Value::rgba(c_comb(red, s.get("red"))?,
                                   c_comb(green, s.get("green"))?,
                                   c_comb(blue, s.get("blue"))?,
                                   a_comb(alpha, s.get("alpha"))?))
                }
            }
            v => Err(badarg("color", &v)),
        }
    }));
    f.insert("change_color",
             func!((color,
                    red,
                    green,
                    blue,
                    hue,
                    saturation,
                    lightness,
                    alpha),
                   |s: &Scope| {
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
        match s.get("color") {
            Value::Color(red, green, blue, alpha, _) => {
                let h_adj = s.get("hue");
                let s_adj = s.get("saturation");
                let l_adj = s.get("lightness");
                let a_adj = s.get("alpha");
                if h_adj != Value::Null || s_adj != Value::Null ||
                   l_adj != Value::Null {
                    let (h, s, l) = rgb_to_hsl(u8_to_frac(red),
                                               u8_to_frac(green),
                                               u8_to_frac(blue));
                    Ok(hsla_to_rgba(a_or(h, h_adj)? * Rational::new(1, 360),
                                    sl_or(s, s_adj)?,
                                    sl_or(l, l_adj)?,
                                    a_or(alpha, a_adj)?))
                } else {
                    Ok(Value::rgba(c_or(red, s.get("red"))?,
                                   c_or(green, s.get("green"))?,
                                   c_or(blue, s.get("blue"))?,
                                   a_or(alpha, s.get("alpha"))?))
                }
            }
            v => Err(badarg("color", &v)),
        }
    }));
}

/// Convert rgb (0 .. 1) to hue (degrees) / sat (0 .. 1) / lighness (0 .. 1)
fn rgb_to_hsl(r: Rational,
              g: Rational,
              b: Rational)
              -> (Rational, Rational, Rational) {
    let (max, min) = {
        let v = [(0, r), (0, g), (0, b)];
        (v.iter().max().unwrap().clone(), v.iter().min().unwrap().clone())
    };
    let largest = max.0;
    let (max, min) = (max.1, min.1);
    let half = Rational::new(1, 2);
    let mid = (max + min) * half;

    if max == min {
        (Rational::zero(), Rational::zero(), mid)
    } else {
        let d = max - min;
        let s = if mid > half {
            d / (Rational::from_integer(2) - max - min)
        } else {
            d / (max + min)
        };
        let h = match largest {
            0 => {
                (g - b) / d +
                if g < b {
                    Rational::from_integer(6)
                } else {
                    Rational::zero()
                }
            }
            1 => (b - r) / d + Rational::from_integer(2),
            _ => (r - g) / d + Rational::from_integer(4),
        } * Rational::new(360, 6);
        (h, s, mid)
    }
}

fn u8_to_frac(v: Rational) -> Rational {
    v * Rational::new(1, 255)
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
        Value::Numeric(v, _, _) => Ok(v),
        v => Err(badarg("number", &v)),
    }
}

/// Gets a percentage as a fraction 0 .. 1.
/// If v is not a percentage, keep it as it is.
fn to_rational_percent(v: Value) -> Result<Rational, Error> {
    match v {
        Value::Numeric(v, Unit::Percent, _) => Ok(v * Rational::new(1, 100)),
        Value::Numeric(v, _, _) => Ok(v),
        v => Err(badarg("number", &v)),
    }
}
