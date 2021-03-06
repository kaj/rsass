use crate::output::{Format, Formatted};
use num_bigint::BigInt;
use num_integer::Integer;
use num_rational::{Ratio, Rational};
use num_traits::{One, Signed, Zero};
use std::cmp::Ordering;
use std::convert::TryFrom;
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
    Rational(Ratio<isize>),
    BigRational(Ratio<BigInt>),
    Float(f64),
}

impl From<Ratio<isize>> for NumValue {
    fn from(value: Ratio<isize>) -> NumValue {
        NumValue::Rational(value)
    }
}
impl From<isize> for NumValue {
    fn from(value: isize) -> NumValue {
        NumValue::Rational(value.into())
    }
}
impl From<Ratio<BigInt>> for NumValue {
    fn from(value: Ratio<BigInt>) -> NumValue {
        NumValue::BigRational(value)
    }
}
impl From<f64> for NumValue {
    fn from(value: f64) -> NumValue {
        NumValue::Float(value)
    }
}

impl NumValue {
    pub fn is_negative(&self) -> bool {
        match self {
            NumValue::Rational(s) => s.is_negative(),
            NumValue::BigRational(s) => s.is_negative(),
            NumValue::Float(s) => s.is_sign_negative(),
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
            NumValue::BigRational(s) => (-s).into(),
            NumValue::Float(s) => (-s).into(),
        }
    }
}

impl Eq for NumValue {}
impl PartialEq for NumValue {
    fn eq(&self, rhs: &NumValue) -> bool {
        self.partial_cmp(rhs) == Some(Ordering::Equal)
    }
}
impl PartialOrd for NumValue {
    fn partial_cmp(&self, rhs: &NumValue) -> Option<Ordering> {
        match (self, rhs) {
            (NumValue::Rational(s), NumValue::Rational(r)) => {
                s.partial_cmp(r)
            }
            (NumValue::Rational(s), NumValue::BigRational(r)) => {
                biggen(s).partial_cmp(r)
            }
            (NumValue::BigRational(s), NumValue::Rational(r)) => {
                s.partial_cmp(&biggen(r))
            }
            (NumValue::BigRational(s), NumValue::BigRational(r)) => {
                s.partial_cmp(r)
            }
            (NumValue::Float(s), r) => f64_cmp(s, &r.into()),
            (s, NumValue::Float(r)) => f64_cmp(&s.into(), r),
        }
    }
}

fn f64_cmp(a: &f64, b: &f64) -> Option<Ordering> {
    match a.partial_cmp(b) {
        // Rust considers inf == inf, and sass doesn't
        Some(Ordering::Equal) if !a.is_normal() || !b.is_normal() => None,
        result => result,
    }
}

impl Mul for NumValue {
    type Output = NumValue;
    fn mul(self, rhs: Self) -> Self {
        &self * &rhs
    }
}
impl Mul for &NumValue {
    type Output = NumValue;
    fn mul(self, rhs: Self) -> NumValue {
        match (self, rhs) {
            (NumValue::Rational(s), NumValue::Rational(r)) => (s * r).into(),
            (NumValue::Rational(s), NumValue::BigRational(r)) => {
                (biggen(s) * r).into()
            }
            (NumValue::BigRational(s), NumValue::Rational(r)) => {
                (s * biggen(r)).into()
            }
            (NumValue::BigRational(s), NumValue::BigRational(r)) => {
                (s * r).into()
            }
            (NumValue::Float(s), r) => (s * f64::from(r)).into(),
            (s, NumValue::Float(r)) => (f64::from(s) * r).into(),
        }
    }
}
impl Mul<&Ratio<isize>> for &NumValue {
    type Output = NumValue;
    fn mul(self, rhs: &Ratio<isize>) -> NumValue {
        match self {
            NumValue::Rational(s) => (s * rhs).into(),
            NumValue::BigRational(s) => (s * biggen(&rhs)).into(),
            NumValue::Float(s) => (s * ratio_to_float(rhs)).into(),
        }
    }
}
impl Mul<isize> for NumValue {
    type Output = Self;
    fn mul(self, rhs: isize) -> Self {
        match self {
            NumValue::Rational(s) => (s * rhs).into(),
            NumValue::BigRational(s) => (s * BigInt::from(rhs)).into(),
            NumValue::Float(s) => (s * (rhs as f64)).into(),
        }
    }
}

