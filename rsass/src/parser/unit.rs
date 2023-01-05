use super::strings::unitname;
use super::{PResult, Span};
use crate::value::Unit;
use nom::combinator::{map, value};
use nom::{branch::alt, bytes::complete::tag};

pub fn unit(input: Span) -> PResult<Unit> {
    alt((
        value(Unit::Percent, tag("%")),
        map(unitname, |name| match name.as_ref() {
            // Distance units, <length> type
            "em" => Unit::Em,
            "ex" => Unit::Ex,
            "ch" => Unit::Ch,
            "rem" => Unit::Rem,
            "vw" => Unit::Vw,
            "vh" => Unit::Vh,
            "vmin" => Unit::Vmin,
            "vmax" => Unit::Vmax,
            "cm" => Unit::Cm,
            "mm" => Unit::Mm,
            "Q" | "q" => Unit::Q,
            "in" => Unit::In,
            "pt" => Unit::Pt,
            "pc" => Unit::Pc,
            "px" => Unit::Px,

            // <angle> type
            "deg" => Unit::Deg,
            "grad" => Unit::Grad,
            "rad" => Unit::Rad,
            "turn" => Unit::Turn,

            // <time> type
            "s" => Unit::S,
            "ms" => Unit::Ms,

            // <frequency> type
            "Hz" => Unit::Hz,
            "kHz" => Unit::Khz,

            // <resolution>
            "dpi" => Unit::Dpi,
            "dpcm" => Unit::Dpcm,
            "dppx" => Unit::Dppx,

            // Special units
            "fr" => Unit::Fr,

            name => Unit::Unknown(name.to_string()),
        }),
        value(Unit::None, tag("")),
    ))(input)
}
