use super::strings::{sass_string, sass_string_dq, sass_string_sq};
use super::util::{ignore_comments, opt_spacelike, spacelike2};
use super::{input_to_string, PResult, Span};
use crate::sass::{Selector, SelectorPart, Selectors};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::one_of;
use nom::combinator::{map, map_opt, map_res, opt, value};
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, pair, preceded, terminated, tuple};

pub fn selectors(input: Span) -> PResult<Selectors> {
    map_opt(
        separated_list1(terminated(tag(","), ignore_comments), opt(selector)),
        |v| {
            let v = v.into_iter().flatten().collect::<Vec<_>>();
            if v.is_empty() {
                None
            } else {
                Some(Selectors::new(v))
            }
        },
    )(input)
}

pub fn selector(input: Span) -> PResult<Selector> {
    let (input, mut s) = many1(selector_part)(input)?;
    if s.last() == Some(&SelectorPart::Descendant) {
        s.pop();
    }
    Ok((input, Selector::new(s)))
}

fn selector_part(input: Span) -> PResult<SelectorPart> {
    alt((
        map(
            tuple((opt(tag("%")), sass_string, opt(tag("%")))),
            |(pre, mut s, post)| {
                if pre.is_some() {
                    s.prepend("%");
                }
                if post.is_some() {
                    s.append_str("%");
                }
                SelectorPart::Simple(s)
            },
        ),
        value(SelectorPart::Simple("*".into()), tag("*")),
        map(
            preceded(
                tag("::"),
                pair(
                    sass_string,
                    opt(delimited(tag("("), selectors, tag(")"))),
                ),
            ),
            |(name, arg)| SelectorPart::PseudoElement { name, arg },
        ),
        map(
            preceded(
                tag(":"),
                pair(
                    sass_string,
                    opt(delimited(tag("("), selectors, tag(")"))),
                ),
            ),
            |(name, arg)| SelectorPart::Pseudo { name, arg },
        ),
        map(
            delimited(
                terminated(tag("["), opt_spacelike),
                tuple((
                    terminated(sass_string, opt_spacelike),
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
                    terminated(
                        alt((sass_string_dq, sass_string_sq, sass_string)),
                        opt_spacelike,
                    ),
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
                sass_string,
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

#[cfg(test)]
mod test {
    use super::super::check_parse;
    use super::*;
    use crate::sass::{SassString, StringPart};
    use crate::value::Quotes;

    #[test]
    fn simple_selector() {
        assert_eq!(
            check_parse(selector, b"foo "),
            Ok(Selector::new(vec![SelectorPart::Simple("foo".into())])),
        )
    }
    #[test]
    fn escaped_simple_selector() {
        assert_eq!(
            check_parse(selector, b"\\E9m "),
            Ok(Selector::new(vec![SelectorPart::Simple("ém".into())])),
        )
    }

    #[test]
    fn selector2() {
        assert_eq!(
            check_parse(selector, b"foo bar "),
            Ok(Selector::new(vec![
                SelectorPart::Simple("foo".into()),
                SelectorPart::Descendant,
                SelectorPart::Simple("bar".into()),
            ])),
        )
    }

    #[test]
    fn child_selector() {
        assert_eq!(
            check_parse(selector, b"foo > bar "),
            Ok(Selector::new(vec![
                SelectorPart::Simple("foo".into()),
                SelectorPart::RelOp(b'>'),
                SelectorPart::Simple("bar".into()),
            ])),
        )
    }

    #[test]
    fn foo1_selector() {
        assert_eq!(
            check_parse(selector, b"[data-icon='test-1'] "),
            Ok(Selector::new(vec![SelectorPart::Attribute {
                name: "data-icon".into(),
                op: "=".into(),
                val: SassString::new(
                    vec![StringPart::Raw("test-1".into())],
                    Quotes::Single,
                ),
                modifier: None,
            }])),
        )
    }

    #[test]
    fn pseudo_selector() {
        assert_eq!(
            check_parse(selector, b":before "),
            Ok(Selector::new(vec![SelectorPart::Pseudo {
                name: "before".into(),
                arg: None,
            }])),
        )
    }
    #[test]
    fn pseudo_on_simple_selector() {
        assert_eq!(
            check_parse(selector, b"figure:before "),
            Ok(Selector::new(vec![
                SelectorPart::Simple("figure".into()),
                SelectorPart::Pseudo {
                    name: "before".into(),
                    arg: None,
                },
            ])),
        )
    }

    #[test]
    fn selectors_simple() {
        assert_eq!(
            check_parse(selectors, b"foo, bar "),
            Ok(Selectors::new(vec![
                Selector::new(vec![SelectorPart::Simple("foo".into())]),
                Selector::new(vec![SelectorPart::Simple("bar".into())]),
            ])),
        )
    }
}
