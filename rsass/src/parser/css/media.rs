use super::super::util::ignore_comments;
use super::{opt_spacelike, spacelike, strings, values, PResult, Span};
use crate::css::{MediaArgs, Value};
use crate::value::{ListSeparator, Operator};
use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case};
use nom::combinator::{map, value};
use nom::multi::{fold_many0, fold_many1, separated_list1};
use nom::sequence::{delimited, pair, preceded, terminated};
use std::str::from_utf8;

pub fn args(input: Span) -> PResult<MediaArgs> {
    map(
        separated_list1(
            delimited(opt_spacelike, tag(","), spacelike),
            media_args_and,
        ),
        |v| {
            if v.len() == 1 {
                v.into_iter().next().unwrap()
            } else {
                MediaArgs::Comma(v)
            }
        },
    )(input)
}
fn media_args_and(input: Span) -> PResult<MediaArgs> {
    map(
        separated_list1(
            delimited(opt_spacelike, tag_no_case("and"), ignore_comments),
            media_args_or,
        ),
        |v| {
            if v.len() == 1 {
                v.into_iter().next().unwrap()
            } else {
                MediaArgs::And(v)
            }
        },
    )(input)
}

fn media_args_or(input: Span) -> PResult<MediaArgs> {
    map(
        separated_list1(
            delimited(opt_spacelike, tag_no_case("or"), ignore_comments),
            media_args_one,
        ),
        |v| {
            if v.len() == 1 {
                v.into_iter().next().unwrap()
            } else {
                MediaArgs::Or(v)
            }
        },
    )(input)
}

fn media_args_one(input: Span) -> PResult<MediaArgs> {
    alt((
        map(
            pair(
                terminated(
                    alt((tag_no_case("not"), tag_no_case("only"))),
                    ignore_comments,
                ),
                media_args_one,
            ),
            |(op, a)| {
                MediaArgs::UnaryOp(
                    from_utf8(&op).unwrap().into(),
                    Box::new(a),
                )
            },
        ),
        map(strings::css_string, MediaArgs::Name),
        delimited(
            tag("("),
            alt((
                map(
                    pair(
                        terminated(strings::css_string, tag(": ")),
                        values::any,
                    ),
                    |(k, v)| MediaArgs::Condition(k, v),
                ),
                map(media_relation, MediaArgs::Range),
                map(args, |v| MediaArgs::Paren(Box::new(v))),
            )),
            preceded(opt_spacelike, tag(")")),
        ),
        delimited(
            tag("["),
            map(media_args_one, |v| MediaArgs::Bracket(Box::new(v))),
            tag("]"),
        ),
    ))(input)
}

fn media_relation(input: Span) -> PResult<Vec<(Operator, Value)>> {
    let (rest, first) = media_value(input)?;
    fold_many1(
        pair(
            delimited(opt_spacelike, relational_operator, opt_spacelike),
            media_value,
        ),
        move || vec![(Operator::Equal, first.clone())],
        |mut acc, item| {
            acc.push(item);
            acc
        },
    )(rest)
}

pub fn relational_operator(input: Span) -> PResult<Operator> {
    alt((
        value(Operator::Equal, tag("==")),
        value(Operator::EqualSingle, tag("=")),
        value(Operator::NotEqual, tag("!=")),
        value(Operator::GreaterE, tag(">=")),
        value(Operator::Greater, tag(">")),
        value(Operator::LesserE, tag("<=")),
        value(Operator::Lesser, tag("<")),
    ))(input)
}

/// Any css value that is allowd in a media relation / range.
fn media_value(input: Span) -> PResult<Value> {
    alt((
        media_slash_list_no_space,
        delimited(
            terminated(tag("("), opt_spacelike),
            media_value,
            preceded(opt_spacelike, tag(")")),
        ),
    ))(input)
}
pub fn media_slash_list_no_space(input: Span) -> PResult<Value> {
    let (input, first) = values::single(input)?;
    let (input, list) = fold_many0(
        preceded(tag("/"), values::single),
        move || vec![first.clone()],
        |mut list: Vec<Value>, item| {
            list.push(item);
            list
        },
    )(input)?;
    Ok((
        input,
        if list.len() == 1 {
            list.into_iter().next().unwrap()
        } else {
            Value::List(list, Some(ListSeparator::SlashNoSpace), false)
        },
    ))
}
