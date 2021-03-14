use super::{clamp, Rational};
use num_traits::{one, zero, Signed};

/// A color defined by hue, saturation, luminance, and alpha.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Hsla {
    hue: Rational,
    sat: Rational,
    lum: Rational,
    alpha: Rational,
}

impl Hsla {
    /// Create a new hsla color.
    pub fn new(
        hue: Rational,
        sat: Rational,
        lum: Rational,
        alpha: Rational,
    ) -> Hsla {
        Hsla {
            hue: deg_mod(hue),
            sat: clamp(sat, zero(), one()),
            lum: clamp(lum, zero(), one()),
            alpha: clamp(alpha, zero(), one()),
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
        self.alpha = clamp(alpha, zero(), one())
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
