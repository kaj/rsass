//! Types for color values.
mod convert;
mod hsla;
mod hwba;
mod rgba;

pub use self::hsla::Hsla;
pub use self::hwba::Hwba;
pub use self::rgba::{RgbFormat, Rgba};
use crate::output::{Format, Formatted};
use std::borrow::Cow;
use std::fmt::{self, Display};

/// A color in sass/css. May be a Rgba, Hsla, or Hwba value.
#[derive(Clone, Debug, PartialEq)]
pub enum Color {
    /// A rgba color, defined by red, green, blue and alpha components.
    Rgba(Rgba),
    /// A hsla color, defined by its hue, saturation, lightness and alpha.
    Hsla(Hsla),
    /// A hwba color, defined by its hue, whiteness, blackness and alpha.
    Hwba(Hwba),
}

impl Eq for Color {}
impl Ord for Color {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Color::Hsla(a), Color::Hsla(b)) => a.partial_cmp(b).unwrap(),
            (Color::Hsla(a), Color::Hwba(b)) => {
                a.partial_cmp(&Hsla::from(b)).unwrap()
            }
            (Color::Hwba(a), Color::Hsla(b)) => {
                Hsla::from(a).partial_cmp(b).unwrap()
            }
            (a, b) => a.to_rgba().cmp(&b.to_rgba()),
        }
    }
}
impl PartialOrd for Color {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Color {
    pub(crate) fn is_rgb(&self) -> bool {
        matches!(self, Self::Rgba(_))
    }

    /// Get this color as a rgba value.
    ///
    /// If this color is a rgba value, return a borrow of it.
    /// Otherwise, do the conversion and return an owned value.
    pub fn to_rgba(&self) -> Cow<Rgba> {
        match self {
            Self::Rgba(rgba) => Cow::Borrowed(rgba),
            Self::Hsla(hsla) => Cow::Owned(Rgba::from(hsla)),
            Self::Hwba(hwba) => Cow::Owned(Rgba::from(hwba)),
        }
    }
    /// Get this color as a hsla value.
    ///
    /// If this color is a hsla value, return a borrow of it.
    /// Otherwise, do the conversion and return an owned value.
    pub fn to_hsla(&self) -> Cow<Hsla> {
        match self {
            Self::Rgba(rgba) => Cow::Owned(Hsla::from(rgba)),
            Self::Hsla(ref hsla) => Cow::Borrowed(hsla),
            Self::Hwba(hwba) => Cow::Owned(Hsla::from(hwba)),
        }
    }
    /// Get this color as a hwba value.
    ///
    /// If this color is a hwba value, return a borrow of it.
    /// Otherwise, do the conversion and return an owned value.
    pub fn to_hwba(&self) -> Cow<Hwba> {
        match self {
            Self::Rgba(rgba) => Cow::Owned(Hwba::from(rgba)),
            Self::Hsla(hsla) => Cow::Owned(Hwba::from(hsla)),
            Self::Hwba(hwba) => Cow::Borrowed(hwba),
        }
    }

    /// Get the alpha channel of this color.
    ///
    /// The alpha channel is a rational value between 0 and 1.
    pub fn get_alpha(&self) -> f64 {
        match self {
            Self::Rgba(rgba) => rgba.alpha(),
            Self::Hsla(hsla) => hsla.alpha(),
            Self::Hwba(hwba) => hwba.alpha(),
        }
    }
    /// Set the alpha channel of this color.
    ///
    /// The alpha channel is a rational value between 0 and 1.
    pub fn set_alpha(&mut self, alpha: f64) {
        let alpha = alpha.clamp(0., 1.);
        match self {
            Self::Rgba(ref mut rgba) => rgba.set_alpha(alpha),
            Self::Hsla(ref mut hsla) => hsla.set_alpha(alpha),
            Self::Hwba(ref mut hwba) => hwba.set_alpha(alpha),
        }
    }
    /// Rotate the hue of this color by a specific number of degrees.
    pub fn rotate_hue(&self, val: f64) -> Self {
        match self {
            Self::Rgba(rgba) => {
                let hsla = Hsla::from(rgba);
                Hsla::new(
                    hsla.hue() + val,
                    hsla.sat(),
                    hsla.lum(),
                    hsla.alpha(),
                    hsla.hsla_format,
                )
                .into()
            }
            Self::Hsla(hsla) => Hsla::new(
                hsla.hue() + val,
                hsla.sat(),
                hsla.lum(),
                hsla.alpha(),
                hsla.hsla_format,
            )
            .into(),
            Self::Hwba(hwba) => Hwba::new(
                hwba.hue() + val,
                hwba.whiteness(),
                hwba.blackness(),
                hwba.alpha(),
            )
            .into(),
        }
    }
    pub(crate) fn invert(&self, weight: f64) -> Self {
        match self {
            Color::Rgba(rgba) => rgba.invert(weight).into(),
            Color::Hsla(hsla) => hsla.invert(weight).into(),
            Color::Hwba(hwba) => Rgba::from(hwba).invert(weight).into(),
        }
    }
    pub(crate) fn reset_source(&mut self) {
        match self {
            Self::Rgba(rgba) => rgba.reset_source(),
            Self::Hsla(hsla) => hsla.reset_source(),
            _ => (),
        }
    }
    /// Get a reference to this `Value` bound to an output format.
    pub fn format(&self, format: Format) -> Formatted<Self> {
        Formatted {
            value: self,
            format,
        }
    }
}

impl From<Rgba> for Color {
    fn from(rgba: Rgba) -> Self {
        Self::Rgba(rgba)
    }
}
impl From<Hsla> for Color {
    fn from(hsla: Hsla) -> Self {
        Self::Hsla(hsla)
    }
}
impl From<Hwba> for Color {
    fn from(hwba: Hwba) -> Self {
        Self::Hwba(hwba)
    }
}

impl<'a> Display for Formatted<'a, Color> {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self.value {
            Color::Rgba(rgba) => rgba.format(self.format).fmt(out),
            Color::Hsla(hsla) if hsla.hsla_format => {
                hsla.format(self.format).fmt(out)
            }
            Color::Hwba(hwba) => {
                Hsla::from(hwba).format(self.format).fmt(out)
            }
            any => any.to_rgba().format(self.format).fmt(out),
        }
    }
}
