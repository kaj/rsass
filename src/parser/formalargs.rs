use super::strings::name;
use super::util::{ignore_comments, opt_spacelike};
use super::value::space_list;
use super::{PResult, Span};
use crate::sass::{CallArgs, FormalArgs, Name};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, map_res, opt};
use nom::multi::separated_list0;
use nom::sequence::{delimited, pair, preceded, terminated};

pub fn formal_args(input: Span) -> PResult<FormalArgs> {
    let (input, _) = terminated(tag("("), opt_spacelike)(input)?;
    let (input, v) = separated_list0(
        preceded(tag(","), opt_spacelike),
        map(
            pair(
                delimited(tag("$"), name, opt_spacelike),
                opt(delimited(
                    terminated(tag(":"), opt_spacelike),
                    space_list,
                    opt_spacelike,
                )),
            ),
            |(name, d)| (name.into(), d),
        ),
    )(input)?;
    let (input, _) = terminated(opt(tag(",")), opt_spacelike)(input)?;
    let (input, va) = terminated(opt(tag("...")), opt_spacelike)(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((
        input,
        if va.is_none() {
            FormalArgs::new(v)
        } else {
            FormalArgs::new_va(v)
        },
    ))
}

pub fn call_args(input: Span) -> PResult<CallArgs> {
    let (input, _) = tag("(")(input)?;
    let (input, v) = map_res(
        separated_list0(
            delimited(opt_spacelike, tag(","), opt_spacelike),
            pair(
                opt(delimited(
                    tag("$"),
                    map(name, Name::from),
                    preceded(ignore_comments, tag(":")),
                )),
                alt((
                    space_list,
                    delimited(ignore_comments, space_list, ignore_comments),
                )),
            ),
        ),
        CallArgs::new,
    )(input)?;
    let (input, _) = preceded(
        opt(delimited(opt_spacelike, opt(tag(",")), opt_spacelike)),
        tag(")"),
    )(input)?;
    Ok((input, v))
}
