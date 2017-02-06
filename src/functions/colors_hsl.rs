use super::{Error, SassFunction, badarg};
use formalargs::FormalArgs;
use num_rational::Rational;
use num_traits::Zero;
use std::collections::BTreeMap;
use valueexpression::{Unit, Value};
use variablescope::Scope;

pub fn register(f: &mut BTreeMap<&'static str, SassFunction>) {
    f.insert("hsl",
             func!((hue, saturation, lightness), |s: &Scope| {
        let hue = to_rational(s.get("hue"))? * Rational::new(1, 360);
        let sat = to_rational_percent(s.get("saturation"))?;
        let light = to_rational_percent(s.get("lightness"))?;
        let (r, g, b) = hsl_to_rgb(hue, sat, light);
        Ok(Value::Color(frac_to_int(r),
                        frac_to_int(g),
                        frac_to_int(b),
                        Rational::from_integer(1),
                        None))
    }));
    f.insert("hsla",
             func!((hue, saturation, lightness, alpha), |s: &Scope| {
        let hue = to_rational(s.get("hue"))? * Rational::new(1, 360);
        let sat = to_rational_percent(s.get("saturation"))?;
        let light = to_rational_percent(s.get("lightness"))?;
        let alpha = to_rational(s.get("alpha"))?;
        let (r, g, b) = hsl_to_rgb(hue, sat, light);
        Ok(Value::Color(frac_to_int(r),
                        frac_to_int(g),
                        frac_to_int(b),
                        alpha,
                        None))
    }));
    f.insert("adjust_hue",
             func!((color, degrees), |s: &Scope| {
        fn a_comb(orig: Rational, x: Value) -> Result<Rational, Error> {
            match x {
                Value::Null => Ok(orig),
                x => Ok(orig + to_rational(x)?),
            }
        }
        match &s.get("color") {
            &Value::Color(ref red, ref green, ref blue, ref alpha, _) => {
                let h_adj = s.get("degrees");
                let (h, s, l) = rgb_to_hsl(u8_to_frac(*red),
                                           u8_to_frac(*green),
                                           u8_to_frac(*blue));
                let h = a_comb(h, h_adj)?;
                let (r, g, b) = hsl_to_rgb(h * Rational::new(1, 360), s, l);
                Ok(Value::Color(frac_to_int(r),
                                frac_to_int(g),
                                frac_to_int(b),
                                alpha.clone(),
                                None))
            }
            v => Err(badarg("color", v)),
        }
    }));
    f.insert("complement",
             func!((color), |s: &Scope| {
        match &s.get("color") {
            &Value::Color(ref red, ref green, ref blue, ref alpha, _) => {
                let (h, s, l) = rgb_to_hsl(u8_to_frac(*red),
                                           u8_to_frac(*green),
                                           u8_to_frac(*blue));
                let (r, g, b) = hsl_to_rgb((h  + Rational::from_integer(180)) *
                                           Rational::new(1, 360),
                                           s,
                                           l);
                Ok(Value::Color(frac_to_int(r),
                                frac_to_int(g),
                                frac_to_int(b),
                                alpha.clone(),
                                None))
            }
            v => Err(badarg("color", v)),
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

#[test]
fn test_hsl_black() {
    assert_eq!("black", do_evaluate(&[], b"hsl(17, 32%, 0%);"))
}
#[test]
fn test_hsl_white() {
    assert_eq!("white", do_evaluate(&[], b"hsl(300, 82%, 100%);"))
}
#[test]
fn test_hsl_gray() {
    assert_eq!("grey", do_evaluate(&[], b"hsl(300, 0%, 50%);"))
}
#[test]
fn test_hsl_red() {
    assert_eq!("#f7c9c9", do_evaluate(&[], b"hsl(0, 75%, 88%);"))
}
#[test]
fn test_hsl_yellow() {
    assert_eq!("#ffff42", do_evaluate(&[], b"hsl(60, 100%, 63%);"))
}
#[test]
fn test_hsl_blue_magenta() {
    assert_eq!("#6118aa", do_evaluate(&[], b"hsl(270, 75%, 38%);"))
}

#[cfg(test)]
use super::super::variablescope::test::do_evaluate;
