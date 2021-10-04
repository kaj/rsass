use super::super::util::{ignore_comments, opt_spacelike, spacelike2};
use super::super::{input_to_string, PResult, Span};
use super::strings::{css_string, css_string_any};
use crate::css::{Selector, SelectorPart, Selectors};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::one_of;
use nom::combinator::{map, map_res, opt, value};
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, pair, preceded, terminated, tuple};

pub fn selectors(input: Span) -> PResult<Selectors> {
    let (input, v) = separated_list1(
        terminated(tag(","), ignore_comments),
        opt(selector),
    )(input)?;
    Ok((input, Selectors::new(v.into_iter().flatten().collect())))
}

pub fn selector(input: Span) -> PResult<Selector> {
    let (input, mut s) = many1(selector_part)(input)?;
    if s.last() == Some(&SelectorPart::Descendant) {
        s.pop();
    }
    Ok((input, Selector(s)))
}

pub(crate) fn selector_part(input: Span) -> PResult<SelectorPart> {
    alt((
        map(css_string, SelectorPart::Simple),
        value(SelectorPart::Simple("*".into()), tag("*")),
        map(
            preceded(
                tag("::"),
                pair(
                    css_string,
                    opt(delimited(tag("("), selectors, tag(")"))),
                ),
            ),
            |(name, arg)| SelectorPart::PseudoElement { name, arg },
        ),
        map(
            preceded(
                tag(":"),
                pair(
                    css_string,
                    opt(delimited(tag("("), selectors, tag(")"))),
                ),
            ),
            |(name, arg)| SelectorPart::Pseudo { name, arg },
        ),
        map(
            delimited(
                terminated(tag("["), opt_spacelike),
                tuple((
                    terminated(css_string, opt_spacelike),
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
                tag("]"),
            ),
            |(name, op, val, modifier)| SelectorPart::Attribute {
                name,
                op,
                val,
                modifier,
            },
        ),
        map(
            delimited(
                terminated(tag("["), opt_spacelike),
                css_string,
                preceded(opt_spacelike, tag("]")),
            ),
            |name| SelectorPart::Attribute {
                name,
                op: "".to_string(),
                val: "".into(),
                modifier: None,
            },
        ),
        value(SelectorPart::BackRef, tag("&")),
        delimited(
            opt_spacelike,
            alt((
                value(SelectorPart::RelOp(b'>'), tag(">")),
                value(SelectorPart::RelOp(b'+'), tag("+")),
                value(SelectorPart::RelOp(b'~'), tag("~")),
                value(SelectorPart::RelOp(b'\\'), tag("\\")),
            )),
            opt_spacelike,
        ),
        value(SelectorPart::Descendant, spacelike2),
    ))(input)
}
