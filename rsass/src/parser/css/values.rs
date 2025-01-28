use super::{nom_err, strings};
use super::{opt_spacelike, PResult, Span};
use crate::css::{BinOp, CallArgs, Value};
use crate::parser::input_to_str;
use crate::parser::value::{numeric, unicode_range_inner};
use crate::value::{ListSeparator, Operator};
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{char, none_of, one_of};
use nom::combinator::{
    cond, into, map, map_opt, not, opt, peek, recognize, value,
};
use nom::error::context;
use nom::multi::{fold_many0, many0, separated_list0, separated_list1};
use nom::sequence::{delimited, pair, preceded, terminated};
use nom::Parser;
use nom_language::error::VerboseError;

pub fn any(input: Span) -> PResult<Value> {
    let (input, list) =
        separated_list1(spaced(","), slash_list).parse(input)?;
    Ok((input, list_or_single(list, ListSeparator::Comma)))
}
pub fn slash_list(input: Span) -> PResult<Value> {
    let (input, list) =
        separated_list1(spaced("/"), slash_list_no_space).parse(input)?;
    Ok((input, list_or_single(list, ListSeparator::Slash)))
}
pub fn slash_list_no_space(input: Span) -> PResult<Value> {
    let (mut input, first) = space_list(input)?;
    let mut list = vec![first];
    while let PResult::Ok((rest, _)) =
        terminated(tag("/"), peek(not(tag("*")))).parse(input)
    {
        let (rest, value) =
            alt((map(space_list, Some), value(None, opt_spacelike)))
                .parse(rest)?;
        list.push(value.unwrap_or(Value::Literal("".into())));
        input = rest;
    }
    Ok((input, list_or_single(list, ListSeparator::SlashNoSpace)))
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
    )
    .parse(input)?;
    Ok((input, list_or_single(list, ListSeparator::Space)))
}

fn list_or_single(list: Vec<Value>, sep: ListSeparator) -> Value {
    if list.len() == 1 {
        list.into_iter().next().unwrap()
    } else {
        Value::List(list, Some(sep), false)
    }
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
                v => Value::List(vec![v], None, true),
            },
        )
        .parse(input),
        Some(c) if c.is_ascii_digit() => into(numeric).parse(input),
        Some(c) if *c == b'-' || *c == b'.' => {
            alt((into(numeric), string_or_call)).parse(input)
        }
        Some(b'(') => {
            let (end, _) = delimited(tag("("), none_of(")"), opt(tag(")")))
                .parse(input)?;
            let pos = input.up_to(&end);
            Err(nom_err("Parentheses aren't allowed in plain CSS.", pos))
        }
        Some(b'$') => {
            let (end, _) =
                preceded(tag("$"), strings::css_string).parse(input)?;
            let pos = input.up_to(&end);
            Err(nom_err("Sass variables aren't allowed in plain CSS.", pos))
        }
        _ => alt((
            map(unicode_range_inner, Value::UnicodeRange),
            string_or_call,
        ))
        .parse(input),
    }
}

pub fn string_or_call(input: Span) -> PResult<Value> {
    let (rest, string) = strings::css_string_any(input)?;
    if string.quotes().is_none() {
        if let Ok((rest, _)) = terminated(tag("("), opt_spacelike).parse(rest)
        {
            fn endp(input: Span) -> PResult<()> {
                terminated(opt_spacelike, char(')')).parse(input)
            }
            let (rest, args) = if string.value() == "calc" {
                if let Ok((end, _)) = endp(rest) {
                    return Err(nom_err(
                        "Missing argument.",
                        input.up_to(&end),
                    ));
                }
                map(terminated(calc_expr, endp), CallArgs::from_single)
                    .parse(rest)?
            } else {
                terminated(call_args, endp).parse(rest)?
            };
            return Ok((rest, Value::Call(string.take_value(), args)));
        } else if let Ok((end, _)) =
            preceded(char('.'), string_or_call).parse(rest)
        {
            return Err(nom_err(
                "Module namespaces aren't allowed in plain CSS.",
                input.up_to(&end),
            ));
        }
    }
    Ok((rest, string.into()))
}

fn calc_expr(input: Span) -> PResult<Value> {
    let (rest, first) = single_factor(input)?;
    fold_many0(
        (
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
        ),
        move || first.clone(),
        |v, (op, v2)| BinOp::new(v, true, op, true, v2).into(),
    )
    .parse(rest)
}

pub fn single_factor(input: Span) -> PResult<Value> {
    let (rest, first) = single_term(input)?;
    fold_many0(
        (
            delimited(
                opt_spacelike,
                alt((
                    value(Operator::Plus, tag("+")),
                    value(Operator::Minus, tag("-")),
                )),
                opt_spacelike,
            ),
            single_term,
        ),
        move || first.clone(),
        |v, (op, v2)| BinOp::new(v, true, op, true, v2).into(),
    )
    .parse(rest)
}

fn single_term(input: Span) -> PResult<Value> {
    match input.first() {
        Some(b'(') => delimited(
            terminated(tag("("), opt_spacelike),
            calc_expr,
            preceded(opt_spacelike, tag(")")),
        )
        .parse(input),
        Some(b'$') => {
            let (end, _) =
                preceded(tag("$"), strings::css_string).parse(input)?;
            let pos = input.up_to(&end);
            Err(nom_err("Sass variables aren't allowed in plain CSS.", pos))
        }
        Some(c) if b'0' <= *c && *c <= b'9' => into(numeric).parse(input),
        _ => string_or_call(input),
    }
}

fn call_args(input: Span) -> PResult<CallArgs> {
    let (rest, named) = many0(pair(
        terminated(strings::css_string, spaced("=")),
        terminated(single, alt((spaced(","), peek(tag(")"))))),
    ))
    .parse(input)?;
    let named = named
        .into_iter()
        .map(|(name, val)| (name.into(), val))
        .collect();
    let (rest, positional) =
        separated_list0(spaced(","), single_arg).parse(rest)?;
    let (rest, trailing_comma) =
        map(opt(spaced(",")), |c| c.is_some()).parse(rest)?;

    let (rest, _) = cond(
        trailing_comma,
        alt((
            peek(tag(")")),
            context("Expected expression.", recognize(single_arg)),
        )),
    )
    .parse(rest)?;
    Ok((
        rest,
        CallArgs {
            positional,
            named,
            trailing_comma,
        },
    ))
}

fn single_arg(input: Span) -> PResult<Value> {
    fn end(input: Span) -> PResult<()> {
        peek(preceded(opt_spacelike, map(one_of(",)."), |_| ()))).parse(input)
    }
    terminated(space_list, end).parse(input).or_else(|err| {
        terminated(into(ext_arg_as_string), end)
            .parse(input)
            .map_err(|_| err)
    })
}

fn ext_arg_as_string(input: Span) -> PResult<String> {
    map_opt(is_not("\"\\;{}()[] ,"), |s: Span| {
        if s.first().map_or(true, |ch| ch.is_ascii_digit()) {
            None
        } else {
            Some(input_to_str(s).ok()?.to_owned())
        }
    })
    .parse(input)
}

fn spaced<'a>(
    the_tag: &'static str,
) -> impl Parser<Span<'a>, Output = Span<'a>, Error = VerboseError<Span<'a>>>
{
    delimited(opt_spacelike, tag(the_tag), opt_spacelike)
}
