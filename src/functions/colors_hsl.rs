use super::{Error, SassFunction};
use css::Value;
use num_rational::Rational;
use num_traits::{One, Signed, Zero};
use std::collections::BTreeMap;
use value::Unit;
use variablescope::Scope;

pub fn register(f: &mut BTreeMap<&'static str, SassFunction>) {
    def!(f, hsl(hue, saturation, lightness), |s: &Scope| Ok(
        hsla_to_rgba(
            to_rational(s.get("hue"))? * Rational::new(1, 360),
            to_rational_percent(s.get("saturation"))?,
            to_rational_percent(s.get("lightness"))?,
            Rational::one()
        )
    ));
    def!(f, hsla(hue, saturation, lightness, alpha), |s: &Scope| Ok(
        hsla_to_rgba(
            to_rational(s.get("hue"))? * Rational::new(1, 360),
            to_rational_percent(s.get("saturation"))?,
            to_rational_percent(s.get("lightness"))?,
            to_rational(s.get("alpha"))?
        )
    ));
    def!(f, adjust_hue(color, degrees), |s: &Scope| {
        fn a_comb(orig: Rational, x: Value) -> Result<Rational, Error> {
            match x {
                Value::Null => Ok(orig),
                x => Ok(orig + to_rational(x)?),
            }
        }
        match &s.get("color") {
            &Value::Color(ref red, ref green, ref blue, ref alpha, _) => {
                let h_adj = s.get("degrees");
                let (h, s, l) = rgb_to_hsl(red, green, blue);
                let h = a_comb(h, h_adj)?;
                Ok(hsla_to_rgba(h * Rational::new(1, 360), s, l, *alpha))
            }
            v => Err(Error::badarg("color", v)),
        }
    });
    def!(f, complement(color), |s: &Scope| match &s.get("color") {
        &Value::Color(ref red, ref green, ref blue, ref alpha, _) => {
            let (h, s, l) = rgb_to_hsl(red, green, blue);
            let h = (h + Rational::from_integer(180)) * Rational::new(1, 360);
            Ok(hsla_to_rgba(h, s, l, *alpha))
        }
        v => Err(Error::badarg("color", v)),
    });
    def!(f, saturate(color, amount), |args: &Scope| {
        fn comb(orig: Rational, x: Value) -> Result<Rational, Error> {
            match x {
                Value::Null => Ok(orig),
                x => Ok(cap_percentage(orig + to_rational_percent(x)?)),
            }
        }
        match &args.get("color") {
            &Value::Color(ref red, ref green, ref blue, ref alpha, _) => {
                let (h, s, l) = rgb_to_hsl(red, green, blue);
                let s = comb(s, args.get("amount"))?;
                Ok(hsla_to_rgba(h * Rational::new(1, 360), s, l, *alpha))
            }
            v => Err(Error::badarg("color", v)),
        }
    });
    def!(f, lighten(color, amount), |args: &Scope| {
        fn comb(orig: Rational, x: Value) -> Result<Rational, Error> {
            match x {
                Value::Null => Ok(orig),
                x => Ok(cap_percentage(orig + to_rational_percent(x)?)),
            }
        }
        match &args.get("color") {
            &Value::Color(ref red, ref green, ref blue, ref alpha, _) => {
                let (h, s, l) = rgb_to_hsl(red, green, blue);
                let l = comb(l, args.get("amount"))?;
                Ok(hsla_to_rgba(h * Rational::new(1, 360), s, l, *alpha))
            }
            v => Err(Error::badarg("color", v)),
        }
    });
    def!(f, darken(color, amount), |args: &Scope| {
        fn comb(orig: Rational, x: Value) -> Result<Rational, Error> {
            match x {
                Value::Null => Ok(orig),
                x => Ok(cap_percentage(orig - to_rational_percent(x)?)),
            }
        }
        match &args.get("color") {
            &Value::Color(ref red, ref green, ref blue, ref alpha, _) => {
                let (h, s, l) = rgb_to_hsl(red, green, blue);
                let l = comb(l, args.get("amount"))?;
                Ok(hsla_to_rgba(h * Rational::new(1, 360), s, l, *alpha))
            }
            v => Err(Error::badarg("color", v)),
        }
    });
    def!(f, hue(color), |args: &Scope| match &args.get("color") {
        &Value::Color(ref red, ref green, ref blue, ref _alpha, _) => {
            let (h, _s, _l) = rgb_to_hsl(red, green, blue);
            Ok(Value::Numeric(h, Unit::Deg, false, true))
        }
        v => Err(Error::badarg("color", v)),
    });
    def!(f, saturation(color), |args| match &args.get("color") {
        &Value::Color(ref red, ref green, ref blue, ref _alpha, _) => {
            let (_h, s, _l) = rgb_to_hsl(red, green, blue);
            Ok(percentage(s))
        }
        v => Err(Error::badarg("color", v)),
    });
    def!(f, lightness(color), |args| match &args.get("color") {
        &Value::Color(ref red, ref green, ref blue, ref _alpha, _) => {
            let (_h, _s, l) = rgb_to_hsl(red, green, blue);
            Ok(percentage(l))
        }
        v => Err(Error::badarg("color", v)),
    });
    def!(f, desaturate(color, amount), |args: &Scope| {
        fn comb(orig: Rational, x: Value) -> Result<Rational, Error> {
            match x {
                Value::Null => Ok(orig),
                x => Ok(cap_percentage(orig - to_rational_percent(x)?)),
            }
        }
        match &args.get("color") {
            &Value::Color(ref red, ref green, ref blue, ref alpha, _) => {
                let (h, s, l) = rgb_to_hsl(red, green, blue);
                let s = comb(s, args.get("amount"))?;
                Ok(hsla_to_rgba(h * Rational::new(1, 360), s, l, *alpha))
            }
            v => Err(Error::badarg("color", v)),
        }
    });
}

