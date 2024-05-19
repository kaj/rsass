use crate::output::{Format, Formatted};
use num_bigint::BigInt;
use num_integer::Integer;
use num_rational::Ratio;
pub use num_rational::Rational64 as Rational;
use num_traits::{
    CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, One, Signed, ToPrimitive,
    Zero,
};
use std::cmp::Ordering;
use std::fmt::{self, Write};
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

/// The actual number part of a numeric sass or css value.
///
/// Only the actual numeric value is included, not any unit.
/// Internally, a number is represented as either a rational or a
/// floating-point value as needed.
#[derive(Clone, PartialEq, Eq, PartialOrd)]
pub struct Number {
    value: NumValue,
}

#[derive(Clone, Debug)]
enum NumValue {
    Rational(Rational),
    // Note: Using a Box here is essentially a double indirection, but
    // it also makes the size of a NumValue a lot smaller (24 bytes
    // for a Ratio<i64> plus denominator and padding rather than the
    // 48 bytes (two Vec headers) that is put in this box.
    BigRational(Box<Ratio<BigInt>>),
    Float(f64),
}

impl From<Rational> for NumValue {
    fn from(value: Rational) -> Self {
        Self::Rational(value)
    }
}
impl From<i64> for NumValue {
    fn from(value: i64) -> Self {
        Self::Rational(Rational::from_integer(value))
    }
}
impl From<Ratio<BigInt>> for NumValue {
    fn from(value: Ratio<BigInt>) -> Self {
        Self::BigRational(Box::new(value))
    }
}
impl From<f64> for NumValue {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl NumValue {
    pub fn is_negative(&self) -> bool {
        match self {
            Self::Rational(s) => s.is_negative(),
            Self::BigRational(s) => s.is_negative(),
            Self::Float(s) => s.is_sign_negative(),
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
        match &self.value {
            NumValue::Rational(s) => (-s).into(),
            NumValue::BigRational(s) => (-s.as_ref()).into(),
            NumValue::Float(s) => (-s).into(),
        }
    }
}

impl Eq for NumValue {}
impl PartialEq for NumValue {
    fn eq(&self, rhs: &Self) -> bool {
        self.partial_cmp(rhs) == Some(Ordering::Equal)
    }
}
impl PartialOrd for NumValue {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        match (self, rhs) {
            (Self::Rational(s), Self::Rational(r)) => s.partial_cmp(r),
            (Self::Rational(s), Self::BigRational(r)) => {
                biggen(s).partial_cmp(r)
            }
            (Self::BigRational(s), Self::Rational(r)) => {
                s.as_ref().partial_cmp(&biggen(r))
            }
            (Self::BigRational(s), Self::BigRational(r)) => s.partial_cmp(r),
            (Self::Float(s), r) => s.partial_cmp(&r.into()),
            (s, Self::Float(r)) => f64::from(s).partial_cmp(r),
        }
    }
}

impl Mul for NumValue {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        &self * &rhs
    }
}
impl Mul for &NumValue {
    type Output = NumValue;
    fn mul(self, rhs: Self) -> NumValue {
        match (self, rhs) {
            (NumValue::Rational(s), NumValue::Rational(r)) => {
                if (s.is_zero() && r.is_negative())
                    || (r.is_zero() && s.is_negative())
                {
                    (-0.0).into()
                } else {
                    s.checked_mul(r)
                        .map(Into::into)
                        .unwrap_or_else(|| (biggen(s) * biggen(r)).into())
                }
            }
            (NumValue::Rational(s), NumValue::BigRational(r)) => {
                if (s.is_zero() && r.is_negative())
                    || (r.is_zero() && s.is_negative())
                {
                    (-0.0).into()
                } else {
                    (biggen(s) * r.as_ref()).into()
                }
            }
            (NumValue::BigRational(s), NumValue::Rational(r)) => {
                if (s.is_zero() && r.is_negative())
                    || (r.is_zero() && s.is_negative())
                {
                    (-0.0).into()
                } else {
                    (s.as_ref() * biggen(r)).into()
                }
            }
            (NumValue::BigRational(s), NumValue::BigRational(r)) => {
                if (s.is_zero() && r.is_negative())
                    || (r.is_zero() && s.is_negative())
                {
                    (-0.0).into()
                } else {
                    (s.as_ref() * r.as_ref()).into()
                }
            }
            (NumValue::Float(s), r) => (s * f64::from(r)).into(),
            (s, NumValue::Float(r)) => (f64::from(s) * r).into(),
        }
    }
}
impl Mul<&Rational> for &NumValue {
    type Output = NumValue;
    fn mul(self, rhs: &Rational) -> NumValue {
        match self {
            NumValue::Rational(s) => (s * rhs).into(),
            NumValue::BigRational(s) => (s.as_ref() * biggen(rhs)).into(),
            NumValue::Float(s) => {
                rhs.to_f64().map_or(f64::NAN, |r| s * r).into()
            }
        }
    }
}
impl Mul<i64> for NumValue {
    type Output = Self;
    fn mul(self, rhs: i64) -> Self {
        match self {
            s @ Self::Rational(_) => s * Self::from(rhs),
            Self::BigRational(s) => (s.as_ref() * BigInt::from(rhs)).into(),
            Self::Float(s) => (s * (rhs as f64)).into(),
        }
    }
}

