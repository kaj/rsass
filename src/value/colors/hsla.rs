use crate::output::{Format, Formatted};
use crate::value::{Number, Numeric, Rational, Unit};
use num_traits::{one, zero, Signed};
use std::fmt::{self, Display};

/// A color defined by hue, saturation, luminance, and alpha.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Hsla {
    hue: Rational,
    sat: Rational,
    lum: Rational,
    alpha: Rational,
    pub(crate) hsla_format: bool,
}

impl Hsla {
    /// Create a new hsla color.
    pub fn new(
        hue: Rational,
        sat: Rational,
        lum: Rational,
        alpha: Rational,
        hsla_format: bool,
    ) -> Hsla {
        Hsla {
            hue: deg_mod(hue),
            sat: sat.clamp(zero(), one()),
            lum: lum.clamp(zero(), one()),
            alpha: alpha.clamp(zero(), one()),
            hsla_format,
        }
    }

    /// Get the hue of this color.
    pub fn hue(&self) -> Rational {
        self.hue
    }
    /// Get the saturation of this color.
    pub fn sat(&self) -> Rational {
        self.sat
    }
    /// Get the lumination of this color.
    pub fn lum(&self) -> Rational {
        self.lum
    }
    /// Get the alpha value of this color.
    ///
    /// Zero is fully transparent, one is fully opaque.
    pub fn alpha(&self) -> Rational {
        self.alpha
    }
    /// Set the alpha value of this color.
    ///
    /// Zero is fully transparent, one is fully opaque.
    pub fn set_alpha(&mut self, alpha: Rational) {
        self.alpha = alpha.clamp(zero(), one())
    }
    pub(crate) fn reset_source(&mut self) {
        self.hsla_format = false;
    }

    /// Get a reference to this `Value` bound to an output format.
    pub fn format(&self, format: Format) -> Formatted<Self> {
        Formatted {
            value: self,
            format,
        }
    }
}

/// Value is an angle in degrees, return same angle, but 0 <= value < 360.x
fn deg_mod(value: Rational) -> Rational {
    let turn = Rational::from_integer(360);
    let value = value % turn;
    if value.is_negative() {
        value + turn
    } else {
        value
    }
}

impl<'a> Display for Formatted<'a, Hsla> {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        let hsla = self.value;
        let hue = Numeric::new(hsla.hue, Unit::Deg);
        let a = hsla.alpha;
        if a >= one() {
            write!(
                out,
                "hsl({}, {}%, {}%)",
                hue.format(self.format),
                hsla.sat * 100,
                hsla.lum * 100
            )
        } else {
            let a = Number::from(a);
            write!(
                out,
                "hsla({}, {}%, {}%, {})",
                hue.format(self.format),
                hsla.sat * 100,
                hsla.lum * 100,
                a.format(self.format)
            )
        }
    }
}
