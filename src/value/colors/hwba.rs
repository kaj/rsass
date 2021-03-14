use super::{clamp, Rational};
use num_traits::{one, zero};

/// A color defined by hue, whiteness, blackness, and alpha.
///
/// All components are rational numbers.
/// The hue is in degrees (0..360).
/// The whiteness, blackness, and alpha are all in the zero to one
/// range (inclusive), whith the additional invariant that whiteness +
/// blackness will never be more than one.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Hwba {
    hue: Rational,
    w: Rational,
    b: Rational,
    alpha: Rational,
}

impl Hwba {
    /// Create a new hwba color value.
    ///
    /// Hue is modulo 360 degrees.  Other inputs will be clamped to
    /// their ranges.
    pub fn new(
        hue: Rational,
        w: Rational,
        b: Rational,
        alpha: Rational,
    ) -> Hwba {
        let mut w = clamp(w, zero(), one());
        let mut b = clamp(b, zero(), one());
        let wbsum = w + b;
        if w + b > one() {
            w /= wbsum;
            b /= wbsum;
        }
        Hwba {
            hue,
            w,
            b,
            alpha: clamp(alpha, zero(), one()),
        }
    }

    /// Get the hue of this color.
    pub fn hue(&self) -> Rational {
        self.hue
    }
    /// Get the whiteness of this color.
    ///
    /// Zero is no whiteness, one means this color is white.
    pub fn whiteness(&self) -> Rational {
        self.w
    }
    /// Get the black of this color.
    ///
    /// Zero is no blackness, one means this color is black.
    pub fn blackness(&self) -> Rational {
        self.b
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