impl Mul for Number {
    type Output = Self;
    fn mul(mut self, rhs: Self) -> Self {
        self.value = self.value * rhs.value;
        self
    }
}
impl Mul for &Number {
    type Output = Number;
    fn mul(self, rhs: Self) -> Self::Output {
        Number {
            value: &self.value * &rhs.value,
        }
    }
}

impl Rem for NumValue {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self {
        &self % &rhs
    }
}
impl Rem for &NumValue {
    type Output = NumValue;
    fn rem(self, rhs: Self) -> NumValue {
        if rhs.is_zero() {
            return (f64::from(self) % 0.).into();
        }
        fn float_rem(s: f64, r: f64) -> f64 {
            if r.is_infinite() && s.signum() * r.signum() < 0. {
                f64::NAN
            } else {
                s % r
            }
        }
        match (self, rhs) {
            (NumValue::Rational(s), NumValue::Rational(r)) => (s % r).into(),
            (NumValue::Rational(s), NumValue::BigRational(r)) => {
                (biggen(s) % r.as_ref()).into()
            }
            (NumValue::BigRational(s), NumValue::Rational(r)) => {
                (s.as_ref() % biggen(r)).into()
            }
            (NumValue::BigRational(s), NumValue::BigRational(r)) => {
                (s.as_ref() % r.as_ref()).into()
            }
            (NumValue::Float(s), r) => float_rem(*s, r.into()).into(),
            (s, NumValue::Float(r)) => float_rem(s.into(), *r).into(),
        }
    }
}
impl Div for NumValue {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        &self / &rhs
    }
}
impl Div for &NumValue {
    type Output = NumValue;
    fn div(self, rhs: Self) -> NumValue {
        if rhs.is_zero() {
            return (f64::from(self) / f64::from(rhs)).into();
        }
        match (self, rhs) {
            (NumValue::Rational(s), NumValue::Rational(r)) => s
                .checked_div(r)
                .map(Into::into)
                .unwrap_or_else(|| (biggen(s) / biggen(r)).into()),
            (NumValue::Rational(s), NumValue::BigRational(r)) => {
                (biggen(s) / r.as_ref()).into()
            }
            (NumValue::BigRational(s), NumValue::Rational(r)) => {
                (s.as_ref() / biggen(r)).into()
            }
            (NumValue::BigRational(s), NumValue::BigRational(r)) => {
                (s.as_ref() / r.as_ref()).into()
            }
            (NumValue::Float(s), r) => (s / f64::from(r)).into(),
            (s, NumValue::Float(r)) => (f64::from(s) / r).into(),
        }
    }
}

impl Div<i64> for NumValue {
    type Output = Self;
    fn div(self, rhs: i64) -> Self {
        match self {
            Self::Rational(s) => (s / rhs).into(),
            Self::BigRational(s) => (s.as_ref() / BigInt::from(rhs)).into(),
            Self::Float(s) => (s / (rhs as f64)).into(),
        }
    }
}

