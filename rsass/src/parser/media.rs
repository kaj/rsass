use super::css::media::relational_operator;
use super::span::Span;
use super::strings::{sass_string_dq, sass_string_sq};
use super::util::{ignore_comments, opt_spacelike, semi_or_end};
use super::value::{
    self, any_additive_expr, any_product, bracket_list, dictionary,
    function_call_or_string_rulearg, variable,
};
use super::{body_block, list_or_single, PResult};
use crate::sass::{BinOp, Item, Value};
use crate::value::ListSeparator;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{into, map, opt, value};
use nom::multi::{many0, separated_list0};
use nom::sequence::{delimited, preceded, terminated};
use nom::Parser as _;

#[cfg(test)]
use super::check_parse;

pub fn rule<'a>(start: Span, input: Span<'a>) -> PResult<'a, Item> {
    let pos = start.up_to(&input).to_owned();
    let (input, args) = opt(terminated(args, opt_spacelike)).parse(input)?;
    let (input, body) = preceded(
        opt_spacelike,
        alt((map(body_block, Some), value(None, semi_or_end))),
    )
    .parse(input)?;
    Ok((
        input,
        Item::AtMedia {
            args: args.unwrap_or(Value::Null),
            body,
            pos,
        },
    ))
}

pub fn args(input: Span) -> PResult<Value> {
    let (input, args) = separated_list0(
        preceded(tag(","), opt_spacelike),
        map(
            many0(preceded(
                opt(ignore_comments),
                alt((
                    dictionary,
                    delimited(
                        tag("("),
                        map(media_relation, |v| {
                            Value::Paren(Box::new(v), true)
                        }),
                        tag(")"),
                    ),
                    bracket_list,
                    into(value::numeric),
                    variable,
                    map(function_call_or_string_rulearg, |s| match s {
                        Value::Literal(s) => Value::Literal({
                            let lower = s
                                .single_raw()
                                .unwrap_or_default()
                                .to_lowercase();
                            if lower == "not" || lower == "only" {
                                lower.into()
                            } else {
                                s
                            }
                        }),
                        call => call,
                    }),
                    map(sass_string_dq, Value::Literal),
                    map(sass_string_sq, Value::Literal),
                )),
            )),
            |args| list_or_single(args, ListSeparator::Space),
        ),
    )
    .parse(input)?;
    Ok((input, list_or_single(args, ListSeparator::Comma)))
}

fn media_relation(input: Span) -> PResult<Value> {
    let (rest, first) = media_additive_expr(input)?;
    if let Ok((rest, (op, b))) = (
        delimited(opt_spacelike, relational_operator, opt_spacelike),
        media_relation,
    )
        .parse(rest)
    {
        let pos = input.up_to(&rest).to_owned();
        Ok((
            rest,
            Value::BinOp(Box::new(BinOp::new(first, true, op, true, b, pos))),
        ))
    } else {
        Ok((rest, first))
    }
}

fn media_additive_expr(input: Span) -> PResult<Value> {
    any_additive_expr(media_product, input)
}

fn media_product(input: Span) -> PResult<Value> {
    any_product(args, input)
}

#[test]
fn test_media_args_1() {
    check_parse(args, b"#{$media} and ($key + \"-foo\": $value + 5)")
        .unwrap();
}
#[test]
fn test_media_args_2() {
    check_parse(
        args,
        b"print and (foo: 1 2 3), (bar: 3px hux(muz)), not screen",
    )
    .unwrap();
}
