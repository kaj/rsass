use crate::css::Value;
use crate::output::{Format, Formatted};
use crate::value::{Number, Numeric};
use std::fmt::{self, Display};

/// A color defined by hue, saturation, luminance, and alpha.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Hsla {
    hue: f64,
    sat: f64,
    lum: f64,
    alpha: f64,
    pub(crate) hsla_format: bool,
}

impl Hsla {
    /// Create a new hsla color.
    pub fn new(
        hue: f64,
        sat: f64,
        lum: f64,
        alpha: f64,
        hsla_format: bool,
    ) -> Self {
        Self {
            hue: deg_mod(hue),
            sat: sat.clamp(0., f64::INFINITY),
            lum,
            alpha: alpha.max(0.).min(1.),
            hsla_format,
        }
    }

    /// Get the hue of this color.
    pub fn hue(&self) -> f64 {
        self.hue
    }
    /// Get the saturation of this color.
    pub fn sat(&self) -> f64 {
        self.sat
    }
    /// Get the lumination of this color.
    pub fn lum(&self) -> f64 {
        self.lum
    }
    /// Get the alpha value of this color.
    ///
    /// Zero is fully transparent, one is fully opaque.
    pub fn alpha(&self) -> f64 {
        self.alpha
    }
    /// Set the alpha value of this color.
    ///
    /// Zero is fully transparent, one is fully opaque.
    pub fn set_alpha(&mut self, alpha: f64) {
        self.alpha = alpha.clamp(0., 1.);
    }

    pub(crate) fn invert(&self, weight: f64) -> Self {
        Self {
            hue: deg_mod(self.hue + 180.),
            sat: self.sat,
            lum: (1. - self.lum) * weight + self.lum * (1. - weight),
            alpha: self.alpha,
            hsla_format: self.hsla_format,
        }
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
fn deg_mod(value: f64) -> f64 {
    let turn = 360.;
    let value = value % turn;
    if value.is_sign_negative() {
        value + turn
    } else {
        value
    }
}

impl<'a> Display for Formatted<'a, Hsla> {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        let hsla = self.value;
        // Special: Don't round the hue to 360!
        let hue = if hsla.hue + 1e-7 > 360. { 0. } else { hsla.hue };
        let hue = Value::scalar(hue);
        let sat = Value::from(Numeric::percentage(hsla.sat));
        let lum = Value::from(Numeric::percentage(hsla.lum));
        let a = hsla.alpha;
        if a >= 1. {
            write!(
                out,
                "hsl({}, {}, {})",
                hue.to_string(self.format),
                sat.to_string(self.format),
                lum.to_string(self.format),
            )
        } else {
            let a = Number::from(a);
            write!(
                out,
                "hsla({}, {}, {}, {})",
                hue.to_string(self.format),
                sat.to_string(self.format),
                lum.to_string(self.format),
                a.format(self.format)
            )
        }
    }
}