impl Add for NumValue {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::Rational(s), Self::Rational(r)) => s
                .checked_add(&r)
                .map(Into::into)
                .unwrap_or_else(|| (biggen(&s) + biggen(&r)).into()),
            (Self::Rational(s), Self::BigRational(r)) => {
                (biggen(&s) + r.as_ref()).into()
            }
            (Self::BigRational(s), Self::Rational(r)) => {
                (s.as_ref() + biggen(&r)).into()
            }
            (Self::BigRational(s), Self::BigRational(r)) => {
                (s.as_ref() + r.as_ref()).into()
            }
            (Self::Float(s), r) => (s + f64::from(r)).into(),
            (s, Self::Float(r)) => (f64::from(s) + r).into(),
        }
    }
}
impl Sub for NumValue {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        &self - &rhs
    }
}
impl Sub for &NumValue {
    type Output = NumValue;
    fn sub(self, rhs: Self) -> NumValue {
        match (self, rhs) {
            (NumValue::Rational(s), NumValue::Rational(r)) => s
                .checked_sub(r)
                .map(Into::into)
                .unwrap_or_else(|| (biggen(s) - biggen(r)).into()),
            (NumValue::Rational(s), NumValue::BigRational(r)) => {
                (biggen(s) - r.as_ref()).into()
            }
            (NumValue::BigRational(s), NumValue::Rational(r)) => {
                (s.as_ref() - biggen(r)).into()
            }
            (NumValue::BigRational(s), NumValue::BigRational(r)) => {
                (s.as_ref() - r.as_ref()).into()
            }
            (NumValue::Float(s), r) => (s - f64::from(r)).into(),
            (s, NumValue::Float(r)) => (f64::from(s) - r).into(),
        }
    }
}

fn biggen(val: &Rational) -> Ratio<BigInt> {
    Ratio::<BigInt>::new((*val.numer()).into(), (*val.denom()).into())
}

impl One for NumValue {
    fn one() -> Self {
        Self::Rational(One::one())
    }
}
impl One for Number {
    fn one() -> Self {
        Self {
            value: NumValue::one(),
        }
    }
}

impl Zero for NumValue {
    fn zero() -> Self {
        Self::Rational(Zero::zero())
    }
    fn is_zero(&self) -> bool {
        match self {
            Self::Rational(r) => r.is_zero(),
            Self::BigRational(r) => r.is_zero(),
            Self::Float(r) => r.is_zero(),
        }
    }
}

impl NumValue {
    pub fn ceil(&self) -> Self {
        match self {
            Self::Rational(r) => r.ceil().into(),
            Self::BigRational(r) => r.ceil().into(),
            Self::Float(r) => r.ceil().into(),
        }
    }
    pub fn floor(&self) -> Self {
        match self {
            Self::Rational(r) => r.floor().into(),
            Self::BigRational(r) => r.floor().into(),
            Self::Float(r) => r.floor().into(),
        }
    }
    pub fn trunc(&self) -> Self {
        match self {
            Self::Rational(r) => r.trunc().into(),
            Self::BigRational(r) => r.trunc().into(),
            Self::Float(r) => r.trunc().into(),
        }
    }
    pub fn round(&self) -> Self {
        match self {
            Self::Rational(r) => r.round().into(),
            Self::BigRational(r) => r.round().into(),
            Self::Float(r) => r.round().into(),
        }
    }
    pub fn signum(&self) -> Self {
        match self {
            Self::Rational(r) => r.signum().into(),
            Self::BigRational(r) => r.signum().into(),
            Self::Float(r) => {
                // The sass spec says sign(-0) is -0
                // ... which may be a bug, but here's to compatibility:
                if r.is_zero() {
                    (*r).into()
                } else {
                    r.signum().into()
                }
            }
        }
    }
    pub fn as_ratio(&self) -> Result<Rational, BadNumber> {
        match self {
            Self::Rational(r) => Ok(*r),
            Self::BigRational(r) => {
                let mut numer = r.numer().clone();
                let mut denom = r.denom().clone();
                loop {
                    let tn = i64::try_from(&numer);
                    let td = i64::try_from(&denom);
                    if let (Ok(n), Ok(d)) = (tn, td) {
                        return Ok(Ratio::new(n, d));
                    }
                    numer /= 32;
                    denom /= 32;
                    if denom.is_zero() {
                        return Err(BadNumber::TooLarge);
                    }
                }
            }
            Self::Float(r) => {
                Ratio::approximate_float(*r).ok_or(BadNumber::BadFloat(*r))
            }
        }
    }
}

