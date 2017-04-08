//! The Unit enum defines css units

use std::fmt;

/// Units in css.
///
/// As defined in https://www.w3.org/TR/css3-values/
#[derive(Clone, Debug, PartialEq, Eq)]
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
    None,
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
            Unit::None => Ok(()),
        }
    }
}

named!(pub unit<&[u8], Unit>,
       alt_complete!(
           // Distance units, <length> type
           value!(Unit::Em, tag!("em")) |
           value!(Unit::Ex, tag!("ex")) |
           value!(Unit::Ch, tag!("ch")) |
           value!(Unit::Rem, tag!("rem")) |
           value!(Unit::Vw, tag!("vw")) |
           value!(Unit::Vh, tag!("vh")) |
           value!(Unit::Vmin, tag!("vmin")) |
           value!(Unit::Vmax, tag!("vmax")) |
           value!(Unit::Cm, tag!("cm")) |
           value!(Unit::Mm, tag!("mm")) |
           value!(Unit::Q, tag!("q")) |
           value!(Unit::In, tag!("in")) |
           value!(Unit::Pt, tag!("pt")) |
           value!(Unit::Pc, tag!("pc")) |
           value!(Unit::Px, tag!("px")) |
           // <angle> type
           value!(Unit::Deg, tag!("deg")) |
           value!(Unit::Grad, tag!("grad")) |
           value!(Unit::Rad, tag!("rad")) |
           value!(Unit::Turn, tag!("turn")) |
           // <time> type
           value!(Unit::S, tag!("s")) |
           value!(Unit::Ms, tag!("ms")) |
           // <frequency> type
           value!(Unit::Hz, tag!("Hz")) |
           value!(Unit::Khz, tag!("kHz")) |
           // <resolution>
           value!(Unit::Dpi, tag!("dpi")) |
           value!(Unit::Dpcm, tag!("dpcm")) |
           value!(Unit::Dppx, tag!("dppx")) |
           // Special units
           value!(Unit::Percent, tag!("%")) |
           value!(Unit::None, tag!(""))));