impl Mul for Number {
    type Output = Number;
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
    type Output = NumValue;
    fn rem(self, rhs: Self) -> NumValue {
        &self % &rhs
    }
}
impl Rem for &NumValue {
    type Output = NumValue;
    fn rem(self, rhs: Self) -> NumValue {
        if rhs.is_zero() {
            return (f64::from(self) % 0.).into();
        }
        match (self, rhs) {
            (NumValue::Rational(s), NumValue::Rational(r)) => (s % r).into(),
            (NumValue::Rational(s), NumValue::BigRational(r)) => {
                (biggen(&s) % r).into()
            }
            (NumValue::BigRational(s), NumValue::Rational(r)) => {
                (s % biggen(&r)).into()
            }
            (NumValue::BigRational(s), NumValue::BigRational(r)) => {
                (s % r).into()
            }
            (NumValue::Float(s), r) => (s % f64::from(r)).into(),
            (s, NumValue::Float(r)) => (f64::from(s) % r).into(),
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
            return (f64::from(self) / 0.).into();
        }
        match (self, rhs) {
            (NumValue::Rational(s), NumValue::Rational(r)) => (s / r).into(),
            (NumValue::Rational(s), NumValue::BigRational(r)) => {
                (biggen(&s) / r).into()
            }
            (NumValue::BigRational(s), NumValue::Rational(r)) => {
                (s / biggen(&r)).into()
            }
            (NumValue::BigRational(s), NumValue::BigRational(r)) => {
                (s / r).into()
            }
            (NumValue::Float(s), r) => (s / f64::from(r)).into(),
            (s, NumValue::Float(r)) => (f64::from(s) / r).into(),
        }
    }
}

impl Div<isize> for NumValue {
    type Output = Self;
    fn div(self, rhs: isize) -> Self {
        match self {
            NumValue::Rational(s) => (s / rhs).into(),
            NumValue::BigRational(s) => (s / BigInt::from(rhs)).into(),
            NumValue::Float(s) => (s / (rhs as f64)).into(),
        }
    }
}

impl Add for NumValue {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        match (self, rhs) {
            (NumValue::Rational(s), NumValue::Rational(r)) => (s + r).into(),
            (NumValue::Rational(s), NumValue::BigRational(r)) => {
                (biggen(&s) + r).into()
            }
            (NumValue::BigRational(s), NumValue::Rational(r)) => {
                (s + biggen(&r)).into()
            }
            (NumValue::BigRational(s), NumValue::BigRational(r)) => {
                (s + r).into()
            }
            (NumValue::Float(s), r) => (s + f64::from(r)).into(),
            (s, NumValue::Float(r)) => (f64::from(s) + r).into(),
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
            (NumValue::Rational(s), NumValue::Rational(r)) => (s - r).into(),
            (NumValue::Rational(s), NumValue::BigRational(r)) => {
                (biggen(&s) - r).into()
            }
            (NumValue::BigRational(s), NumValue::Rational(r)) => {
                (s - biggen(&r)).into()
            }
            (NumValue::BigRational(s), NumValue::BigRational(r)) => {
                (s - r).into()
            }
            (NumValue::Float(s), r) => (s + f64::from(r)).into(),
            (s, NumValue::Float(r)) => (f64::from(s) + r).into(),
        }
    }
}

fn biggen(val: &Ratio<isize>) -> Ratio<BigInt> {
    Ratio::<BigInt>::new((*val.numer()).into(), (*val.denom()).into())
}

impl One for NumValue {
    fn one() -> NumValue {
        NumValue::Rational(One::one())
    }
}
impl One for Number {
    fn one() -> Number {
        Number {
            value: NumValue::one(),
        }
    }
}

