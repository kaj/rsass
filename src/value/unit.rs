//! The Unit enum defines css units

use crate::value::NumValue;
use num_traits::One;
use std::f64::consts::FRAC_1_PI;
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

    pub fn scale_to(&self, other: &Self) -> Option<NumValue> {
        if self == other {
            Some(NumValue::one())
        } else if self.dimension() == other.dimension() {
            Some(self.scale_factor() / other.scale_factor())
        } else {
            None
        }
    }

    /// Some of these are exact and correct, others are more arbitrary.
    /// When comparing 10cm to 4in, these factors will give correct results.
    /// When comparing rems to vw, who can say?
    fn scale_factor(&self) -> NumValue {
        match *self {
            Unit::Em | Unit::Rem => NumValue::rational(10, 2),
            Unit::Ex => NumValue::rational(10, 3),
            Unit::Ch => NumValue::rational(10, 4),
            Unit::Vw | Unit::Vh | Unit::Vmin | Unit::Vmax => NumValue::one(),
            Unit::Cm => NumValue::rational(10, 1),
            Unit::Mm => NumValue::one(),
            Unit::Q => NumValue::rational(1, 4),
            Unit::In => NumValue::rational(254, 10),
            Unit::Pt => NumValue::rational(254, 720),
            Unit::Pc => NumValue::rational(254, 60),
            Unit::Px => NumValue::rational(254, 960),

            Unit::Deg => NumValue::rational(1, 360),
            Unit::Grad => NumValue::rational(1, 400),
            Unit::Rad => NumValue::Float(FRAC_1_PI / 2.0), // 1/(2 pi)
            Unit::Turn => NumValue::one(),

            Unit::S => NumValue::one(),
            Unit::Ms => NumValue::rational(1, 1000),

            Unit::Hz => NumValue::one(),
            Unit::Khz => NumValue::rational(1000, 1),

            Unit::Dpi => NumValue::rational(96, 1),
            Unit::Dpcm => NumValue::rational(9600, 254),
            Unit::Dppx => NumValue::one(),

            Unit::Percent => NumValue::rational(1, 100),
            Unit::Fr => NumValue::one(),
            Unit::None => NumValue::one(),
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
