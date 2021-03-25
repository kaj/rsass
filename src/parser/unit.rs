use super::strings::name;
use super::Span;
use crate::value::Unit;
use nom::combinator::{map_opt, value};
use nom::IResult;
use nom::{branch::alt, bytes::complete::tag};

pub fn unit(input: Span) -> IResult<Span, Unit> {
    alt((
        value(Unit::Percent, tag("%")),
        map_opt(name, |name| match name.as_ref() {
            // Distance units, <length> type
            "em" => Some(Unit::Em),
            "ex" => Some(Unit::Ex),
            "ch" => Some(Unit::Ch),
            "rem" => Some(Unit::Rem),
            "vw" => Some(Unit::Vw),
            "vh" => Some(Unit::Vh),
            "vmin" => Some(Unit::Vmin),
            "vmax" => Some(Unit::Vmax),
            "cm" => Some(Unit::Cm),
            "mm" => Some(Unit::Mm),
            "q" => Some(Unit::Q),
            "in" => Some(Unit::In),
            "pt" => Some(Unit::Pt),
            "pc" => Some(Unit::Pc),
            "px" => Some(Unit::Px),

            // <angle> type
            "deg" => Some(Unit::Deg),
            "grad" => Some(Unit::Grad),
            "rad" => Some(Unit::Rad),
            "turn" => Some(Unit::Turn),

            // <time> type
            "s" => Some(Unit::S),
            "ms" => Some(Unit::Ms),

            // <frequency> type
            "Hz" => Some(Unit::Hz),
            "kHz" => Some(Unit::Khz),

            // <resolution>
            "dpi" => Some(Unit::Dpi),
            "dpcm" => Some(Unit::Dpcm),
            "dppx" => Some(Unit::Dppx),

            // Special units
            "fr" => Some(Unit::Fr),

            name if ok_as_unit(name) => Some(Unit::Unknown(name.to_string())),
            _ => None,
        }),
        value(Unit::None, tag("")),
    ))(input)
}

fn ok_as_unit(name: &str) -> bool {
    name.chars().all(|c| c.is_alphabetic())
}