pub fn hsla_to_rgba(
    hue: Rational,
    sat: Rational,
    lig: Rational,
    a: Rational,
) -> Value {
    let (r, g, b) = hsl_to_rgb(hue, sat, lig);
    Value::rgba(frac_to_int(r), frac_to_int(g), frac_to_int(b), a)
}

fn percentage(v: Rational) -> Value {
    Value::Numeric(v * Rational::from_integer(100), Unit::Percent, false, true)
}

/// Convert hue (degrees) / sat (0 .. 1) / lighness (0 .. 1) ro rgb (0 .. 1)
fn hsl_to_rgb(
    hue: Rational,
    sat: Rational,
    l: Rational,
) -> (Rational, Rational, Rational) {
    if sat.is_zero() {
        (l, l, l)
    } else {
        fn hue2rgb(p: Rational, q: Rational, t: Rational) -> Rational {
            let t = t - t.floor();
            if t < Rational::new(1, 6) {
                p + (q - p) * Rational::from_integer(6) * t
            } else if t < Rational::new(1, 2) {
                q
            } else if t < Rational::new(2, 3) {
                p
                    + (q - p) * (Rational::new(2, 3) - t)
                        * Rational::from_integer(6)
            } else {
                p
            }
        }
        let q = if l < Rational::new(1, 2) {
            l * (Rational::one() + sat)
        } else {
            l + sat - l * sat
        };
        let p = Rational::from_integer(2) * l - q;

        (
            hue2rgb(p, q, hue + Rational::new(1, 3)),
            hue2rgb(p, q, hue),
            hue2rgb(p, q, hue - Rational::new(1, 3)),
        )
    }
}

/// Convert rgb (0 .. 255) to hue (degrees) / sat (0 .. 1) / lighness (0 .. 1)
pub fn rgb_to_hsl(
    red: &Rational,
    green: &Rational,
    blue: &Rational,
) -> (Rational, Rational, Rational) {
    let ff = Rational::from_integer(255);
    let (red, green, blue) = (red / ff, green / ff, blue / ff);
    let (max, min, largest) = max_min_largest(red, green, blue);
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
                (green - blue) / d + if green < blue {
                    Rational::from_integer(6)
                } else {
                    Rational::zero()
                }
            }
            1 => (blue - red) / d + Rational::from_integer(2),
            _ => (red - green) / d + Rational::from_integer(4),
        } * Rational::new(360, 6);
        (h, s, mid)
    }
}

// Find which of three numbers are largest and smallest
fn max_min_largest(
    a: Rational,
    b: Rational,
    c: Rational,
) -> (Rational, Rational, u32) {
    let v = [(a, 0), (b, 1), (c, 2)];
    let max = v.iter().max().unwrap();
    let min = v.iter().min().unwrap();
    (max.0, min.0, max.1)
}

/// Convert a value in the 0 .. 1 range to u8
fn frac_to_int(v: Rational) -> Rational {
    cap_u8(Rational::from_integer(255) * v)
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
        Value::Numeric(v, ..) => Ok(v),
        v => Err(Error::badarg("number", &v)),
    }
}

/// Gets a percentage as a fraction 0 .. 1.
/// If v is not a percentage, keep it as it is.
fn to_rational_percent(v: Value) -> Result<Rational, Error> {
    match v {
        Value::Numeric(v, Unit::Percent, ..) => Ok(v * Rational::new(1, 100)),
        Value::Numeric(v, ..) => {
            if v <= Rational::new(1, 1) {
                Ok(v)
            } else {
                Ok(v * Rational::new(1, 100))
            }
        }
        v => Err(Error::badarg("number", &v)),
    }
}

fn cap_percentage(r: Rational) -> Rational {
    let zero = Rational::zero();
    if r < zero {
        zero
    } else if r > Rational::one() {
        Rational::one()
    } else {
        r
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
    assert_eq!("gray", do_evaluate(&[], b"hsl(300, 0%, 50%);"))
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
