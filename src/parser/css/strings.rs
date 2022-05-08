use super::super::{input_to_str, input_to_string, PResult, Span};
use crate::css::CssString;
use crate::value::Quotes;
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take};
use nom::character::complete::one_of;
use nom::combinator::{
    into, map, map_opt, map_res, not, opt, peek, recognize, value,
};
use nom::multi::{fold_many0, many0, many_m_n};
use nom::sequence::{delimited, preceded, terminated};
use std::str::from_utf8;

pub fn css_string_any(input: Span) -> PResult<CssString> {
    alt((css_string_dq, css_string_sq, into(css_string)))(input)
}

pub fn css_string(input: Span) -> PResult<String> {
    let (input, first) = alt((
        map(selector_plain_part, String::from),
        normalized_first_escaped_char,
        map(hash_no_interpolation, String::from),
    ))(input)?;
    fold_many0(
        // Note: This could probably be a whole lot more efficient,
        // but try to get stuff correct before caring too much about that.
        alt((
            map(selector_plain_part, String::from),
            normalized_escaped_char,
            map(hash_no_interpolation, String::from),
        )),
        move || first.clone(),
        |mut acc: String, item: String| {
            acc.push_str(&item);
            acc
        },
    )(input)
}

pub fn css_string_dq(input: Span) -> PResult<CssString> {
    let (input, parts) = delimited(
        tag("\""),
        many0(alt((
            map_res(is_not("\""), input_to_string),
            value("\"".to_string(), tag("\\\"")),
            normalized_escaped_char_q,
        ))),
        tag("\""),
    )(input)?;
    Ok((input, CssString::new(parts.join(""), Quotes::Double)))
}

pub fn css_string_sq(input: Span) -> PResult<CssString> {
    let (input, parts) = delimited(
        tag("'"),
        many0(alt((
            map_res(is_not("'"), input_to_string),
            value(String::from("'"), tag("\\'")),
            normalized_escaped_char_q,
        ))),
        tag("'"),
    )(input)?;
    Ok((input, CssString::new(parts.join(""), Quotes::Single)))
}

fn normalized_first_escaped_char(input: Span) -> PResult<String> {
    let (rest, c) = escaped_char(input)?;
    let result = if c.is_alphabetic() || u32::from(c) >= 0xa1 {
        format!("{}", c)
    } else if !c.is_control() && !c.is_numeric() && c != '\n' && c != '\t' {
        format!("\\{}", c)
    } else {
        format!("\\{:x} ", u32::from(c))
    };
    Ok((rest, result))
}
fn normalized_escaped_char(input: Span) -> PResult<String> {
    let (rest, c) = escaped_char(input)?;
    let result = if c.is_alphanumeric() || c == '-' || u32::from(c) >= 0xa1 {
        format!("{}", c)
    } else if !c.is_control() && c != '\n' && c != '\t' {
        format!("\\{}", c)
    } else {
        format!("\\{:x} ", u32::from(c))
    };
    Ok((rest, result))
}

fn normalized_escaped_char_q(input: Span) -> PResult<String> {
    let (rest, c) = escaped_char(input)?;
    let result = if c == '\0' {
        "\u{fffd}".to_string()
    } else if c.is_control() && c != '\t' {
        format!("\\{:x} ", u32::from(c))
    } else if c == '-' || c == '\\' || c == ' ' {
        format!("\\{}", c)
    } else {
        c.to_string()
    };
    Ok((rest, result))
}

fn selector_plain_part(input: Span) -> PResult<&str> {
    map_res(is_not("\r\n\t >$\"'\\#+*/()[]{}:;,=!&@"), input_to_str)(input)
}

fn hash_no_interpolation(input: Span) -> PResult<&str> {
    map_res(terminated(tag("#"), peek(not(tag("{")))), input_to_str)(input)
}

fn escaped_char(input: Span) -> PResult<char> {
    preceded(
        tag("\\"),
        alt((map_opt(hex_number, std::char::from_u32), take_char)),
    )(input)
}

fn hex_number(input: Span) -> PResult<u32> {
    map_res(
        map_res(
            terminated(
                recognize(many_m_n(1, 6, one_of("0123456789ABCDEFabcdef"))),
                opt(tag(" ")),
            ),
            input_to_str,
        ),
        |s| u32::from_str_radix(s, 16),
    )(input)
}

fn take_char(input: Span) -> PResult<char> {
    alt((
        map_opt(take(1usize), single_char),
        map_opt(take(2usize), single_char),
        map_opt(take(3usize), single_char),
        map_opt(take(4usize), single_char),
        map_opt(take(5usize), single_char),
    ))(input)
}

fn single_char(data: Span) -> Option<char> {
    from_utf8(data.fragment())
        .ok()
        .and_then(|s| s.chars().next())
}