impl Number {
    /// Create a rational number.
    pub fn rational(num: i64, denom: i64) -> Self {
        Rational::new(num, denom).into()
    }
    /// Get this number as a rational number.
    ///
    /// If the value is bignum rational or floating point, it is
    /// approximated as long as it is withing range, otherwises an
    /// error is returned.
    pub fn as_ratio(&self) -> Result<Rational, BadNumber> {
        self.value.as_ratio()
    }

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
        Self {
            value: self.value.signum(),
        }
    }
    /// Computes the absolute value of the number, retaining the flags.
    pub fn abs(&self) -> Self {
        match &self.value {
            NumValue::Rational(s) => s.abs().into(),
            NumValue::BigRational(s) => s.abs().into(),
            NumValue::Float(s) => s.abs().into(),
        }
    }
    /// Return true if this number is less than zero.
    pub fn is_negative(&self) -> bool {
        self.value.is_negative()
    }

    /// Return true if this value is neither inifinte nor NaN.
    pub fn is_finite(&self) -> bool {
        match self.value {
            NumValue::Rational(_) => true,
            NumValue::BigRational(_) => true,
            NumValue::Float(r) => r.is_finite(),
        }
    }

    /// Returns true if the number is an integer.
    pub fn into_integer(self) -> Result<i64, Self> {
        fn float_int(s: f64) -> Option<i64> {
            let sr = s.round();
            if (sr - s).abs() <= std::f32::EPSILON.into() {
                Some(sr as i64)
            } else {
                None
            }
        }
        match &self.value {
            NumValue::Rational(s) => {
                if s.is_integer() {
                    Ok(s.to_integer())
                } else {
                    s.to_f64().and_then(float_int).ok_or(self)
                }
            }
            NumValue::BigRational(s) => {
                if s.is_integer() {
                    i64::try_from(s.to_integer()).map_err(|_| self)
                } else {
                    s.to_f64().and_then(float_int).ok_or(self)
                }
            }
            NumValue::Float(s) => float_int(*s).ok_or(self),
        }
    }

    /// Converts to an integer, rounding towards zero.
    ///
    /// An integer that is too big to fit in an i64 returns `None`.
    pub fn to_integer(&self) -> Option<i64> {
        match &self.value {
            NumValue::Rational(s) => Some(s.to_integer()),
            NumValue::BigRational(s) => i64::try_from(s.to_integer()).ok(),
            NumValue::Float(s) => Some(s.ceil() as i64),
        }
    }
    /// Computes self^p.
    pub fn powi(self, p: i32) -> Self {
        match self.value {
            NumValue::Rational(s) => s.pow(p).into(),
            NumValue::BigRational(s) => s.pow(p).into(),
            NumValue::Float(s) => s.powi(p).into(),
        }
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
        Self {
            value: value.into(),
        }
    }
}
impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Self::from(i64::from(value))
    }
}
impl From<usize> for Number {
    fn from(value: usize) -> Self {
        match i64::try_from(value) {
            Ok(v) => v.into(),
            Err(_) => Ratio::from_integer(BigInt::from(value)).into(),
        }
    }
}
impl From<Rational> for Number {
    fn from(value: Rational) -> Self {
        Self {
            value: value.into(),
        }
    }
}
impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Self {
            value: value.into(),
        }
    }
}
impl From<Ratio<BigInt>> for Number {
    fn from(value: Ratio<BigInt>) -> Self {
        Self {
            value: value.into(),
        }
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
impl Mul<Rational> for Number {
    type Output = Self;
    fn mul(self, rhs: Rational) -> Self {
        Self {
            value: &self.value * &rhs,
        }
    }
}
impl Mul<i64> for Number {
    type Output = Self;
    fn mul(self, rhs: i64) -> Self {
        Self {
            value: self.value * rhs,
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
            value: self.value / rhs,
        }
    }
}

impl From<Number> for f64 {
    fn from(val: Number) -> Self {
        Self::from(val.value)
    }
}
impl From<NumValue> for f64 {
    fn from(val: NumValue) -> Self {
        Self::from(&val)
    }
}
impl From<&NumValue> for f64 {
    fn from(val: &NumValue) -> Self {
        match val {
            NumValue::Rational(s) => s.to_f64().unwrap_or(Self::NAN),
            NumValue::BigRational(s) => s.to_f64().unwrap_or(Self::NAN),
            NumValue::Float(s) => *s,
        }
    }
}

impl<'a> Div for &'a Number {
    type Output = Number;
    fn div(self, rhs: Self) -> Self::Output {
        Number {
            value: &self.value / &rhs.value,
        }
    }
}

impl Mul<&Rational> for &Number {
    type Output = Number;
    fn mul(self, rhs: &Rational) -> Self::Output {
        Number {
            value: &self.value * rhs,
        }
    }
}

impl Rem for &Number {
    type Output = Number;
    fn rem(self, rhs: Self) -> Self::Output {
        // The modulo operator in rust handles negative values different from
        // num-rational.
        let a = &self.value;
        let b = &rhs.value;
        let result = a % b;
        let result = if !a.is_zero() && (b.is_negative() != a.is_negative()) {
            result + b.clone()
        } else {
            result
        };
        Number { value: result }
    }
}

impl Sub for &Number {
    type Output = Number;
    fn sub(self, rhs: Self) -> Self::Output {
        Number {
            value: &self.value - &rhs.value,
        }
    }
}

impl Zero for Number {
    fn zero() -> Self {
        Self::from(0)
    }
    fn is_zero(&self) -> bool {
        self.value.is_zero()
    }
}

impl<'a> fmt::Display for Formatted<'a, Number> {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self.value.value {
            NumValue::Rational(ref s) => do_fmt(out, s, self.format),
            NumValue::BigRational(ref s) => do_fmt(out, s, self.format),
            NumValue::Float(ref s) => {
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
                    let mut dec = String::with_capacity(if frac.is_zero() {
                        0
                    } else {
                        self.format.precision
                    });

                    if !frac.is_zero() {
                        let max_decimals = 16 - whole.log10().ceil() as usize;
                        for _ in 1..max_decimals.min(self.format.precision) {
                            frac *= 10.;
                            write!(dec, "{}", (frac as i8).abs())?;
                            frac = frac.fract();
                            if frac.is_zero() {
                                break;
                            }
                        }
                        if !frac.is_zero() {
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
                                write!(dec, "{end}")?;
                            }
                        }
                    }

                    if s.is_sign_negative()
                        && (!whole.is_zero() || !dec.is_empty())
                    {
                        out.write_char('-')?;
                    }

                    let skip_zero = self.format.is_compressed();
                    if !(whole.is_zero() && skip_zero && !dec.is_empty()) {
                        write!(out, "{whole}")?;
                    }

                    if !dec.is_empty() {
                        write!(out, ".{dec}")?;
                    }
                    Ok(())
                }
            }
        }
    }
}

