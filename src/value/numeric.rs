use super::{Number, Rational, Unit};
use crate::output::{Format, Formatted};
use crate::Error;
use std::fmt::{self, Display};
use std::ops::Neg;

/// A Numeric value is a [`Number`] with a [`Unit`] (which may be
/// Unit::None).
#[derive(Clone, Debug, Eq)]
pub struct Numeric {
    /// The number value of this numeric.
    pub value: Number,
    /// The unit of this numeric.
    pub unit: Unit,
}

impl Numeric {
    /// Create a new numeric value.
    ///
    /// The value can be given as anything that can be converted into
    /// a [`Number`], e.g. an [`isize`], a [`Rational`], or a [`f64`].
    pub fn new(value: impl Into<Number>, unit: Unit) -> Numeric {
        Numeric {
            value: value.into(),
            unit,
        }
    }
    /// Create a new numeric value with no unit.
    pub fn scalar(value: impl Into<Number>) -> Numeric {
        Numeric {
            value: value.into(),
            unit: Unit::None,
        }
    }

    /// Convert this numeric value to a given unit, if possible.
    ///
    /// # Examples
    /// ```
    /// # use rsass::value::{Numeric, Unit};
    /// let inch = Numeric::new(1, Unit::In);
    /// assert_eq!(inch.as_unit(Unit::Mm).unwrap() * 5, 127.into());
    /// assert_eq!(inch.as_unit(Unit::Deg), None);
    /// ```
    pub fn as_unit(&self, unit: Unit) -> Option<Number> {
        self.unit.scale_to(&unit).map(|scale| &self.value * &scale)
    }

    /// Convert this numeric value to a given unit, if possible.
    ///
    /// Like [as_unit], except a unitless numeric is considered to be
    /// the expected unit.
    pub fn as_unit_def(&self, unit: Unit) -> Option<Number> {
        if self.is_no_unit() {
            Some(self.value.clone())
        } else {
            self.as_unit(unit)
        }
    }

    /// Get this number as a rational number.
    ///
    /// The unit is ignored.  If the value is bignum rational or
    /// floating point, it is approximated as long as it is withing
    /// range, otherwises an error is returned.
    pub fn as_ratio(&self) -> Result<Rational, Error> {
        self.value.as_ratio()
    }

    /// Return true if this value has no unit.
    pub fn is_no_unit(&self) -> bool {
        self.unit == Unit::None
    }

    /// Get a reference to this `Value` bound to an output format.
    pub fn format(&self, format: Format) -> Formatted<Numeric> {
        Formatted {
            value: self,
            format,
        }
    }
}

impl PartialEq for Numeric {
    fn eq(&self, other: &Numeric) -> bool {
        self.partial_cmp(other) == Some(std::cmp::Ordering::Equal)
    }
}

impl PartialOrd for Numeric {
    fn partial_cmp(&self, other: &Numeric) -> Option<std::cmp::Ordering> {
        if self.unit == other.unit {
            self.value.partial_cmp(&other.value)
        } else if let Some(scaled) = other.as_unit(self.unit.clone()) {
            self.value.partial_cmp(&scaled)
        } else {
            None
        }
    }
}

impl Neg for &Numeric {
    type Output = Numeric;
    fn neg(self) -> Self::Output {
        Numeric {
            value: -&self.value,
            unit: self.unit.clone(),
        }
    }
}

impl<'a> Display for Formatted<'a, Numeric> {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        self.value.value.format(self.format).fmt(out)?;
        self.value.unit.fmt(out)
    }
}
