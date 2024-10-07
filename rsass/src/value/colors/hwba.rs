/// A color defined by hue, whiteness, blackness, and alpha.
///
/// All components are rational numbers.
/// The hue is in degrees (0..360).
/// The whiteness, blackness, and alpha are all in the zero to one
/// range (inclusive), whith the additional invariant that whiteness +
/// blackness will never be more than one.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Hwba {
    hue: f64,
    w: f64,
    b: f64,
    alpha: f64,
}

impl Hwba {
    /// Create a new hwba color value.
    ///
    /// Hue is modulo 360 degrees.  Other inputs will be clamped to
    /// their ranges.
    pub fn new(hue: f64, w: f64, b: f64, alpha: f64) -> Self {
        let wbsum = w + b;
        let (w, b) = if wbsum > 1. {
            (w / wbsum, b / wbsum)
        } else {
            (w, b)
        };
        let alpha = alpha.clamp(0., 1.);
        Self { hue, w, b, alpha }
    }

    /// Get the hue of this color.
    pub fn hue(&self) -> f64 {
        self.hue
    }
    /// Get the whiteness of this color.
    ///
    /// Zero is no whiteness, one means this color is white.
    pub fn whiteness(&self) -> f64 {
        self.w
    }
    /// Get the black of this color.
    ///
    /// Zero is no blackness, one means this color is black.
    pub fn blackness(&self) -> f64 {
        self.b
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
}