fn do_fmt<N>(
    out: &mut fmt::Formatter,
    value: &Ratio<N>,
    format: Format,
) -> fmt::Result
where
    N: Clone + fmt::Display + Integer + Signed + From<i8>,
{
    let ten: N = 10i8.into();
    let mut frac = value.fract();

    let mut whole = value.to_integer().abs();
    let mut dec = String::with_capacity(if frac.is_zero() {
        0
    } else {
        format.precision
    });

    if !frac.is_zero() {
        for _ in 0..(format.precision - 1) {
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
                write!(dec, "{end}")?;
            }
        }
    }

    if value.is_negative() && (!whole.is_zero() || !dec.is_empty()) {
        out.write_char('-')?;
    }

    let skip_zero = format.is_compressed();
    if !(whole.is_zero() && skip_zero && !dec.is_empty()) {
        write!(out, "{whole}")?;
    }

    if !dec.is_empty() {
        write!(out, ".{dec}")?;
    }
    Ok(())
}

impl fmt::Debug for Number {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        out.write_str("Number ")?;
        match &self.value {
            NumValue::Rational(r) => {
                r.numer().fmt(out)?;
                out.write_str(" / ")?;
                r.denom().fmt(out)
            }
            NumValue::BigRational(r) => {
                out.debug_list().entry(r.numer()).entry(r.denom()).finish()
            }
            NumValue::Float(f) => f.fmt(out),
        }
    }
}

/// Error signifying that a number could not be approximated as
/// a rational.
pub enum BadNumber {
    /// The number was to large.
    TooLarge,
    /// The number was a "special" float value.
    BadFloat(f64),
}

impl From<BadNumber> for String {
    fn from(n: BadNumber) -> Self {
        n.to_string()
    }
}

impl fmt::Display for BadNumber {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::TooLarge => out.write_str("Number too large"),
            Self::BadFloat(f) => write!(out, "Bad float: {f}"),
        }
    }
}

#[test]
fn debug_integer() {
    assert_eq!(format!("{:?}", Number::from(17)), "Number 17 / 1",);
}
#[test]
fn debug_long_integer() {
    assert_eq!(format!("{:?}", Number::from(17)), "Number 17 / 1",);
}

#[test]
fn debug_biginteger() {
    assert_eq!(
        format!(
            "{:?}",
            Number::from(Ratio::<BigInt>::new(17.into(), 1.into()))
        ),
        "Number [17, 1]",
    );
}

#[test]
fn debug_float() {
    assert_eq!(format!("{:?}", Number::from(17.5)), "Number 17.5",);
}

#[test]
fn debug_int_value() {
    assert_eq!(
        format!("{:#?}", crate::sass::Value::scalar(17)),
        "Numeric(\
         \n    Number 17 / 1; UnitSet [],\
         \n)",
    );
}
