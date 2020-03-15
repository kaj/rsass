use crate::output::{Format, Formatted};
use num_rational::Rational;
use num_traits::{One, Signed, Zero};
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
        let t = self.value.value.to_integer();
        let skip_zero = self.format.is_compressed() || !self.value.lead_zero;
        if t == 0 {
            if self.value.value.is_negative() {
                out.write_str("-")?;
                if !skip_zero {
                    out.write_str("0")?;
                }
            } else if self.value.plus_sign {
                out.write_str("+0")?;
            } else if self.value.value.is_zero() || !skip_zero {
                out.write_char('0')?;
            }
        } else {
            if self.value.plus_sign && !t.is_negative() {
                out.write_char('+')?;
            }
            write!(out, "{}", t)?;
        }
        let mut f = self.value.value.fract().abs();
        if !f.is_zero() {
            out.write_char('.')?;
            for i in 0..(self.format.precision - 1) {
                f = f * 10;
                let digit = f.to_integer();
                f = f.fract();
                if (f + Rational::new(1, 10)
                    .pow((self.format.precision - i) as i32))
                    >= Rational::one()
                {
                    assert!(digit < 9);
                    write!(out, "{}", digit + 1)?;
                    return Ok(());
                } else {
                    write!(out, "{}", digit)?;
                }
                if f.is_zero() {
                    return Ok(());
                }
            }
            if !f.is_zero() {
                write!(out, "{}", (f * 10).round().to_integer())?;
            }
        }
        Ok(())
    }
}