impl Zero for NumValue {
    fn zero() -> NumValue {
        NumValue::Rational(Zero::zero())
    }
    fn is_zero(&self) -> bool {
        match self {
            NumValue::Rational(r) => r.is_zero(),
            NumValue::BigRational(r) => r.is_zero(),
            NumValue::Float(r) => r.is_zero(),
        }
    }
}

impl NumValue {
    pub fn ceil(&self) -> NumValue {
        match self {
            NumValue::Rational(r) => r.ceil().into(),
            NumValue::BigRational(r) => r.ceil().into(),
            NumValue::Float(r) => r.ceil().into(),
        }
    }
    pub fn floor(&self) -> NumValue {
        match self {
            NumValue::Rational(r) => r.floor().into(),
            NumValue::BigRational(r) => r.floor().into(),
            NumValue::Float(r) => r.floor().into(),
        }
    }
    pub fn round(&self) -> NumValue {
        match self {
            NumValue::Rational(r) => r.round().into(),
            NumValue::BigRational(r) => r.round().into(),
            NumValue::Float(r) => r.round().into(),
        }
    }
    pub fn as_ratio(&self) -> Result<Rational, crate::Error> {
        match self {
            NumValue::Rational(r) => Ok(*r),
            NumValue::BigRational(r) => {
                let mut numer = r.numer().clone();
                let mut denom = r.denom().clone();
                loop {
                    let tn = isize::try_from(&numer);
                    let td = isize::try_from(&denom);
                    if let (Ok(n), Ok(d)) = (tn, td) {
                        return Ok(Ratio::new(n, d));
                    }
                    numer /= 32;
                    denom /= 32;
                    if denom.is_zero() {
                        return Err(crate::Error::BadValue(
                            "Number too large".into(),
                        ));
                    }
                }
            }
            NumValue::Float(r) => Ratio::approximate_float(*r)
                .ok_or_else(|| crate::Error::BadValue(r.to_string())),
        }
    }
}

impl Number {
    /// Create a rational number.
    pub fn rational(num: isize, denom: isize) -> Self {
        Rational::new(num, denom).into()
    }
    /// Get this number as a rational number.
    ///
    /// If the value is bignum rational or floating point, it is
    /// approximated as long as it is withing range, otherwises an
    /// error is returned.
    pub fn as_ratio(&self) -> Result<Rational, crate::Error> {
        self.value.as_ratio()
    }

    /// Get a copy of this number, rounded away from zero.
    pub fn ceil(&self) -> Self {
        Number {
            value: self.value.ceil(),
        }
    }
    /// Get a copy of this number, rounded towards zero.
    pub fn floor(&self) -> Self {
        Number {
            value: self.value.floor(),
        }
    }
    /// Get a copy of this number, rounded to nearest integer.
    pub fn round(&self) -> Self {
        Number {
            value: self.value.round(),
        }
    }
    /// Computes the absolute value of the number, retaining the flags.
    pub fn abs(self) -> Self {
        match self.value {
            NumValue::Rational(s) => s.abs().into(),
            NumValue::BigRational(s) => s.abs().into(),
            NumValue::Float(s) => s.abs().into(),
        }
    }

    /// Returns true if the number is an integer.
    pub fn is_integer(&self) -> bool {
        match &self.value {
            NumValue::Rational(s) => s.is_integer(),
            NumValue::BigRational(s) => s.is_integer(),
            NumValue::Float(s) => {
                // TODO: A bigger epsilon might be needed?
                (s.round() - s).abs() <= std::f64::EPSILON
            }
        }
    }

