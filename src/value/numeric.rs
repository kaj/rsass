use super::{Number, Rational, Unit};
use crate::output::{Format, Formatted};
use crate::Error;
use std::fmt::{self, Display};
use std::ops::Neg;

/// A Numeric value is a rational value with a Unit (which may be
/// Unit::None) and flags.
#[derive(Clone, Debug, Eq)]
pub struct Numeric {
    pub value: Number,
    pub unit: Unit,
}

impl Numeric {
    pub fn new(value: impl Into<Number>, unit: Unit) -> Numeric {
        Numeric {
            value: value.into(),
            unit,
        }
    }

    /// Get a reference to this `Value` bound to an output format.
    pub fn format(&self, format: Format) -> Formatted<Numeric> {
        Formatted {
            value: self,
            format,
        }
    }
    pub fn as_unit(&self, unit: Unit) -> Option<Number> {
        self.unit.scale_to(&unit).map(|scale| &self.value * &scale)
    }
    pub fn as_ratio(&self) -> Result<Rational, Error> {
        self.value.as_ratio()
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
