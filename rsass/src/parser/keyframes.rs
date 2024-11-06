use super::strings::{sass_string, sass_string_ext2};
use super::util::{comment, ignore_comments, opt_spacelike};
use super::value::{number, value_expression};
use super::PResult;
use super::{variable_declaration, Span};
use crate::sass::{Item, KfItem, SassString, Value};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, map_res, opt, recognize};
use nom::multi::{many_till, separated_list1};
use nom::sequence::{delimited, pair, preceded, terminated};
use std::str::from_utf8;

pub fn keyframes2(input: Span) -> PResult<Item> {
    let (input, setname) =
        dbg!(terminated(sass_string_ext2, opt_spacelike)(input))?;
    let (input, (body, _end)) = preceded(
        terminated(tag("{"), opt_spacelike),
        many_till(
            alt((
                map(
                    pair(
                        separated_list1(
                            delimited(opt_spacelike, tag(","), opt_spacelike),
                            stop_name,
                        ),
                        preceded(
                            delimited(opt_spacelike, tag("{"), opt_spacelike),
                            map(
                                many_till(
                                    body_rule,
                                    terminated(
                                        terminated(tag("}"), opt_spacelike),
                                        opt(tag(";")),
                                    ),
                                ),
                                |(a, _)| a,
                            ),
                        ),
                    ),
                    |(names, body)| KfItem::Stop(names, body),
                ),
                map(
                    terminated(variable_declaration, opt_spacelike),
                    KfItem::VariableDeclaration,
                ),
                map(comment, KfItem::Comment),
            )),
            terminated(terminated(tag("}"), opt_spacelike), opt(tag(";"))),
        ),
    )(input)?;
    Ok((input, Item::KeyFrames(setname, body)))
}

fn body_rule(input: Span) -> PResult<(SassString, Value)> {
    pair(
        terminated(
            sass_string,
            delimited(ignore_comments, tag(":"), ignore_comments),
        ),
        terminated(
            value_expression,
            delimited(opt_spacelike, opt(tag(";")), opt_spacelike),
        ),
    )(input)
}

fn stop_name(input: Span) -> PResult<SassString> {
    alt((
        map_res(recognize(terminated(number, tag("%"))), |s| {
            from_utf8(s.fragment())
                .map(|s| SassString::from(s.to_lowercase()))
        }),
        sass_string,
    ))(input)
}