    /// Converts to an integer, rounding towards zero.
    ///
    /// An integer that is too big to fit in an isize returns `None`.
    pub fn to_integer(&self) -> Option<isize> {
        match &self.value {
            NumValue::Rational(s) => Some(s.to_integer()),
            NumValue::BigRational(s) => isize::try_from(s.to_integer()).ok(),
            NumValue::Float(s) => Some(s.ceil() as isize),
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

impl From<isize> for Number {
    fn from(value: isize) -> Number {
        Number {
            value: value.into(),
        }
    }
}
impl From<i32> for Number {
    fn from(value: i32) -> Number {
        // isize is be at least 32 bits.
        (value as isize).into()
    }
}
impl From<usize> for Number {
    fn from(value: usize) -> Number {
        match isize::try_from(value) {
            Ok(v) => v.into(),
            Err(_) => Ratio::from_integer(BigInt::from(value)).into(),
        }
    }
}
impl From<Rational> for Number {
    fn from(value: Rational) -> Number {
        Number {
            value: value.into(),
        }
    }
}
impl From<f64> for Number {
    fn from(value: f64) -> Number {
        Number {
            value: value.into(),
        }
    }
}
impl From<Ratio<BigInt>> for Number {
    fn from(value: Ratio<BigInt>) -> Number {
        Number {
            value: value.into(),
        }
    }
}
impl Add for Number {
    type Output = Number;
    fn add(self, rhs: Self) -> Self::Output {
        Number {
            value: self.value + rhs.value,
        }
    }
}
impl Mul<Ratio<isize>> for Number {
    type Output = Self;
    fn mul(self, rhs: Ratio<isize>) -> Self {
        Number {
            value: &self.value * &rhs,
        }
    }
}
impl Mul<isize> for Number {
    type Output = Self;
    fn mul(self, rhs: isize) -> Self {
        Number {
            value: self.value * rhs,
        }
    }
}
impl Div for Number {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Number {
            value: self.value / rhs.value,
        }
    }
}
impl Div<isize> for Number {
    type Output = Self;
    fn div(self, rhs: isize) -> Self {
        Number {
            value: self.value / rhs,
        }
    }
}

impl From<Number> for f64 {
    fn from(val: Number) -> f64 {
        f64::from(val.value)
    }
}
impl From<NumValue> for f64 {
    fn from(val: NumValue) -> f64 {
        f64::from(&val)
    }
}
impl From<&NumValue> for f64 {
    fn from(val: &NumValue) -> f64 {
        match val {
            NumValue::Rational(s) => ratio_to_float(s),
            NumValue::BigRational(s) => ratio_to_float(s),
            NumValue::Float(s) => *s,
        }
    }
}

use std::convert::TryInto;
fn ratio_to_float<
    T: TryInto<i32> + Div<isize, Output = T> + Signed + Clone,
>(
    val: &Ratio<T>,
) -> f64 {
    let numer: T = val.numer().clone();
    let sign = f64::from(numer.signum().try_into().unwrap_or(1));
    let mut numer = numer.abs();
    let mut denom: T = val.denom().clone();
    loop {
        let tn = numer.clone().try_into();
        let td = denom.clone().try_into();
        if let (Ok(n), Ok(d)) = (tn, td) {
            return sign * (f64::from(n) / f64::from(d));
        }
        numer = numer / 32;
        denom = denom / 32;
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

impl Mul<&Ratio<isize>> for &Number {
    type Output = Number;
    fn mul(self, rhs: &Ratio<isize>) -> Self::Output {
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
        Number::from(0)
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
                        "{}Infinity",
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
                        for _ in 0..(self.format.precision - 1) {
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
                                write!(dec, "{}", end)?;
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
                        write!(out, "{}", whole)?;
                    }

                    if !dec.is_empty() {
                        write!(out, ".{}", dec)?;
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
                write!(dec, "{}", end)?;
            }
        }
    }

    if value.is_negative() && (!whole.is_zero() || !dec.is_empty()) {
        out.write_char('-')?;
    }

    let skip_zero = format.is_compressed();
    if !(whole.is_zero() && skip_zero && !dec.is_empty()) {
        write!(out, "{}", whole)?;
    }

    if !dec.is_empty() {
        write!(out, ".{}", dec)?;
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
        format!("{:?}", Number::from(Ratio::<BigInt>::new(17.into(), 1.into()))),
        "Number [BigInt { sign: Plus, data: BigUint { data: [17] } }, BigInt { sign: Plus, data: BigUint { data: [1] } }]",
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
         \n    Numeric {\
         \n        value: Number 17 / 1,\
         \n        unit: UnitSet [],\
         \n    },\
         \n)",
    );
}
