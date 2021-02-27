use super::{Number, Numeric, UnitSet};
use crate::css::Value;
use std::fmt;

pub struct ValueRange {
    from: Number,
    to: Number,
    step: Number,
    unit: UnitSet,
}

impl ValueRange {
    pub fn new(
        from: Value,
        to: Value,
        inclusive: bool,
    ) -> Result<ValueRange, RangeError> {
        let from =
            from.numeric_value().map_err(RangeError::FromNotNumeric)?;
        let to = to.numeric_value().map_err(RangeError::ToNotNumeric)?;

        let to = if from.is_no_unit() || to.is_no_unit() {
            to.value
        } else if let Some(scaled) = to.as_unitset(&from.unit) {
            scaled
        } else {
            return Err(RangeError::IncompatibleUnits(from.unit, to.unit));
        };
        let step = if to >= from.value {
            Number::from(1)
        } else {
            Number::from(-1)
        };
        let to = if inclusive { to + step.clone() } else { to };
        Ok(ValueRange {
            from: from.value,
            to,
            step,
            unit: from.unit,
        })
    }
}

impl Iterator for ValueRange {
    type Item = Value;
    fn next(&mut self) -> Option<Value> {
        if self.from.partial_cmp(&self.to)
            == Number::from(0).partial_cmp(&self.step)
        {
            let result =
                Numeric::new(self.from.clone(), self.unit.clone()).into();
            self.from = self.from.clone() + self.step.clone();
            Some(result)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub enum RangeError {
    FromNotNumeric(Value),
    ToNotNumeric(Value),
    IncompatibleUnits(UnitSet, UnitSet),
}

impl fmt::Display for RangeError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RangeError::FromNotNumeric(v) => {
                write!(
                    out,
                    "Bad Range: from needs to be numeric, got {}",
                    v.format(Default::default()),
                )
            }
            RangeError::ToNotNumeric(v) => {
                write!(
                    out,
                    "Bad Range: to needs to be numeric, got {}",
                    v.format(Default::default())
                )
            }
            RangeError::IncompatibleUnits(a, b) => {
                write!(
                    out,
                    "for loop needs compatible units, got {}..{}",
                    a, b,
                )
            }
        }
    }
}
