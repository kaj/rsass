//! The `calc` function is special.  A css function that is partially evaluated in sass.
//! This should apply to `min`, `max` and `clamp` as well.
//!
//! Note that function calls in actual css is different, and implementented
//! in [crate::parser::css].
use super::strings::sass_string;
use super::util::{opt_spacelike, spacelike2};
use super::value::{function_call, numeric, special_function, variable};
use super::{ignore_comments, PResult, Span};
use crate::sass::{CallArgs, Value};
use crate::value::Operator;
use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case};
use nom::character::complete::multispace0;
use nom::combinator::{into, map, not, peek, value};
use nom::sequence::{delimited, preceded, terminated, tuple};

pub fn css_function(input: Span) -> PResult<Value> {
    let (rest, arg) = delimited(
        terminated(tag_no_case("calc("), ignore_comments),
        one_arg,
        preceded(ignore_comments, tag(")")),
    )(input)?;
    let pos = input.up_to(&rest).to_owned();
    Ok((
        rest,
        Value::Call("calc".into(), CallArgs::new_single(arg), pos),
    ))
}

fn one_arg(input: Span) -> PResult<Value> {
    alt((
        map(delimited(tag("("), one_arg, tag(")")), |v| {
            Value::Paren(v.into(), true)
        }),
        sum_expression,
        map(sass_string, Value::Literal),
    ))(input)
}

fn sum_expression(input: Span) -> PResult<Value> {
    let (mut rest, mut v) = term(input)?;
    while let Ok((nrest, (s1, op, s2, v2))) = alt((
        tuple((
            value(false, tag("")),
            alt((
                value(Operator::Plus, tag("+")),
                value(Operator::Minus, tag("-")),
            )),
            ignore_comments,
            term,
        )),
        tuple((
            value(true, spacelike2),
            alt((
                value(Operator::Plus, tag("+")),
                value(Operator::Minus, terminated(tag("-"), spacelike2)),
            )),
            ignore_comments,
            term,
        )),
    ))(rest)
    {
        v = Value::BinOp(Box::new(v), s1, op, s2, Box::new(v2));
        rest = nrest;
    }
    Ok((rest, v))
}

fn term(input: Span) -> PResult<Value> {
    let (mut rest, mut v) = single_value(input)?;
    while let Ok((nrest, (s1, op, s2, v2))) = tuple((
        map(multispace0, |s: Span| !s.fragment().is_empty()),
        alt((
            value(Operator::Multiply, tag("*")),
            value(Operator::Div, terminated(tag("/"), peek(not(tag("/"))))),
            value(Operator::Modulo, tag("%")),
        )),
        map(multispace0, |s: Span| !s.fragment().is_empty()),
        single_value,
    ))(rest)
    {
        rest = nrest;
        v = Value::BinOp(Box::new(v), s1, op, s2, Box::new(v2));
    }
    Ok((rest, v))
}

fn single_value(input: Span) -> PResult<Value> {
    alt((
        paren,
        value(Value::True, tag("true")),
        value(Value::False, tag("false")),
        value(Value::HereSelector, tag("&")),
        into(numeric),
        variable,
        value(Value::Null, tag("null")),
        special_function,
        function_call,
    ))(input)
}

fn paren(input: Span) -> PResult<Value> {
    map(
        delimited(
            terminated(tag("("), opt_spacelike),
            sum_expression,
            preceded(opt_spacelike, tag(")")),
        ),
        |inner| Value::Paren(Box::new(inner), false),
    )(input)
}
