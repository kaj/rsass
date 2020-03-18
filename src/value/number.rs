use crate::output::{Format, Formatted};
use num_rational::Rational;
use num_traits::{Signed, Zero};
use std::fmt::{self, Write};
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

/// The actual number part of a numeric sass or css value.
///
/// Only the actual numeric value is included, not any unit, but flags
/// to show a leading plus sign and/or leading zero (for values
/// between -1 and 1) is included.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Number {
    pub value: Rational,
    pub plus_sign: bool,
    pub lead_zero: bool,
}

impl Number {
    /// Computes the absolute value of the number, retaining the flags.
    pub fn abs(self) -> Self {
        Number {
            value: self.value.abs(),
            plus_sign: self.plus_sign,
            lead_zero: self.lead_zero,
        }
    }
    /// Returns true if the number is an integer.
    pub fn is_integer(&self) -> bool {
        self.value.is_integer()
    }
    /// Converts to an integer, rounding towards zero.
    pub fn to_integer(&self) -> isize {
        self.value.to_integer()
    }
    pub fn format(&self, format: Format) -> Formatted<Self> {
        Formatted {
            value: self,
            format,
        }
    }
}

impl<T> From<T> for Number
where
    T: Into<Rational>,
{
    fn from(value: T) -> Number {
        Number {
            value: value.into(),
            plus_sign: false,
            lead_zero: true,
        }
    }
}

impl Add for Number {
    type Output = Number;
    fn add(self, rhs: Self) -> Self::Output {
        Number::from(self.value + rhs.value)
    }
}
impl<'a> Div for &'a Number {
    type Output = Number;
    fn div(self, rhs: Self) -> Self::Output {
        Number::from(self.value / rhs.value)
    }
}
impl<'a> Mul for &'a Number {
    type Output = Number;
    fn mul(self, rhs: Self) -> Self::Output {
        Number::from(self.value * rhs.value)
    }
}
impl<'a> Rem for &'a Number {
    type Output = Number;
    fn rem(self, rhs: Self) -> Self::Output {
        Number::from(self.value % rhs.value)
    }
}
impl<'a> Neg for &'a Number {
    type Output = Number;
    fn neg(self) -> Number {
        Number::from(-self.value)
    }
}

impl<'a> Sub for &'a Number {
    type Output = Number;
    fn sub(self, rhs: Self) -> Self::Output {
        Number::from(self.value - rhs.value)
    }
}
impl Zero for Number {
    fn zero() -> Self {
        Number::from(0)
    }
    fn is_zero(&self) -> bool {
        self.value.is_zero()
    }
}

impl<'a> fmt::Display for Formatted<'a, Number> {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        if self.value.value.is_negative() {
            out.write_char('-')?;
        }

        let mut whole = self.value.value.to_integer().abs();
        let mut dec = String::new();

        let mut frac = self.value.value.fract();
        if frac != Rational::from_integer(0) {
            dec.write_char('.')?;
            for _ in 0..(self.format.precision - 1) {
                frac *= Rational::from_integer(10);
                write!(dec, "{}", frac.to_integer().abs())?;
                frac = frac.fract();
                if frac == Rational::from_integer(0) {
                    break;
                }
            }
            if frac != Rational::from_integer(0) {
                let end = (frac * Rational::from_integer(10))
                    .round()
                    .abs()
                    .to_integer();
                if end == 10 {
                    loop {
                        match dec.pop().unwrap() {
                            '9' => continue,
                            '.' => {
                                whole += 1;
                                break;
                            }
                            c => {
                                dec.push_str(
                                    &(c.to_digit(10).unwrap() + 1)
                                        .to_string(),
                                );
                                break;
                            }
                        }
                    }
                } else if end == 0 {
                    loop {
                        match dec.pop().unwrap() {
                            '0' => continue,
                            '.' => break,
                            c => {
                                dec.push(c);
                                break;
                            }
                        }
                    }
                } else {
                    write!(dec, "{}", end)?;
                }
            }
        }

        let skip_zero = self.format.is_compressed() || !self.value.lead_zero;
        if !(whole == 0 && skip_zero && !dec.is_empty()) {
            write!(out, "{}", whole)?;
        }

        write!(out, "{}", dec)?;
        Ok(())
    }
}
