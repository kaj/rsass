use super::strings;
use super::{opt_spacelike, PResult, Span};
use crate::css::{CallArgs, Value};
use crate::parser::value::number;
use crate::value::ListSeparator;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{opt, peek};
use nom::multi::{fold_many0, many0};
use nom::sequence::{delimited, pair, preceded, terminated};

pub fn any(input: Span) -> PResult<Value> {
    let (input, first) = slash_list(input)?;
    let (input, list) = fold_many0(
        preceded(
            delimited(opt_spacelike, tag(","), opt_spacelike),
            slash_list,
        ),
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
            Value::List(list, Some(ListSeparator::Comma), false)
        },
    ))
}
pub fn slash_list(input: Span) -> PResult<Value> {
    let (input, first) = space_list(input)?;
    let (input, list) = fold_many0(
        preceded(
            delimited(opt_spacelike, tag("/"), opt_spacelike),
            space_list,
        ),
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
            Value::List(list, Some(ListSeparator::Slash), false)
        },
    ))
}
pub fn space_list(input: Span) -> PResult<Value> {
    let (input, first) = single(input)?;
    let (input, list) = fold_many0(
        preceded(opt_spacelike, single),
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
            Value::List(list, Some(ListSeparator::Space), false)
        },
    ))
}

fn single(input: Span) -> PResult<Value> {
    if let Ok((rest, num)) = number(input) {
        return Ok((rest, num.into()));
    }
    let (rest, string) = strings::css_string_any(input)?;
    if let Ok((rest, args)) = delimited(
        terminated(tag("("), opt_spacelike),
        opt(terminated(call_args, opt_spacelike)),
        tag(")"),
    )(rest)
    {
        Ok((
            rest,
            Value::Call(string.to_string(), args.unwrap_or_default()),
        ))
    } else {
        Ok((rest, string.into()))
    }
}

fn call_args(input: Span) -> PResult<CallArgs> {
    let (rest, named) = many0(pair(
        terminated(
            strings::css_string,
            delimited(opt_spacelike, tag("="), opt_spacelike),
        ),
        terminated(
            single,
            alt((terminated(tag(","), opt_spacelike), peek(tag(")")))),
        ),
    ))(input)?;
    let named = named
        .into_iter()
        .map(|(name, val)| (name.into(), val))
        .collect();
    let (rest, positional) = many0(terminated(
        single,
        alt((terminated(tag(","), opt_spacelike), peek(tag(")")))),
    ))(rest)?;

    Ok((
        rest,
        CallArgs {
            positional,
            named,
            trailing_comma: false,
        },
    ))
}
