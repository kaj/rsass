use super::{Dimension, Number, Unit};
use num_traits::one;
use std::fmt::{self, Display};
use std::ops::{Div, Mul};

/// A set of units.
///
/// Eg. `10em * 10em` is an area of 100em².
/// Css does not support such arbitrary units, but sass does, and if
/// you divide it by a length you get a valid css length.
#[derive(Clone, PartialEq, Eq)]
pub struct UnitSet {
    units: Vec<(Unit, i8)>,
}

impl UnitSet {
    /// An empty UnitSet for a scalar value.
    pub fn scalar() -> Self {
        UnitSet { units: vec![] }
    }

    /// Check if this UnitSet is empty.
    pub fn is_none(&self) -> bool {
        self.units.iter().all(|(u, _)| *u == Unit::None)
    }
    /// Check if this UnitSet is the percent unit.
    pub fn is_percent(&self) -> bool {
        self.units == [(Unit::Percent, 1)]
    }

    /// Check if this UnitSet is compatible with another UnitSet.
    pub fn is_compatible(&self, other: &Self) -> bool {
        self.is_none()
            || other.is_none()
            || self.dimension() == other.dimension()
    }

    fn dimension(&self) -> Vec<(Dimension, i8)> {
        use std::collections::BTreeMap;
        self.units
            .iter()
            .fold(BTreeMap::new(), |mut map, (unit, power)| {
                *map.entry(unit.dimension()).or_insert(0) += *power;
                map
            })
            .into_iter()
            .filter(|(_d, power)| *power != 0)
            .collect::<Vec<_>>()
    }

    /// Get a scaling factor to convert this unit to another unit.
    ///
    /// Returns None if the units are of different dimension.
    pub fn scale_to(&self, other: &UnitSet) -> Option<Number> {
        if let [(u, 1)] = other.units.as_slice() {
            self.scale_to_unit(u)
        } else if other.is_none() {
            self.scale_to_unit(&Unit::None)
        } else {
            let quote = self / other;
            if quote.dimension().is_empty() {
                Some(quote.units.iter().fold(one(), |a, (unit, power)| {
                    a * unit.scale_factor().powi((*power).into())
                }))
            } else {
                // self and other differs in dimension.
                None
            }
        }
    }
    /// Get a scaling factor to convert this unit to another unit.
    ///
    /// Returns None if the units are of different dimension.
    pub fn scale_to_unit(&self, other: &Unit) -> Option<Number> {
        if let [(u, 1)] = self.units.as_slice() {
            u.scale_to(other)
        } else if self.is_none() {
            Unit::None.scale_to(other)
        } else {
            // TODO: Handle e.g. em*% as em.
            None
        }
    }

    /// Simplify this unit set, returning a scaling factor.
    pub fn simplify(&mut self) -> Number {
        let mut factor = one();
        if self.units.len() > 1 {
            for i in 1..(self.units.len()) {
                let (a, b) = self.units.split_at_mut(i);
                let (au, ap) = a.last_mut().unwrap();
                for (bu, bp) in b {
                    if let Some(f) = bu.scale_to(au) {
                        if ap.abs() > bp.abs() {
                            factor = factor * f.powi((*bp).into());
                            *ap += *bp;
                            *bp = 0;
                        } else {
                            factor = factor / f.powi((*ap).into());
                            *bp += *ap;
                            *ap = 0;
                        }
                    }
                }
            }
        }
        self.units.retain(|(_u, p)| *p != 0);
        factor
    }
}

impl Div for &UnitSet {
    type Output = UnitSet;
    fn div(self, rhs: Self) -> Self::Output {
        let mut result = self.clone();
        'rhs: for (ru, rp) in &rhs.units {
            for (lu, lp) in &mut result.units {
                if lu == ru {
                    *lp -= rp;
                    continue 'rhs;
                }
            }
            result.units.push((ru.clone(), -rp));
        }
        result.units.retain(|(_u, p)| *p != 0);
        result
    }
}
impl Mul for &UnitSet {
    type Output = UnitSet;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = self.clone();
        'rhs: for (ru, rp) in &rhs.units {
            for (lu, lp) in &mut result.units {
                if lu == ru {
                    *lp += rp;
                    continue 'rhs;
                }
            }
            result.units.push((ru.clone(), *rp));
        }
        result.units.retain(|(_u, p)| *p != 0);
        result
    }
}

impl From<Unit> for UnitSet {
    fn from(unit: Unit) -> Self {
        UnitSet {
            units: if unit == Unit::None {
                vec![]
            } else {
                vec![(unit, 1)]
            },
        }
    }
}

impl Display for UnitSet {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        let pos: Vec<_> =
            self.units.iter().filter(|(_u, p)| *p > 0).collect();
        let neg: Vec<_> =
            self.units.iter().filter(|(_u, p)| *p < 0).collect();

        if let Some(((u, p), rest)) = pos.split_first() {
            write_one(out, u, *p)?;
            for (u, p) in rest {
                out.write_str("*")?;
                write_one(out, u, p.abs())?;
            }
            if let Some(((u, p), rest)) = neg.split_first() {
                out.write_str("/")?;
                write_one(out, u, p.abs())?;
                for (u, p) in rest {
                    out.write_str("*")?;
                    write_one(out, u, p.abs())?;
                }
            }
        } else {
            match neg.split_first() {
                None => (),
                Some(((u, p), [])) => {
                    write_one(out, u, *p)?;
                }
                Some(((u, p), rest)) => {
                    out.write_str("(")?;
                    write_one(out, u, p.abs())?;
                    for (u, p) in rest {
                        out.write_str("*")?;
                        write_one(out, u, p.abs())?;
                    }
                    out.write_str(")^-1")?;
                }
            }
        }
        Ok(())
    }
}

fn write_one(out: &mut fmt::Formatter, u: &Unit, p: i8) -> fmt::Result {
    u.fmt(out)?;
    if p != 1 {
        write!(out, "^{}", p)?;
    }
    Ok(())
}

impl fmt::Debug for UnitSet {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        out.write_str("UnitSet ")?;
        out.debug_list().entries(&self.units).finish()
    }
}
