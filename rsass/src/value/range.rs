use super::{Numeric, UnitSet};
use crate::css::Value;

pub struct ValueRange {
    from: i64,
    to: i64,
    step: i64,
    unit: UnitSet,
}

impl ValueRange {
    pub fn new(from: i64, to: i64, inclusive: bool, unit: UnitSet) -> Self {
        let step = if to >= from { 1 } else { -1 };
        let to = if inclusive { to + step } else { to };
        Self {
            from,
            to,
            step,
            unit,
        }
    }
}

impl Iterator for ValueRange {
    type Item = Value;
    fn next(&mut self) -> Option<Value> {
        if self.from.partial_cmp(&self.to) == 0.partial_cmp(&self.step) {
            let result = Numeric::new(self.from, self.unit.clone()).into();
            self.from += self.step;
            Some(result)
        } else {
            None
        }
    }
}
