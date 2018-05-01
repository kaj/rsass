use num_rational::Rational;
use num_traits::{Signed, Zero};
use std::fmt::{self, Write};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Number {
    pub value: Rational,
    pub plus_sign: bool,
}

impl Number {
    pub fn new(value: Rational, plus_sign: bool) -> Self {
        Number {
            value,
            plus_sign,
        }
    }
    pub fn from_integer(n: isize) -> Self {
        Number {
            value: Rational::from_integer(n),
            plus_sign: false,
        }
    }
    pub fn abs(self) -> Self {
        Number {
            value: self.value.abs(),
            plus_sign: self.plus_sign,
        }
    }
    pub fn is_integer(&self) -> bool {
        self.value.is_integer()
    }
    pub fn to_integer(&self) -> isize {
        self.value.to_integer()
    }
}

impl Add for Number {
    type Output = Number;
    fn add(self, rhs: Self) -> Self::Output {
        Number::new(self.value + rhs.value, false)
    }
}
impl<'a> Div for &'a Number {
    type Output = Number;
    fn div(self, rhs: Self) -> Self::Output {
        Number::new(self.value / rhs.value, false)
    }
}
impl<'a> Mul for &'a Number {
    type Output = Number;
    fn mul(self, rhs: Self) -> Self::Output {
        Number::new(self.value * rhs.value, false)
    }
}
impl<'a> Neg for &'a Number {
    type Output = Number;
    fn neg(self) -> Number {
        Number::new(-self.value, self.plus_sign)
    }
}

impl<'a> Sub for &'a Number {
    type Output = Number;
    fn sub(self, rhs: Self) -> Self::Output {
        Number::new(self.value - rhs.value, false)
    }
}
impl Zero for Number {
    fn zero() -> Self {
        Number::new(Rational::zero(), false)
    }
    fn is_zero(&self) -> bool {
        self.value.is_zero()
    }
}

impl fmt::Display for Number {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        let t = self.value.to_integer();
        let skip_zero = out.alternate();
        if t == 0 {
            if self.value.is_negative() {
                out.write_str("-0")?;
            } else if self.plus_sign {
                out.write_str("+0")?;
            } else if self.value.is_zero() || !skip_zero {
                out.write_char('0')?;
            }
        } else {
            if self.plus_sign && !t.is_negative() {
                out.write_char('+')?;
            }
            write!(out, "{}", t)?;
        }
        let mut f = self.value.fract().abs();
        if !f.is_zero() {
            out.write_char('.')?;
            for _ in 0..4 {
                f *= 10;
                write!(out, "{}", f.to_integer())?;
                f = f.fract();
                if f.is_zero() {
                    break;
                }
            }
            if !f.is_zero() {
                write!(out, "{}", (f * 10).round().to_integer())?;
            }
        }
        Ok(())
    }
}
