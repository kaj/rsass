use num_rational::Rational;
use num_traits::{Signed, Zero};
use std::fmt::{self, Write};
use std::ops::{Add, Div, Mul, Neg, Sub};

/// How many digits of precision to use when outputting decimal numbers.
pub mod precision {
    use std::sync::atomic::{AtomicUsize, Ordering};

    static PRECISION: AtomicUsize = AtomicUsize::new(5);

    /// Set how many digits of precision to use when outputting decimal numbers.
    ///
    /// This modifies a global singleton and should probably be called only once,
    /// at program start.
    pub fn set(precision: usize) {
        PRECISION.store(precision, Ordering::Relaxed);
    }
    pub fn get() -> usize {
        PRECISION.load(Ordering::Relaxed)
    }
}

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

impl fmt::Display for Number {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        let prec = precision::get() as u32;
        let p_fac = 10_isize.pow(prec);
        let value = (self.value * p_fac).round().to_integer();
        eprintln!("Format {:?}; round to {}; {} / {}", self.value, prec, value, p_fac);
        let whole = value / p_fac;
        let decimals = value.abs() - whole.abs() * p_fac;
        eprintln!(" ... whole: {}, decimals: {}", whole, decimals);
        let skip_zero = out.alternate() || !self.lead_zero;
        if whole.is_zero() {
            if value.is_negative() {
                out.write_str("-")?;
                if !skip_zero {
                    out.write_str("0")?;
                }
            } else if self.plus_sign {
                out.write_str("+0")?;
            } else if decimals.is_zero() || !skip_zero {
                out.write_char('0')?;
            }
        } else {
            if self.plus_sign && !whole.is_negative() {
                out.write_char('+')?;
            }
            write!(out, "{}", whole)?;
        }
        if !decimals.is_zero() {
            out.write_char('.')?;
            let mut rem = decimals;
            let mut scale = p_fac;
            while rem > 0 {
                scale /= 10;
                let t = rem / scale;
                rem = rem - t * scale;
                write!(out, "{}", t)?;
            }
        }
        Ok(())
    }
}
