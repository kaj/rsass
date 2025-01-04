use super::{CssDimension, Dimension, Unit};
use std::fmt::{self, Display, Write};
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
    /// An empty `UnitSet` for a scalar value.
    pub fn scalar() -> Self {
        Self { units: vec![] }
    }

    /// Check if this `UnitSet` is empty.
    pub fn is_none(&self) -> bool {
        self.units.iter().all(|(u, _)| *u == Unit::None)
    }
    pub(crate) fn is_pos(&self) -> bool {
        self.units.iter().any(|(u, p)| *u != Unit::None && *p > 0)
    }
    /// Check if this `UnitSet` conains only known units.
    pub fn is_known(&self) -> bool {
        !self
            .units
            .iter()
            .any(|(u, _)| matches!(u, Unit::Unknown(_)))
    }
    /// Check if this `UnitSet` is the percent unit.
    pub fn is_percent(&self) -> bool {
        self.units == [(Unit::Percent, 1)]
    }

    /// Check if this `UnitSet` is compatible with another `UnitSet`.
    pub fn is_compatible(&self, other: &Self) -> bool {
        self.is_none()
            || other.is_none()
            || self.dimension() == other.dimension()
    }

    pub(crate) fn dimension(&self) -> Vec<(Dimension, i8)> {
        use std::collections::BTreeMap;
        self.units
            .iter()
            .map(|(unit, p)| (unit.dimension(), p))
            .filter(|(dim, _p)| *dim != Dimension::None)
            .fold(BTreeMap::new(), |mut map, (dim, power)| {
                *map.entry(dim).or_insert(0) += *power;
                map
            })
            .into_iter()
            .filter(|(_d, power)| *power != 0)
            .collect()
    }

    pub(crate) fn css_dimension(&self) -> CssDimensionSet {
        use std::collections::BTreeMap;
        let dim = self
            .units
            .iter()
            .map(|(unit, p)| (CssDimension::from(unit.dimension()), p))
            .filter(|(dim, _p)| *dim != CssDimension::None)
            .fold(BTreeMap::new(), |mut map, (dim, power)| {
                *map.entry(dim).or_insert(0) += *power;
                map
            })
            .into_iter()
            .filter(|(_d, power)| *power != 0)
            .collect();
        CssDimensionSet { dim }
    }

    pub(crate) fn valid_in_css(&self) -> bool {
        self.units.len() < 2 && self.css_dimension().valid_in_css()
    }

    /// Get a scaling factor to convert this unit to another unit.
    ///
    /// Returns None if the units are of different dimension.
    pub fn scale_to(&self, other: &Self) -> Option<f64> {
        if let [(u, 1)] = other.units.as_slice() {
            self.scale_to_unit(u)
        } else if other.is_none() {
            self.scale_to_unit(&Unit::None)
        } else {
            let quote = self / other;
            if quote.dimension().is_empty() {
                Some(quote.units.iter().fold(1., |a, (unit, power)| {
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
    pub fn scale_to_unit(&self, other: &Unit) -> Option<f64> {
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
    pub fn simplify(&mut self) -> f64 {
        let mut factor = 1.;
        if self.units.len() > 1 {
            for i in 1..(self.units.len()) {
                let (a, b) = self.units.split_at_mut(i);
                let (au, ap) = a.last_mut().unwrap();
                for (bu, bp) in b {
                    if let Some(f) = bu.scale_to(au) {
                        if ap.abs() > bp.abs() {
                            factor *= f.powi((*bp).into());
                            *ap += *bp;
                            *bp = 0;
                        } else {
                            factor /= f.powi((*ap).into());
                            *bp += *ap;
                            *ap = 0;
                        }
                    }
                }
            }
        }
        self.units.retain(|(_u, p)| *p != 0);
        factor.into()
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
        Self {
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
        // "short" format is used (only?) by the math.units(..) function.
        let short = out.alternate();
        let mut pos = self.units.iter().filter(|(_u, p)| *p > 0);
        let mut neg = self.units.iter().filter(|(_u, p)| *p < 0);

        if let Some((u, p)) = pos.next() {
            write_one(out, u, *p)?;
            for (u, p) in pos {
                out.write_str(if short { "*" } else { " * 1" })?;
                write_one(out, u, p.abs())?;
            }
            if let Some((u, p)) = neg.next() {
                out.write_str(if short { "/" } else { " / 1" })?;
                let paren = short && neg.clone().next().is_some();
                if paren {
                    out.write_char('(')?;
                }
                write_one(out, u, p.abs())?;
                for (u, p) in neg {
                    out.write_str(if short { "*" } else { " / 1" })?;
                    write_one(out, u, p.abs())?;
                }
                if paren {
                    out.write_char(')')?;
                }
            }
        } else if short {
            match (neg.next(), neg.next()) {
                (None, _) => (),
                (Some((u, p)), None) => {
                    write_one(out, u, *p)?;
                }
                (Some((u, p)), Some((nu, np))) => {
                    out.write_str("(")?;
                    write_one(out, u, p.abs())?;
                    out.write_str("*")?;
                    write_one(out, nu, np.abs())?;
                    for (u, p) in neg {
                        out.write_str("*")?;
                        write_one(out, u, p.abs())?;
                    }
                    out.write_str(")^-1")?;
                }
            }
        } else {
            for (u, p) in neg {
                out.write_str(" / 1")?;
                write_one(out, u, p.abs())?;
            }
        }
        Ok(())
    }
}

fn write_one(out: &mut fmt::Formatter, u: &Unit, p: i8) -> fmt::Result {
    u.fmt(out)?;
    if (0..=3).contains(&p) {
        for _ in 1..p {
            write!(out, " * 1{u}")?;
        }
    } else {
        write!(out, "^{p}")?;
    }
    Ok(())
}

impl fmt::Debug for UnitSet {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        out.write_str("UnitSet ")?;
        out.debug_list().entries(&self.units).finish()
    }
}

/// The dimension of a numeric value.
///
/// May be e.g. lenght, or something complex like length^17*angle*time^-3.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct CssDimensionSet {
    dim: Vec<(CssDimension, i8)>,
}
impl CssDimensionSet {
    /// Return true for the empty dimension, i.e. the dimension of a unitless number.
    pub fn is_empty(&self) -> bool {
        self.dim.is_empty()
    }
    /// Return true if any unknown unit is part of the dimension.
    pub fn is_unknown(&self) -> bool {
        self.dim
            .iter()
            .any(|(dim, _)| matches!(dim, CssDimension::Unknown(_)))
    }

    pub(crate) fn valid_in_css(&self) -> bool {
        match &self.dim[..] {
            [] => true,
            [(_d, p)] => *p == 1,
            _ => false,
        }
    }
}
