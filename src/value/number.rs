use crate::output::{Format, Formatted};
use num_integer::Integer;
use num_rational::Ratio;
use num_traits::{Signed, Zero};
use std::fmt::{self, Write};
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

/// The actual number part of a numeric sass or css value.
///
/// Only the actual numeric value is included, not any unit, but flags
/// to show a leading plus sign and/or leading zero (for values
/// between -1 and 1) is included.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Number<N>
where
    N: Clone + Integer + Signed,
{
    pub value: Ratio<N>,
    pub plus_sign: bool,
    pub lead_zero: bool,
}

impl<N> Number<N>
where
    N: Clone + Integer + Signed,
{
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
    pub fn to_integer(&self) -> N {
        self.value.to_integer()
    }

    pub fn format(&self, format: Format) -> Formatted<Self> {
        Formatted {
            value: self,
            format,
        }
    }
}

impl<N, T> From<T> for Number<N>
where
    T: Into<Ratio<N>>,
    N: Clone + Integer + Signed,
{
    fn from(value: T) -> Number<N> {
        Number {
            value: value.into(),
            plus_sign: false,
            lead_zero: true,
        }
    }
}

impl<N> Add for Number<N>
where
    N: Clone + Integer + Signed,
{
    type Output = Number<N>;
    fn add(self, rhs: Self) -> Self::Output {
        Number::from(self.value + rhs.value)
    }
}

impl<'a, N> Div for &'a Number<N>
where
    N: Clone + Integer + Signed,
{
    type Output = Number<N>;
    fn div(self, rhs: Self) -> Self::Output {
        Number::from(&self.value / &rhs.value)
    }
}

impl<'a, N> Mul for &'a Number<N>
where
    N: Clone + Integer + Signed,
{
    type Output = Number<N>;
    fn mul(self, rhs: Self) -> Self::Output {
        Number::from(&self.value * &rhs.value)
    }
}

impl<'a, N> Rem for &'a Number<N>
where
    N: Clone + Integer + Signed,
{
    type Output = Number<N>;
    fn rem(self, rhs: Self) -> Self::Output {
        Number::from(&self.value % &rhs.value)
    }
}

impl<'a, N> Neg for &'a Number<N>
where
    N: Clone + Integer + Signed,
{
    type Output = Number<N>;
    fn neg(self) -> Number<N> {
        Number::from(-&self.value)
    }
}

impl<'a, N> Sub for &'a Number<N>
where
    N: Clone + Integer + Signed,
{
    type Output = Number<N>;
    fn sub(self, rhs: Self) -> Self::Output {
        Number::from(&self.value - &rhs.value)
    }
}

impl Zero for Number<isize> {
    fn zero() -> Self {
        Number::from(0)
    }
    fn is_zero(&self) -> bool {
        self.value.is_zero()
    }
}

impl<'a, N> fmt::Display for Formatted<'a, Number<N>>
where
    N: Clone + fmt::Display + Integer + Signed + From<i8>,
{
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        let ten: N = 10i8.into();
        let mut frac = self.value.value.fract();

        let mut whole = self.value.value.to_integer().abs();
        let mut dec = String::with_capacity(if frac.is_zero() {
            0
        } else {
            self.format.precision
        });

        if !frac.is_zero() {
            for _ in 0..(self.format.precision - 1) {
                frac = frac * &ten;
                write!(dec, "{}", frac.to_integer().abs())?;
                frac = frac.fract();
                if frac.is_zero() {
                    break;
                }
            }
            if !frac.is_zero() {
                let end = (frac * &ten).round().abs().to_integer();
                if end == ten {
                    loop {
                        match dec.pop() {
                            Some('9') => continue,
                            None => {
                                whole = whole + N::one();
                                break;
                            }
                            Some(c) => {
                                dec.push(char::from(c as u8 + 1));
                                break;
                            }
                        }
                    }
                } else if end.is_zero() {
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
                    write!(dec, "{}", end)?;
                }
            }
        }

        if self.value.value.is_negative()
            && (!whole.is_zero() || !dec.is_empty())
        {
            out.write_char('-')?;
        }

        let skip_zero = self.format.is_compressed();
        if !(whole.is_zero() && skip_zero && !dec.is_empty()) {
            write!(out, "{}", whole)?;
        }

        if !dec.is_empty() {
            write!(out, ".{}", dec)?;
        }
        Ok(())
    }
}
