use super::strings::name;
use super::util::{ignore_comments, opt_spacelike};
use super::value::space_list;
use super::{PResult, Span};
use crate::sass::{CallArgs, FormalArgs, Name};
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
    delimited(
        terminated(tag("("), opt_spacelike),
        map_res(
            pair(
                separated_list0(
                    terminated(tag(","), opt_spacelike),
                    pair(
                        opt(delimited(
                            tag("$"),
                            map(name, Name::from),
                            delimited(
                                ignore_comments,
                                tag(":"),
                                opt_spacelike,
                            ),
                        )),
                        terminated(space_list, opt_spacelike),
                    ),
                ),
                opt(terminated(tag(","), opt_spacelike)),
            ),
            |(args, trail)| CallArgs::new(args, trail.is_some()),
        ),
        tag(")"),
    )(input)
}
