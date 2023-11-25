//! The Unit enum defines css units

use crate::value::Number;
use num_traits::one;
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
    Unknown(String),
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
            Unit::Cm
            | Unit::Mm
            | Unit::Q
            | Unit::In
            | Unit::Pc
            | Unit::Pt
            | Unit::Px => Dimension::LengthAbs,

            Unit::Vw => Dimension::LengthVw,
            Unit::Vh => Dimension::LengthVh,
            Unit::Vmin | Unit::Vmax => Dimension::LengthVx,
            Unit::Ch | Unit::Em | Unit::Ex => Dimension::LenghtEm,
            Unit::Rem => Dimension::LengthRem,

            Unit::Deg | Unit::Grad | Unit::Rad | Unit::Turn => {
                Dimension::Angle
            }

            Unit::S | Unit::Ms => Dimension::Time,

            Unit::Hz | Unit::Khz => Dimension::Frequency,

            Unit::Dpi | Unit::Dpcm | Unit::Dppx => Dimension::Resolution,

            Unit::Percent | Unit::Fr | Unit::None => Dimension::None,

            Unit::Unknown(ref name) => Dimension::Unknown(name.clone()),
        }
    }

    /// Get a scaling factor to convert this unit to another unit.
    ///
    /// Returns None if the units are of different dimension.
    pub fn scale_to(&self, other: &Self) -> Option<Number> {
        if self == other {
            Some(one())
        } else if self.dimension() == other.dimension() {
            Some(self.scale_factor() / other.scale_factor())
        } else {
            None
        }
    }

    /// Some of these are exact and correct, others are more arbitrary.
    /// When comparing 10cm to 4in, these factors will give correct results.
    /// When comparing rems to vw, who can say?
    pub(crate) fn scale_factor(&self) -> Number {
        #[allow(clippy::match_same_arms)]
        match *self {
            Unit::Em | Unit::Rem => Number::rational(10, 2),
            Unit::Ex => Number::rational(10, 3),
            Unit::Ch => Number::rational(10, 4),
            Unit::Vw | Unit::Vh | Unit::Vmin | Unit::Vmax => one(),
            Unit::Cm => Number::rational(10, 1),
            Unit::Mm => one(),
            Unit::Q => Number::rational(1, 4),
            Unit::In => Number::rational(254, 10),
            Unit::Pt => Number::rational(254, 720),
            Unit::Pc => Number::rational(254, 60),
            Unit::Px => Number::rational(254, 960),

            Unit::Deg => Number::rational(1, 360),
            Unit::Grad => Number::rational(1, 400),
            Unit::Rad => (FRAC_1_PI / 2.0).into(), // 1/(2 pi)
            Unit::Turn => one(),

            Unit::S => one(),
            Unit::Ms => Number::rational(1, 1000),

            Unit::Hz => one(),
            Unit::Khz => Number::rational(1000, 1),

            Unit::Dpi => Number::rational(1, 96),
            Unit::Dpcm => Number::rational(254, 9600),
            Unit::Dppx => one(),

            Unit::Percent => Number::rational(1, 100),
            Unit::Fr => one(),
            Unit::None => one(),

            Unit::Unknown(_) => one(),
        }
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Distance units, <length> type
            Unit::Em => write!(out, "em"),
            Unit::Ex => write!(out, "ex"),
            Unit::Ch => write!(out, "ch"),
            Unit::Rem => write!(out, "rem"),
            Unit::Vw => write!(out, "vw"),
            Unit::Vh => write!(out, "vh"),
            Unit::Vmin => write!(out, "vmin"),
            Unit::Vmax => write!(out, "vmax"),
            Unit::Cm => write!(out, "cm"),
            Unit::Mm => write!(out, "mm"),
            Unit::Q => write!(out, "Q"),
            Unit::In => write!(out, "in"),
            Unit::Pt => write!(out, "pt"),
            Unit::Pc => write!(out, "pc"),
            Unit::Px => write!(out, "px"),
            // <angle> type
            Unit::Deg => write!(out, "deg"),
            Unit::Grad => write!(out, "grad"),
            Unit::Rad => write!(out, "rad"),
            Unit::Turn => write!(out, "turn"),
            // <time> type
            Unit::S => write!(out, "s"),
            Unit::Ms => write!(out, "ms"),
            // <frequency> type
            Unit::Hz => write!(out, "Hz"),
            Unit::Khz => write!(out, "kHz"),
            // <resolution>
            Unit::Dpi => write!(out, "dpi"),
            Unit::Dpcm => write!(out, "dpcm"),
            Unit::Dppx => write!(out, "dppx"),
            // Special units
            Unit::Percent => write!(out, "%"),
            Unit::Fr => write!(out, "fr"),
            Unit::None => Ok(()),

            Unit::Unknown(ref name) => out.write_str(name),
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
            | Dimension::LenghtEm => CssDimension::Length,
            Dimension::Angle => CssDimension::Angle,
            Dimension::Time => CssDimension::Time,
            Dimension::Frequency => CssDimension::Frequency,
            Dimension::Resolution => CssDimension::Resolution,
            Dimension::None => CssDimension::None,
            Dimension::Unknown(s) => CssDimension::Unknown(s),
        }
    }
}
