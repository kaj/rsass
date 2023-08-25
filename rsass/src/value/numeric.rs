use super::{BadNumber, Number, Rational, Unit, UnitSet};
use crate::output::{Format, Formatted};
use std::fmt::{self, Display};
use std::ops::{Div, Mul, Neg};

/// A Numeric value is a [`Number`] with a [`Unit`] (which may be
/// `Unit::None`).
#[derive(Clone, Eq)]
pub struct Numeric {
    /// The number value of this numeric.
    pub value: Number,
    /// The unit of this numeric.
    pub unit: UnitSet,
}

impl fmt::Debug for Numeric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}; {:?}", self.value, self.unit)
    }
}

impl Numeric {
    /// Create a new numeric value.
    ///
    /// The value can be given as anything that can be converted into
    /// a [`Number`], e.g. an [`isize`], a [`Rational`], or a [`f64`].
    pub fn new<V: Into<Number>, U: Into<UnitSet>>(value: V, unit: U) -> Self {
        Numeric {
            value: value.into(),
            unit: unit.into(),
        }
    }
    /// Create a new numeric value with no unit.
    pub fn scalar(value: impl Into<Number>) -> Numeric {
        Numeric {
            value: value.into(),
            unit: UnitSet::scalar(),
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
        self.unit
            .scale_to_unit(&unit)
            .map(|scale| &self.value * &scale)
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
    pub fn as_unitset(&self, unit: &UnitSet) -> Option<Number> {
        self.unit.scale_to(unit).map(|scale| &self.value * &scale)
    }

    /// Convert this numeric value to a given unit, if possible.
    ///
    /// Like [`as_unit`](Self::as_unit), except a unitless numeric is
    /// considered to be the expected unit.
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
    pub fn as_ratio(&self) -> Result<Rational, BadNumber> {
        self.value.as_ratio()
    }

    /// Return true if this value has no unit.
    pub fn is_no_unit(&self) -> bool {
        self.unit.is_none()
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
        } else if self.is_no_unit() || other.is_no_unit() {
            match self.value.partial_cmp(&other.value) {
                Some(std::cmp::Ordering::Equal) => None,
                other => other,
            }
        } else if let Some(scaled) = other.as_unitset(&self.unit) {
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

impl Div for &Numeric {
    type Output = Numeric;
    fn div(self, rhs: Self) -> Self::Output {
        let value = &self.value / &rhs.value;
        let mut unit = &self.unit / &rhs.unit;
        let scale = unit.simplify();
        Numeric {
            value: value * scale,
            unit,
        }
    }
}
impl Mul for &Numeric {
    type Output = Numeric;
    fn mul(self, rhs: Self) -> Self::Output {
        let value = &self.value * &rhs.value;
        let mut unit = &self.unit * &rhs.unit;
        let scale = unit.simplify();
        Numeric {
            value: value * scale,
            unit,
        }
    }
}

impl<'a> Display for Formatted<'a, Numeric> {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        let mut unit = self.value.unit.clone();
        let value = &self.value.value * &unit.simplify();
        value.format(self.format).fmt(out)?;
        if !(unit.is_none() || value.is_finite()) {
            out.write_str(" * 1")?;
        }
        unit.fmt(out)
    }
}
