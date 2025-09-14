//! The Unit enum defines css units

use std::f64::consts::FRAC_1_PI;
use std::fmt;

/// Units in css.
///
/// As defined in <https://www.w3.org/TR/css3-values/>
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Unit {
    /// `em` unit, lengths in em-like dimension.
    Em,
    /// `ex` unit, lengths in em-like dimension.
    Ex,
    /// `ch` unit, lengths in em-like dimension.
    Ch,
    /// `ch` unit, lengths in rem-like dimension.
    Rem,
    /// `vw` unit, length relative to viewport width.
    Vw,
    /// `vh` unit, length relative to viewport height.
    Vh,
    /// `vmin` unit, length relative to min viewport size.
    Vmin,
    /// `vmax` unit, length relative to max viewport size.
    Vmax,
    /// `cm` unit, absolute length.
    Cm,
    /// `mm` unit, absolute length.
    Mm,
    /// `q` unit, absolute length (4Q == 1mm).
    Q,
    /// `in` unit, absolute length in inch.
    In,
    /// `pt` unit, absolute length (72pt == 1in).
    Pt,
    /// `pc`unit, absolute length (1pc == 12pt, 6pc == 1in).
    Pc,
    /// `px`unit, originally pixel size, but does not really mean anything now.
    Px,

    /// `deg` unit, angle in degrees (360 to a turn).
    Deg,
    /// `grad` unit, angle in grad (400 to a turn).
    Grad,
    /// `rad` unit, angle in degrees (2pi to a turn).
    Rad,
    /// `turn` unit, angle in turns.
    Turn,

    /// `s` unit, time in seconds.
    S,
    /// `ms` unit, time in milliseconds.
    Ms,
    /// `hz` unit, frequency in Hz.
    Hz,
    /// `khz` unit, frequency in kHz.
    Khz,

    /// `dpi` unit, resolution in dots per inch.
    Dpi,
    /// `dpcm` unit, resolution in dots per cm.
    Dpcm,
    /// `dppx` unit, resolution in dots per px unit.
    Dppx,

    /// `%` unit, a percentage of something.
    Percent,
    /// `fr` unit, for grid-relative lengths.
    Fr,
    /// No unit.
    None,

    /// An unknown (but named) unit.
    // Note: I tried using a Box<str> (16 bytes, instead of 24), but
    // the niche optimization is better for String, so a Unit is 24
    // bytes in both cases, at least with Rust 1.89.0).
    Unknown(String),
}

#[cfg(all(test, target_pointer_width = "64"))]
mod test_sizes {
    use super::Unit;
    use crate::testutil::test_size;
    // Se above about `Unit::Unknown`.
    // Maybe I should bring this down by interning?
    test_size!(Unit, 24);
}

/// Dimension of a unit.
///
/// Units of the same dimension can be converted to each other.
/// There are multiple "length" dimensions, since font-based,
/// window-based and absolute lengths can't be converted to each
/// other.
///
/// This type is for compatibility in sass functions.
/// See also [`CssDimension`].
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Dimension {
    /// An absolute length, can be converted to metric.
    LengthAbs,
    /// A length relative to viewport width.
    LengthVw,
    /// A length relative to viewport height.
    LengthVh,
    /// A length relative to viewport size (min or max).
    LengthVx,
    /// A length relatvie to base font size.
    LengthRem,
    /// A length relative to font size.
    LenghtEm,
    /// An angle.
    Angle,
    /// A duration.
    Time,
    /// A frequency.
    Frequency,
    /// A resolution (number of pixels per length).
    Resolution,
    /// No dimension (no unit, percentage, or grid fraction).
    None,
    /// The dimension of an unknown (but named) unit.
    Unknown(String),
}

impl Unit {
    /// Get the dimension of this unit.
    pub fn dimension(&self) -> Dimension {
        match *self {
            Self::Cm
            | Self::Mm
            | Self::Q
            | Self::In
            | Self::Pc
            | Self::Pt
            | Self::Px => Dimension::LengthAbs,

            Self::Vw => Dimension::LengthVw,
            Self::Vh => Dimension::LengthVh,
            Self::Vmin | Self::Vmax => Dimension::LengthVx,
            Self::Ch | Self::Em | Self::Ex => Dimension::LenghtEm,
            Self::Rem => Dimension::LengthRem,

            Self::Deg | Self::Grad | Self::Rad | Self::Turn => {
                Dimension::Angle
            }

            Self::S | Self::Ms => Dimension::Time,

            Self::Hz | Self::Khz => Dimension::Frequency,

            Self::Dpi | Self::Dpcm | Self::Dppx => Dimension::Resolution,

            Self::Percent | Self::Fr | Self::None => Dimension::None,

            Self::Unknown(ref name) => Dimension::Unknown(name.clone()),
        }
    }

