use formalargs::FormalArgs;
use num_rational::Rational;
use num_traits::Zero;
use std::collections::BTreeMap;
use super::{Error, SassFunction, badarg};
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
        fn c_comb(orig: u8, x: Value) -> Result<u8, Error> {
            match x {
                Value::Null => Ok(orig),
                x => {
                    Ok(cap_u8(orig as isize + try!(to_rational(x)).round().to_integer()))
                }
            }
        }
        fn a_comb(orig: Rational, x: Value) -> Result<Rational, Error> {
            match x {
                Value::Null => Ok(orig),
                x => Ok(orig + try!(to_rational(x))),
            }
        }
        fn sl_comb(orig: Rational, x: Value) -> Result<Rational, Error> {
            match x {
                Value::Null => Ok(orig),
                x => Ok(orig + try!(to_rational_percent(x))),
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
                    let h = try!(a_comb(h, h_adj));
                    let s = try!(sl_comb(s, s_adj));
                    let l = try!(sl_comb(l, l_adj));
                    let (r, g, b) = hsl_to_rgb(h * Rational::new(1, 360), s, l);
                    Ok(Value::Color(frac_to_int(r),
                                    frac_to_int(g),
                                    frac_to_int(b),
                                    try!(a_comb(alpha, a_adj)),
                                    None))
                } else {
                    Ok(Value::Color(try!(c_comb(red, s.get("red"))),
                                    try!(c_comb(green, s.get("green"))),
                                    try!(c_comb(blue, s.get("blue"))),
                                    try!(a_comb(alpha, s.get("alpha"))),
                                    None))
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
        fn c_or(orig: u8, x: Value) -> Result<u8, Error> {
            match x {
                Value::Null => Ok(orig),
                x => Ok(cap_u8(try!(to_rational(x)).round().to_integer())),
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
                    let h = try!(a_or(h, h_adj));
                    let s = try!(sl_or(s, s_adj));
                    let l = try!(sl_or(l, l_adj));
                    let (r, g, b) = hsl_to_rgb(h * Rational::new(1, 360), s, l);
                    Ok(Value::Color(frac_to_int(r),
                                    frac_to_int(g),
                                    frac_to_int(b),
                                    try!(a_or(alpha, a_adj)),
                                    None))
                } else {
                    Ok(Value::Color(try!(c_or(red, s.get("red"))),
                                    try!(c_or(green, s.get("green"))),
                                    try!(c_or(blue, s.get("blue"))),
                                    try!(a_or(alpha, s.get("alpha"))),
                                    None))
                }
            }
            v => Err(badarg("color", &v)),
        }
    }));
}

/// Convert hue (degrees) / sat (0 .. 1) / lighness (0 .. 1) ro rgb (0 .. 1)
fn hsl_to_rgb(h: Rational,
              s: Rational,
              l: Rational)
              -> (Rational, Rational, Rational) {
    let one = Rational::from_integer(1);
    if s.is_zero() {
        (l, l, l)
    } else {
        fn hue2rgb(p: Rational, q: Rational, t: Rational) -> Rational {
            let t = t - t.floor();
            if t < Rational::new(1, 6) {
                p + (q - p) * Rational::from_integer(6) * t
            } else if t < Rational::new(1, 2) {
                q
            } else if t < Rational::new(2, 3) {
                p +
                (q - p) * (Rational::new(2, 3) - t) * Rational::from_integer(6)
            } else {
                p
            }
        }
        let q = if l < Rational::new(1, 2) {
            l * (one + s)
        } else {
            l + s - l * s
        };
        let p = Rational::from_integer(2) * l - q;

        (hue2rgb(p, q, h + Rational::new(1, 3)),
         hue2rgb(p, q, h),
         hue2rgb(p, q, h - Rational::new(1, 3)))
    }
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

/// Convert a value in the 0 .. 1 range to u8
fn frac_to_int(v: Rational) -> u8 {
    cap_u8((Rational::from_integer(255) * v).round().to_integer())
}

fn u8_to_frac(v: u8) -> Rational {
    Rational::new(v as isize, 255)
}

fn cap_u8(n: isize) -> u8 {
    if n > 255 {
        255
    } else if n < 0 {
        0
    } else {
        n as u8
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
