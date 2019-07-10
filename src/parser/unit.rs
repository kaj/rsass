use crate::value::Unit;
use nom::IResult;

pub fn unit(input: &[u8]) -> IResult<&[u8], Unit> {
    Ok(match input {
        // Distance units, <length> type
        v if v.starts_with(b"em") => (&v[2..], Unit::Em),
        v if v.starts_with(b"ex") => (&v[2..], Unit::Ex),
        v if v.starts_with(b"ch") => (&v[2..], Unit::Ch),
        v if v.starts_with(b"rem") => (&v[3..], Unit::Rem),
        v if v.starts_with(b"vw") => (&v[2..], Unit::Vw),
        v if v.starts_with(b"vh") => (&v[2..], Unit::Vh),
        v if v.starts_with(b"vmin") => (&v[4..], Unit::Vmin),
        v if v.starts_with(b"vmax") => (&v[4..], Unit::Vmax),
        v if v.starts_with(b"cm") => (&v[2..], Unit::Cm),
        v if v.starts_with(b"mm") => (&v[2..], Unit::Mm),
        v if v.starts_with(b"q") => (&v[1..], Unit::Q),
        v if v.starts_with(b"in") => (&v[2..], Unit::In),
        v if v.starts_with(b"pt") => (&v[2..], Unit::Pt),
        v if v.starts_with(b"pc") => (&v[2..], Unit::Pc),
        v if v.starts_with(b"px") => (&v[2..], Unit::Px),

        // <angle> type
        v if v.starts_with(b"deg") => (&v[3..], Unit::Deg),
        v if v.starts_with(b"grad") => (&v[4..], Unit::Grad),
        v if v.starts_with(b"rad") => (&v[3..], Unit::Rad),
        v if v.starts_with(b"turn") => (&v[4..], Unit::Turn),

        // <time> type
        v if v.starts_with(b"s") => (&v[1..], Unit::S),
        v if v.starts_with(b"ms") => (&v[2..], Unit::Ms),

        // <frequency> type
        v if v.starts_with(b"Hz") => (&v[2..], Unit::Hz),
        v if v.starts_with(b"kHz") => (&v[3..], Unit::Khz),

        // <resolution>
        v if v.starts_with(b"dpi") => (&v[3..], Unit::Dpi),
        v if v.starts_with(b"dpcm") => (&v[4..], Unit::Dpcm),
        v if v.starts_with(b"dppx") => (&v[4..], Unit::Dppx),

        // Special units
        v if v.starts_with(b"fr") => (&v[2..], Unit::Fr),
        v if v.starts_with(b"%") => (&v[1..], Unit::Percent),

        v => (v, Unit::None),
    })
}