    /// Get a scaling factor to convert this unit to another unit.
    ///
    /// Returns None if the units are of different dimension.
    pub fn scale_to(&self, other: &Self) -> Option<f64> {
        if self == other {
            Some(1.)
        } else if self.dimension() == other.dimension() {
            Some(self.scale_factor() / other.scale_factor())
        } else {
            None
        }
    }

    /// Some of these are exact and correct, others are more arbitrary.
    /// When comparing 10cm to 4in, these factors will give correct results.
    /// When comparing rems to vw, who can say?
    pub(crate) fn scale_factor(&self) -> f64 {
        #[allow(clippy::match_same_arms)]
        match *self {
            Self::Em | Self::Rem => 5.,
            Self::Ex => 3.,
            Self::Ch => 2.,
            Self::Vw | Self::Vh | Self::Vmin | Self::Vmax => 1.,
            Self::Cm => 10.,
            Self::Mm => 1.,
            Self::Q => 1. / 4.,
            Self::In => 254. / 10.,
            Self::Pt => 254. / 720.,
            Self::Pc => 254. / 60.,
            Self::Px => 254. / 960.,

            Self::Deg => 1. / 360.,
            Self::Grad => 1. / 400.,
            Self::Rad => FRAC_1_PI / 2.0,
            Self::Turn => 1.,

            Self::S => 1.,
            Self::Ms => 1. / 1000.,

            Self::Hz => 1.,
            Self::Khz => 1000.,

            Self::Dpi => 1. / 96.,
            Self::Dpcm => 254. / 9600.,
            Self::Dppx => 1.,

            Self::Percent => 1. / 100.,
            Self::Fr => 1.,
            Self::None => 1.,

            Self::Unknown(_) => 1.,
        }
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Distance units, <length> type
            Self::Em => write!(out, "em"),
            Self::Ex => write!(out, "ex"),
            Self::Ch => write!(out, "ch"),
            Self::Rem => write!(out, "rem"),
            Self::Vw => write!(out, "vw"),
            Self::Vh => write!(out, "vh"),
            Self::Vmin => write!(out, "vmin"),
            Self::Vmax => write!(out, "vmax"),
            Self::Cm => write!(out, "cm"),
            Self::Mm => write!(out, "mm"),
            Self::Q => write!(out, "Q"),
            Self::In => write!(out, "in"),
            Self::Pt => write!(out, "pt"),
            Self::Pc => write!(out, "pc"),
            Self::Px => write!(out, "px"),
            // <angle> type
            Self::Deg => write!(out, "deg"),
            Self::Grad => write!(out, "grad"),
            Self::Rad => write!(out, "rad"),
            Self::Turn => write!(out, "turn"),
            // <time> type
            Self::S => write!(out, "s"),
            Self::Ms => write!(out, "ms"),
            // <frequency> type
            Self::Hz => write!(out, "Hz"),
            Self::Khz => write!(out, "kHz"),
            // <resolution>
            Self::Dpi => write!(out, "dpi"),
            Self::Dpcm => write!(out, "dpcm"),
            Self::Dppx => write!(out, "dppx"),
            // Special units
            Self::Percent => write!(out, "%"),
            Self::Fr => write!(out, "fr"),
            Self::None => Ok(()),

            Self::Unknown(ref name) => out.write_str(name),
        }
    }
}

/// Dimension of a unit.
///
/// Units of the same dimension can be converted to each other.
/// There is a single "length" dimension, since all lengths _can_ be
/// converted to each other in the browser, where all are converted to
/// device pixels anyway.
///
/// This type is for compatibility in css functions.
/// See also [`Dimension`].
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CssDimension {
    /// A length of any kind.
    Length,
    /// An angle.
    Angle,
    /// A duration.
    Time,
    /// A frequency.
    Frequency,
    /// A resolution (number of pixels per length).
    Resolution,
    /// No dimension (no unit, percentage, or grid fraction).
    None,
    /// The dimension of an unknown (but named) unit.
    Unknown(String),
}

impl From<Dimension> for CssDimension {
    fn from(dim: Dimension) -> Self {
        match dim {
            Dimension::LengthAbs
            | Dimension::LengthVw
            | Dimension::LengthVh
            | Dimension::LengthVx
            | Dimension::LengthRem
            | Dimension::LenghtEm => Self::Length,
            Dimension::Angle => Self::Angle,
            Dimension::Time => Self::Time,
            Dimension::Frequency => Self::Frequency,
            Dimension::Resolution => Self::Resolution,
            Dimension::None => Self::None,
            Dimension::Unknown(s) => Self::Unknown(s),
        }
    }
}
