use super::strings;
use super::{opt_spacelike, PResult, Span};
use crate::css::{BinOp, CallArgs, Value};
use crate::parser::value::numeric;
use crate::value::{ListSeparator, Operator};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{into, map, opt, peek, value};
use nom::multi::{fold_many0, many0, separated_list0};
use nom::sequence::{delimited, pair, preceded, terminated, tuple};

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
    let (input, first) = slash_list_no_space(input)?;
    let (input, list) = fold_many0(
        preceded(
            delimited(opt_spacelike, tag("/"), opt_spacelike),
            slash_list_no_space,
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
pub fn slash_list_no_space(input: Span) -> PResult<Value> {
    let (input, first) = space_list(input)?;
    let (input, list) = fold_many0(
        preceded(tag("/"), space_list),
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

pub fn single(input: Span) -> PResult<Value> {
    match input.first() {
        Some(b'[') => map(
            delimited(
                terminated(tag("["), opt_spacelike),
                any,
                preceded(opt_spacelike, tag("]")),
            ),
            |v| match v {
                Value::List(v, sep, false) => Value::List(v, sep, true),
                v => Value::List(vec![v], Default::default(), true),
            },
        )(input),
        Some(c) if b'0' <= *c && *c <= b'9' => into(numeric)(input),
        _ => string_or_call(input),
    }
}

fn string_or_call(input: Span) -> PResult<Value> {
    let (rest, string) = strings::css_string_any(input)?;
    if string.quotes().is_none() {
        if string.value() == "calc" {
            if let Ok((rest, args)) = delimited(
                terminated(tag("("), opt_spacelike),
                terminated(calc_expression, opt_spacelike),
                tag(")"),
            )(rest)
            {
                let args = CallArgs::from_single(args);
                return Ok((rest, Value::Call(string.take_value(), args)));
            }
        } else {
            if let Ok((rest, args)) = delimited(
                terminated(tag("("), opt_spacelike),
                terminated(call_args, opt_spacelike),
                tag(")"),
            )(rest)
            {
                return Ok((rest, Value::Call(string.take_value(), args)));
            }
        }
    }
    Ok((rest, string.into()))
}

fn calc_expression(input: Span) -> PResult<Value> {
    let (rest, first) = single_factor(input)?;
    fold_many0(
        tuple((
            delimited(
                opt_spacelike,
                alt((
                    value(Operator::Div, tag("/")),
                    value(Operator::Modulo, tag("%")),
                    value(Operator::Multiply, tag("*")),
                )),
                opt_spacelike,
            ),
            single_factor,
        )),
        move || first.clone(),
        |v, (op, v2)| BinOp::new(v, true, op, true, v2).into(),
    )(rest)
}

pub fn single_factor(input: Span) -> PResult<Value> {
    let (rest, first) = single_term(input)?;
    fold_many0(
        tuple((
            delimited(
                opt_spacelike,
                alt((
                    value(Operator::Plus, tag("+")),
                    value(Operator::Minus, tag("-")),
                )),
                opt_spacelike,
            ),
            single_term,
        )),
        move || first.clone(),
        |v, (op, v2)| BinOp::new(v, true, op, true, v2).into(),
    )(rest)
}

fn single_term(input: Span) -> PResult<Value> {
    match input.first() {
        Some(b'(') => delimited(
            terminated(tag("("), opt_spacelike),
            calc_expression,
            preceded(opt_spacelike, tag(")")),
        )(input),
        Some(c) if b'0' <= *c && *c <= b'9' => into(numeric)(input),
        _ => string_or_call(input),
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
    let (rest, positional) =
        separated_list0(terminated(tag(","), opt_spacelike), single)(rest)?;
    let (rest, trail) = opt(tag(","))(rest)?;
    Ok((
        rest,
        CallArgs {
            positional,
            named,
            trailing_comma: trail.is_some(),
        },
    ))
}
