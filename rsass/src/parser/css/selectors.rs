use super::super::util::{ignore_comments, opt_spacelike, spacelike2};
use super::super::{input_to_string, PResult, Span};
use super::strings::{css_string, css_string_any};
use crate::css::{OldSelector, OldSelectorPart, OldSelectors};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::one_of;
use nom::combinator::{into, map, map_res, opt, value};
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, pair, preceded, terminated, tuple};

pub fn selectors(input: Span) -> PResult<OldSelectors> {
    map(
        separated_list1(terminated(tag(","), ignore_comments), selector),
        OldSelectors::new,
    )(input)
}

pub fn selector(input: Span) -> PResult<OldSelector> {
    let (input, mut s) = selector_parts(input)?;
    if s.last() == Some(&OldSelectorPart::Descendant) {
        s.pop();
    }
    Ok((input, OldSelector(s)))
}

pub(crate) fn selector_parts(input: Span) -> PResult<Vec<OldSelectorPart>> {
    many1(selector_part)(input)
}

fn selector_part(input: Span) -> PResult<OldSelectorPart> {
    let (input, mark) =
        alt((tag("&"), tag("::"), tag(":"), tag("."), tag("["), tag("")))(
            input,
        )?;
    match mark.fragment() {
        b"&" => value(OldSelectorPart::BackRef, tag(""))(input),
        b"::" => map(
            pair(
                into(css_string),
                opt(delimited(tag("("), selectors, tag(")"))),
            ),
            |(name, arg)| OldSelectorPart::PseudoElement { name, arg },
        )(input),
        b":" => map(
            pair(
                into(css_string),
                opt(delimited(tag("("), selectors, tag(")"))),
            ),
            |(name, arg)| OldSelectorPart::Pseudo { name, arg },
        )(input),
        b"." => map(simple_part, |mut s| {
            s.insert(0, '.');
            OldSelectorPart::Simple(s)
        })(input),
        b"[" => delimited(
            opt_spacelike,
            alt((
                map(
                    tuple((
                        terminated(name_opt_ns, opt_spacelike),
                        terminated(
                            map_res(
                                alt((
                                    tag("*="),
                                    tag("|="),
                                    tag("="),
                                    tag("$="),
                                    tag("~="),
                                    tag("^="),
                                )),
                                input_to_string,
                            ),
                            opt_spacelike,
                        ),
                        terminated(css_string_any, opt_spacelike),
                        opt(terminated(
                            one_of(
                                "ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                 abcdefghijklmnopqrstuvwxyz",
                            ),
                            opt_spacelike,
                        )),
                    )),
                    |(name, op, val, modifier)| OldSelectorPart::Attribute {
                        name: name.into(),
                        op,
                        val,
                        modifier,
                    },
                ),
                map(terminated(name_opt_ns, opt_spacelike), |name| {
                    OldSelectorPart::Attribute {
                        name: name.into(),
                        op: "".to_string(),
                        val: "".into(),
                        modifier: None,
                    }
                }),
            )),
            tag("]"),
        )(input),
        b"" => alt((
            map(simple_part, OldSelectorPart::Simple),
            delimited(
                opt_spacelike,
                alt((
                    value(OldSelectorPart::RelOp(b'>'), tag(">")),
                    value(OldSelectorPart::RelOp(b'+'), tag("+")),
                    value(OldSelectorPart::RelOp(b'~'), tag("~")),
                    value(OldSelectorPart::RelOp(b'\\'), tag("\\")),
                )),
                opt_spacelike,
            ),
            value(OldSelectorPart::Descendant, spacelike2),
        ))(input),
        _ => unreachable!(),
    }
}

fn name_opt_ns(input: Span) -> PResult<String> {
    fn name_part(input: Span) -> PResult<String> {
        alt((value(String::from("*"), tag("*")), css_string))(input)
    }
    alt((
        map(preceded(tag("|"), name_part), |mut s| {
            s.insert(0, '|');
            s
        }),
        map(
            pair(name_part, opt(preceded(tag("|"), name_part))),
            |(a, b)| {
                if let Some(b) = b {
                    format!("{a}|{b}")
                } else {
                    a
                }
            },
        ),
    ))(input)
}

fn simple_part(input: Span) -> PResult<String> {
    let (rest, (pre, mut s, post)) =
        tuple((opt(tag("%")), name_opt_ns, opt(tag("%"))))(input)?;
    if pre.is_some() {
        s.insert(0, '%');
    }
    if post.is_some() {
        s.push('%');
    }
    Ok((rest, s))
}
