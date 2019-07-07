use super::strings::name;
use super::util::{ignore_comments, opt_spacelike};
use super::value::space_list;
use crate::sass::{CallArgs, FormalArgs, Value};
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::multi::separated_list;
use nom::sequence::{delimited, pair, preceded, terminated};
use nom::*;

pub fn formal_args(input: &[u8]) -> IResult<&[u8], FormalArgs> {
    let (input, _) = terminated(tag("("), opt_spacelike)(input)?;
    let (input, v) = separated_list(
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
            |(name, d)| (name.replace('-', "_"), d.unwrap_or(Value::Null)),
        ),
    )(input)?;
    let (input, _) = terminated(opt(tag(",")), opt_spacelike)(input)?;
    let (input, va) = terminated(opt(tag("...")), opt_spacelike)(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((input, FormalArgs::new(v, va.is_some())))
}

named!(pub call_args<CallArgs>,
       delimited!(
           tag("("),
           map!(separated_list!(
               delimited!(opt_spacelike, tag(","), opt_spacelike),
               pair!(opt!(delimited!(
                        tag("$"),
                        map!(name, |n: String| n.replace("-", "_")),
                        preceded!(ignore_comments,
                                  tag(":")))),
                     alt!(space_list |
                          delimited!(ignore_comments,
                                     space_list,
                                     ignore_comments)))),
                CallArgs::new),
           preceded!(
               opt!(delimited!(opt_spacelike, opt!(tag(",")), opt_spacelike)),
               tag(")"))));
