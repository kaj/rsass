//! The Unit enum defines css units

use num_rational::Rational;
use num_traits::One;
use std::fmt;

/// Units in css.
///
/// As defined in <https://www.w3.org/TR/css3-values/>
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Unit {
    // Distance units, <length> type
    Em,
    Ex,
    Ch,
    Rem,
    Vw,
    Vh,
    Vmin,
    Vmax,
    Cm,
    Mm,
    Q,
    In,
    Pt,
    Pc,
    Px,
    // Other quantities
    // <angle> type
    Deg,
    Grad,
    Rad,
    Turn,
    // <time> type
    S,
    Ms,
    // <frequency> type
    Hz,
    Khz,
    // <resolution>
    Dpi,
    Dpcm,
    Dppx,
    // Special units
    Percent,
    Fr,
    None,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Dimension {
    LengthAbs,
    LengthVw,
    LengthVh,
    LengthVx,
    LengthRem,
    LenghtEm,
    Angle,
    Time,
    Frequency,
    Resolution,
    None,
}

impl Unit {
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
        }
    }

    pub fn scale_to(&self, other: &Self) -> Option<Rational> {
        if self == other {
            Some(Rational::one())
        } else if self.dimension() == other.dimension() {
            Some(self.scale_factor() / other.scale_factor())
        } else {
            None
        }
    }

    /// Some of these are exact and correct, others are more arbitrary.
    /// When comparing 10cm to 4in, these factors will give correct results.
    /// When comparing rems to vw, who can say?
    fn scale_factor(&self) -> Rational {
        match *self {
            Unit::Em | Unit::Rem => Rational::new(10, 2),
            Unit::Ex => Rational::new(10, 3),
            Unit::Ch => Rational::new(10, 4),
            Unit::Vw | Unit::Vh | Unit::Vmin | Unit::Vmax => Rational::one(),
            Unit::Cm => Rational::new(10, 1),
            Unit::Mm => Rational::one(),
            Unit::Q => Rational::new(1, 4),
            Unit::In => Rational::new(254, 10),
            Unit::Pt => Rational::new(254, 720),
            Unit::Pc => Rational::new(254, 60),
            Unit::Px => Rational::new(254, 960),

            Unit::Deg => Rational::new(1, 360),
            Unit::Grad => Rational::new(1, 400),
            // 1/(2*pi), rational approximation correct to 15 decimals.
            Unit::Rad => Rational::new(25510582 / 2, 80143857),
            Unit::Turn => Rational::one(),

            Unit::S => Rational::one(),
            Unit::Ms => Rational::new(1, 1000),

            Unit::Hz => Rational::one(),
            Unit::Khz => Rational::new(1000, 1),

            Unit::Dpi => Rational::new(96, 1),
            Unit::Dpcm => Rational::new(9600, 254),
            Unit::Dppx => Rational::one(),

            Unit::Percent => Rational::new(1, 100),
            Unit::Fr => Rational::one(),
            Unit::None => Rational::one(),
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
            Unit::Q => write!(out, "q"),
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
        }
    }
}
