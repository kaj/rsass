use nom::types::CompleteByteSlice as Input;
use crate::value::Unit;

named!(pub unit<Input, Unit>,
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
           value!(Unit::Fr, tag!("fr")) |
           value!(Unit::None)));
