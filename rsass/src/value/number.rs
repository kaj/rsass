use crate::output::{Format, Formatted};
use std::fmt::{self, Write};
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

/// The actual number part of a numeric sass or css value.
///
/// Only the actual numeric value is included, not any unit.
/// Internally, a number is represented as a floating-point value.
#[derive(Clone)]
pub struct Number {
    value: f64,
}
impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        (self.value - other.value).abs() / self.value.abs() <= f64::EPSILON
    }
}
impl Eq for Number {}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            Some(std::cmp::Ordering::Equal)
        } else {
            self.value.partial_cmp(&other.value)
        }
    }
}

impl Neg for Number {
    type Output = Self;
    fn neg(self) -> Self {
        -&self
    }
}
impl Neg for &Number {
    type Output = Number;
    fn neg(self) -> Number {
        (-self.value).into()
    }
}

impl Mul for Number {
    type Output = Self;
    fn mul(mut self, rhs: Self) -> Self {
        self.value *= rhs.value;
        self
    }
}
impl Mul for &Number {
    type Output = Number;
    fn mul(self, rhs: Self) -> Self::Output {
        (self.value * rhs.value).into()
    }
}

impl Number {
    /// Get a copy of this number, rounded away from zero.
    pub fn ceil(&self) -> Self {
        Self {
            value: self.value.ceil(),
        }
    }
    /// Get a copy of this number, rounded down.
    pub fn floor(&self) -> Self {
        Self {
            value: self.value.floor(),
        }
    }
    /// Get a copy of this number, rounded towards zero.
    pub fn trunc(&self) -> Self {
        Self {
            value: self.value.trunc(),
        }
    }
    /// Get a copy of this number, rounded to nearest integer.
    pub fn round(&self) -> Self {
        Self {
            value: self.value.round(),
        }
    }
    /// Get the sign of this number.
    pub fn signum(&self) -> Self {
        // The sass spec says sign(-0) is -0
        // ... which may be a bug, but here's to compatibility:
        if self.value == 0. {
            self.clone()
        } else {
            self.value.signum().into()
        }
    }
    /// Computes the absolute value of the number, retaining the flags.
    pub fn abs(&self) -> Self {
        self.value.abs().into()
    }
    /// Return true if this number is less than zero.
    pub fn is_negative(&self) -> bool {
        self.value.is_sign_negative()
    }

    /// Return true if this value is neither inifinte nor NaN.
    pub fn is_finite(&self) -> bool {
        self.value.is_finite()
    }

    /// Convert this number to an `i64` if it is (very close to)
    /// integer and in range for `i64`, otherwise return `Err(self)`.
    pub fn into_integer(self) -> Result<i64, Self> {
        let int = self.value.round() as i64;
        if ((int as f64) - self.value).abs() <= f32::EPSILON.into() {
            Ok(int)
        } else {
            Err(self)
        }
    }

    /// Computes self^p.
    pub fn powi(self, p: i32) -> Self {
        self.value.powi(p).into()
    }

    /// Get a reference to this `Value` bound to an output format.
    pub fn format(&self, format: Format) -> Formatted<Self> {
        Formatted {
            value: self,
            format,
        }
    }
}

impl From<i64> for Number {
    fn from(value: i64) -> Self {
        (value as f64).into()
    }
}
impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Self::from(f64::from(value))
    }
}
impl From<usize> for Number {
    fn from(value: usize) -> Self {
        (value as f64).into()
    }
}

impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Self { value }
    }
}
impl Add for Number {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value + rhs.value,
        }
    }
}
impl Mul<i64> for Number {
    type Output = Self;
    fn mul(self, rhs: i64) -> Self {
        Self {
            value: self.value * (rhs as f64),
        }
    }
}
impl Div for Number {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Self {
            value: self.value / rhs.value,
        }
    }
}
impl Div<i64> for Number {
    type Output = Self;
    fn div(self, rhs: i64) -> Self {
        Self {
            value: self.value / (rhs as f64),
        }
    }
}

impl From<Number> for f64 {
    fn from(val: Number) -> Self {
        val.value
    }
}

impl<'a> Div for &'a Number {
    type Output = Number;
    fn div(self, rhs: Self) -> Self::Output {
        (self.value / rhs.value).into()
    }
}

impl Rem for &Number {
    type Output = Number;
    fn rem(self, rhs: Self) -> Self::Output {
        // The modulo operator in rust handles negative values different from
        // num-rational.
        let a = self.value;
        let b = rhs.value;
        let result = a % b;
        let result =
            if a != 0. && (b.is_sign_negative() != a.is_sign_negative()) {
                if b.is_finite() {
                    result + b
                } else {
                    f64::NAN
                }
            } else {
                result
            };
        Number { value: result }
    }
}

impl Sub for &Number {
    type Output = Number;
    fn sub(self, rhs: Self) -> Self::Output {
        (self.value - rhs.value).into()
    }
}

impl<'a> fmt::Display for Formatted<'a, Number> {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        let s = self.value.value;
        if s.is_nan() {
            write!(out, "NaN")
        } else if s.is_infinite() {
            write!(
                out,
                "{}infinity",
                if s.is_sign_negative() { "-" } else { "" }
            )
        } else {
            let mut frac = s.fract();
            let mut whole = s.trunc().abs();
            let mut dec = String::with_capacity(if frac == 0. {
                0
            } else {
                self.format.precision
            });

            if frac != 0. {
                let max_decimals = 16 - whole.log10().ceil() as usize;
                for _ in 1..max_decimals.min(self.format.precision) {
                    frac *= 10.;
                    write!(dec, "{}", (frac as i8).abs())?;
                    frac = frac.fract();
                    if frac == 0. {
                        break;
                    }
                }
                if frac != 0. {
                    let end = (frac * 10.).round().abs() as u8;
                    if end == 10 {
                        loop {
                            match dec.pop() {
                                Some('9') => continue,
                                None => {
                                    whole += 1.;
                                    break;
                                }
                                Some(c) => {
                                    dec.push(char::from(c as u8 + 1));
                                    break;
                                }
                            }
                        }
                    } else if end == 0 {
                        loop {
                            match dec.pop() {
                                Some('0') => continue,
                                None => break,
                                Some(c) => {
                                    dec.push(c);
                                    break;
                                }
                            }
                        }
                    } else {
                        write!(dec, "{end}")?;
                    }
                }
            }

            if s.is_sign_negative() && (whole != 0. || !dec.is_empty()) {
                out.write_char('-')?;
            }

            let skip_zero = self.format.is_compressed();
            if !(whole == 0. && skip_zero && !dec.is_empty()) {
                write!(out, "{whole}")?;
            }

            if !dec.is_empty() {
                write!(out, ".{dec}")?;
            }
            Ok(())
        }
    }
}

impl fmt::Debug for Number {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(out, "Number({:?})", self.value)
    }
}

#[test]
fn debug_integer() {
    assert_eq!(format!("{:?}", Number::from(17)), "Number(17.0)",);
}
#[test]
fn debug_long_integer() {
    assert_eq!(format!("{:#?}", Number::from(17)), "Number(17.0)",);
}

#[test]
fn debug_float() {
    assert_eq!(format!("{:?}", Number::from(17.5)), "Number(17.5)",);
}

#[test]
fn debug_int_value() {
    assert_eq!(
        format!("{:#?}", crate::sass::Value::scalar(17)),
        "Numeric(\
         \n    Number(17.0); UnitSet [],\
         \n)",
    );
}
