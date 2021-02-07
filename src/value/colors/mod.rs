//! Types for color values.
mod hsla;
mod rgba;

pub use self::hsla::Hsla;
pub use self::rgba::Rgba;
use crate::output::{Format, Formatted};
use crate::value::Number;
use num_rational::Rational;
use num_traits::{one, zero, One, Zero};
use std::borrow::Cow;
use std::fmt::{self, Display};

/// A color in sass/css, may be a Rgba or a Hsla value.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Color {
    Rgba(Rgba),
    Hsla(Hsla),
}

impl Color {
    /// Get this color as a rgba value.
    ///
    /// If this color is a rgba value, return a borrow if it.
    /// Otherwise, do the conversion and return an owned value.
    pub fn to_rgba(&self) -> Cow<Rgba> {
        match self {
            Color::Rgba(ref rgba) => Cow::Borrowed(rgba),
            Color::Hsla(hsla) => Cow::Owned(Rgba::from_hsla(
                hsla.hue, hsla.sat, hsla.lum, hsla.alpha,
            )),
        }
    }
    /// Get this color as a hsla value.
    ///
    /// If this color is a hsla value, return a borrow if it.
    /// Otherwise, do the conversion and return an owned value.
    pub fn to_hsla(&self) -> Cow<Hsla> {
        match self {
            Color::Rgba(rgba) => Cow::Owned(Hsla::from_rgba(rgba)),
            Color::Hsla(ref hsla) => Cow::Borrowed(hsla),
        }
    }
    pub fn to_hwba(&self) -> (Rational, Rational, Rational, Rational) {
        match self {
            Color::Rgba(rgba) => {
                let hsla = Hsla::from_rgba(rgba);
                let white = rgba.get_whiteness();
                let black = rgba.get_blackness();
                (hsla.hue, white, black, hsla.alpha)
            }
            Color::Hsla(hsla) => {
                let rgba =
                    Rgba::from_hsla(hsla.hue, hsla.sat, hsla.lum, hsla.alpha);
                let white = rgba.get_whiteness();
                let black = rgba.get_blackness();
                (hsla.hue, white, black, hsla.alpha)
            }
        }
    }
    pub fn from_hwba(
        hue: Rational,
        w: Rational,
        b: Rational,
        a: Rational,
    ) -> Self {
        let wbsum = w + b;
        let (w, b) = if wbsum > Rational::one() {
            (w / wbsum, b / wbsum)
        } else {
            (w, b)
        };

        let l = (Rational::one() - b + w) / 2;
        let s = if l.is_zero() || l.is_one() {
            Rational::zero()
        } else {
            (Rational::one() - b - l) / std::cmp::min(l, Rational::one() - l)
        };
        Hsla::new(hue, s, l, a).into()
    }
    pub fn get_alpha(&self) -> Rational {
        match self {
            Color::Rgba(rgba) => rgba.alpha,
            Color::Hsla(hsla) => hsla.alpha,
        }
    }
    pub fn set_alpha(&mut self, alpha: Rational) {
        let alpha = clamp(alpha, zero(), one());
        match self {
            Color::Rgba(ref mut rgba) => rgba.alpha = alpha,
            Color::Hsla(ref mut hsla) => hsla.alpha = alpha,
        }
    }
    pub fn format(&self, format: Format) -> Formatted<Self> {
        Formatted {
            value: self,
            format,
        }
    }
}

impl From<Rgba> for Color {
    fn from(rgba: Rgba) -> Color {
        Color::Rgba(rgba)
    }
}
impl From<Hsla> for Color {
    fn from(hsla: Hsla) -> Color {
        Color::Hsla(hsla)
    }
}

fn clamp(v: Rational, min: Rational, max: Rational) -> Rational {
    let v = if v < min { min } else { v };
    if v > max {
        max
    } else {
        v
    }
}

impl<'a> Display for Formatted<'a, Color> {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        // The byte-version of alpha is not used here.
        let rgba = self.value.to_rgba();
        let (r, g, b, _a) = rgba.to_bytes();
        let a = self.value.get_alpha();
        if a >= Rational::one() {
            // E.g. #ff00cc can be written #f0c in css.
            // 0xff / 0x11 = 0xf.
            let short = r % 0x11 == 0 && g % 0x11 == 0 && b % 0x11 == 0;
            let hex_len = if short { 4 } else { 7 };
            if let Some(name) = rgba.name() {
                if !(self.format.is_compressed() && name.len() > hex_len) {
                    return name.fmt(out);
                }
            }
            if self.format.is_compressed() && short {
                write!(out, "#{:x}{:x}{:x}", r / 0x11, g / 0x11, b / 0x11)
            } else {
                write!(out, "#{:02x}{:02x}{:02x}", r, g, b)
            }
        } else if self.format.is_compressed() && rgba.all_zero() {
            write!(out, "transparent")
        } else if self.format.is_compressed() {
            // Note: libsass does not use the format for the alpha like this.
            let a = Number::from(a);
            write!(out, "rgba({},{},{},{})", r, g, b, a.format(self.format))
        } else {
            let a = Number::from(a);
            write!(
                out,
                "rgba({}, {}, {}, {})",
                r,
                g,
                b,
                a.format(self.format)
            )
        }
    }
}
