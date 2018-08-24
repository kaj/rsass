use super::{Error, SassFunction};
use css::Value;
use num_rational::Rational;
use num_traits::{One, Zero};
use std::collections::BTreeMap;
use value::{Number, Rgba, Unit};
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
            &Value::Color(ref rgba, _) => {
                let h_adj = s.get("degrees");
                let (h, s, l, alpha) = rgba.to_hsla();
                let h = a_comb(h, h_adj)?;
                Ok(hsla_to_rgba(h * Rational::new(1, 360), s, l, alpha))
            }
            v => Err(Error::badarg("color", v)),
        }
    });
    def!(f, complement(color), |s: &Scope| match &s.get("color") {
        &Value::Color(ref rgba, _) => {
            let (h, s, l, alpha) = rgba.to_hsla();
            let h = (h + Rational::from_integer(180)) * Rational::new(1, 360);
            Ok(hsla_to_rgba(h, s, l, alpha))
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
            &Value::Color(ref rgba, _) => {
                let (h, s, l, alpha) = rgba.to_hsla();
                let s = comb(s, args.get("amount"))?;
                Ok(hsla_to_rgba(h * Rational::new(1, 360), s, l, alpha))
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
            &Value::Color(ref rgba, _) => {
                let (h, s, l, alpha) = rgba.to_hsla();
                let l = comb(l, args.get("amount"))?;
                Ok(hsla_to_rgba(h * Rational::new(1, 360), s, l, alpha))
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
            &Value::Color(ref rgba, _) => {
                let (h, s, l, alpha) = rgba.to_hsla();
                let l = comb(l, args.get("amount"))?;
                Ok(hsla_to_rgba(h * Rational::new(1, 360), s, l, alpha))
            }
            v => Err(Error::badarg("color", v)),
        }
    });
    def!(f, hue(color), |args: &Scope| match &args.get("color") {
        &Value::Color(ref rgba, _) => {
            let (h, _s, _l, _a) = rgba.to_hsla();
            Ok(Value::Numeric(Number::new(h), Unit::Deg, true))
        }
        v => Err(Error::badarg("color", v)),
    });
    def!(f, saturation(color), |args| match &args.get("color") {
        &Value::Color(ref rgba, _) => {
            let (_h, s, _l, _a) = rgba.to_hsla();
            Ok(percentage(s))
        }
        v => Err(Error::badarg("color", v)),
    });
    def!(f, lightness(color), |args| match &args.get("color") {
        &Value::Color(ref rgba, _) => {
            let (_h, _s, l, _a) = rgba.to_hsla();
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
            &Value::Color(ref rgba, _) => {
                let (h, s, l, alpha) = rgba.to_hsla();
                let s = comb(s, args.get("amount"))?;
                Ok(hsla_to_rgba(h * Rational::new(1, 360), s, l, alpha))
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
    Value::Color(Rgba::from_hsla(hue, sat, lig, a), None)
}

fn percentage(v: Rational) -> Value {
    Value::Numeric(
        Number::new(v * Rational::from_integer(100)),
        Unit::Percent,
        true,
    )
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
        Value::Numeric(v, ..) => {
            let v = v.value;
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
