use super::SrcValue;
use crate::value::{Numeric, ValueRange};
use crate::{Error, Invalid, ScopeRef};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct SrcRange {
    from: SrcValue,
    to: SrcValue,
    inclusive: bool,
}

impl SrcRange {
    pub fn evaluate(&self, scope: ScopeRef) -> Result<ValueRange, Error> {
        let (from, unit) = self.from.eval_map(scope.clone(), |v| {
            let v = v
                .numeric_value()
                .map_err(|v| Invalid::not(&v, "a number"))?;
            let unit = v.unit;
            let v = v.value.into_integer().map_err(|e| {
                Invalid::not(&Numeric::new(e, unit.clone()), "an int")
            })?;

            Ok((v, unit))
        })?;
        let to = self.to.eval_map(scope.clone(), |v| {
            let v = v
                .numeric_value()
                .map_err(|v| Invalid::not(&v, "a number"))?;

            let v = if unit.is_none() || v.is_no_unit() {
                v.value
            } else if let Some(scaled) = v.as_unitset(&unit) {
                scaled
            } else {
                return Err(Invalid::expected_to(
                    &v,
                    &format!("have unit {unit}"),
                ));
            };

            let v = v.into_integer().map_err(|e| {
                Invalid::not(&Numeric::new(e, unit.clone()), "an int")
            })?;

            Ok(v)
        })?;
        Ok(ValueRange::new(from, to, self.inclusive, unit))
    }
}

pub mod parser {
    use super::SrcRange;
    use crate::parser::util::ignore_comments;
    use crate::parser::{single_value_p, PResult, Span};
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::combinator::value;
    use nom::sequence::{delimited, terminated};

    pub fn src_range(input: Span) -> PResult<SrcRange> {
        let (input, from) = delimited(
            terminated(tag("from"), ignore_comments),
            single_value_p,
            ignore_comments,
        )(input)?;
        let (input, inclusive) = terminated(
            alt((value(true, tag("through")), value(false, tag("to")))),
            ignore_comments,
        )(input)?;
        let (input, to) = terminated(single_value_p, ignore_comments)(input)?;
        Ok((
            input,
            SrcRange {
                from,
                to,
                inclusive,
            },
        ))
    }
}
