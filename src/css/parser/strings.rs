use super::super::CssString;
use crate::parser::{input_to_str, input_to_string, PResult, Span};
use crate::value::Quotes;
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take};
use nom::character::complete::one_of;
use nom::combinator::{
    map, map_opt, map_res, not, opt, peek, recognize, value,
};
use nom::multi::{fold_many0, many0, many_m_n};
use nom::sequence::{delimited, preceded, terminated};
use std::str::from_utf8;

pub fn css_string_any(input: Span) -> PResult<CssString> {
    alt((css_string_dq, css_string_sq, css_string))(input)
}

pub fn css_string(input: Span) -> PResult<CssString> {
    let (input, first) = unquoted_first_part(input)?;
    Ok((input, CssString::new(first, Quotes::None)))
}

/*
pub fn sass_string_ext(input: Span) -> PResult<SassString> {
    let (input, parts) =
        many1(extended_part)(input)?;
    Ok((input, SassString::new(parts, Quotes::None)))
}
 */

fn unquoted_first_part(input: Span) -> PResult<String> {
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
/*fn unquoted_part(input: Span) -> PResult<String> {
    fold_many1(
        // Note: This could probably be a whole lot more efficient,
        // but try to get stuff correct before caring too much about that.
        alt((
            map(selector_plain_part, String::from),
            normalized_escaped_char,
            map(hash_no_interpolation, String::from),
        )),
        String::new,
        |mut acc: String, item: String| {
            acc.push_str(&item);
            acc
        },
    )(input)
}*/

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

pub fn css_string_dq(input: Span) -> PResult<CssString> {
    let (input, parts) = dq_parts(input)?;
    Ok((input, CssString::new(parts.join(""), Quotes::Double)))
}

fn dq_parts(input: Span) -> PResult<Vec<String>> {
    delimited(
        tag("\""),
        many0(alt((
            simple_qstring_part,
            map(hash_no_interpolation, String::from),
            value("\"".to_string(), tag("\\\"")),
            value("'".to_string(), tag("'")),
            normalized_escaped_char_q,
        ))),
        tag("\""),
    )(input)
}

pub fn css_string_sq(input: Span) -> PResult<CssString> {
    let (input, parts) = delimited(
        tag("'"),
        many0(alt((
            simple_qstring_part,
            map(hash_no_interpolation, String::from),
            value(String::from("'"), tag("\\'")),
            value(String::from("\""), tag("\"")),
            normalized_escaped_char_q,
        ))),
        tag("'"),
    )(input)?;
    Ok((input, CssString::new(parts.join(""), Quotes::Single)))
}

/*fn cleanup_escape_ws(parts: &mut [String]) {
    let mut t_iter = parts.iter_mut().peekable();
    while let Some(ref mut item) = t_iter.next() {
        if let StringPart::Raw(ref mut s) = item {
            if s.starts_with('\\') && s.ends_with(' ') {
                match t_iter.peek() {
                    None => {
                        s.pop();
                    }
                    Some(StringPart::Raw(next)) => {
                        if let Some(next) = next.chars().next() {
                            if !next.is_ascii_hexdigit() && next != '\t' {
                                s.pop();
                            }
                        }
                    }
                    _ => (),
                }
            }
        }
    }
}*/

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

fn simple_qstring_part(input: Span) -> PResult<String> {
    map_res(is_not("\\#'\""), input_to_string)(input)
}

/*fn selector_string(input: Span) -> PResult<String> {
    fold_many1(
        // Note: This could probably be a whole lot more efficient,
        // but try to get stuff correct before caring too much about that.
        alt((
            map(selector_plain_part, String::from),
            map(tag("\\ "), |_| "\\ ".to_string()),
            map(tag("\\\""), |_| "\\\"".to_string()),
            map(tag("\\\'"), |_| "\\\'".to_string()),
            map(tag("\\\\"), |_| "\\\\".to_string()),
            map(escaped_char, |c| format!("{}", c)),
            map(hash_no_interpolation, String::from),
        )),
        String::new,
        |mut acc: String, item: String| {
            acc.push_str(&item);
            acc
        },
    )(input)
}*/

fn selector_plain_part(input: Span) -> PResult<&str> {
    map_res(is_not("\r\n\t >$\"'\\#+*/()[]{}:;,=!&@"), input_to_str)(input)
}

fn hash_no_interpolation(input: Span) -> PResult<&str> {
    map_res(terminated(tag("#"), peek(not(tag("{")))), input_to_str)(input)
}

/*
pub fn extended_part(input: Span) -> PResult<StringPart> {
    let (input, part) = map_res(
        recognize(pair(
            verify(take_char, is_ext_str_start_char),
            many0(verify(take_char, is_ext_str_char)),
        )),
        input_to_string,
    )(input)?;
    Ok((input, StringPart::Raw(part)))
}

fn is_ext_str_start_char(c: &char) -> bool {
    is_name_char(c)
        || *c == '*'
        || *c == '+'
        || *c == '.'
        || *c == '/'
        || *c == ':'
        || *c == '='
        || *c == '?'
        || *c == '|'
}
fn is_ext_str_char(c: &char) -> bool {
    is_name_char(c)
        || *c == '*'
        || *c == '+'
        || *c == ','
        || *c == '.'
        || *c == '/'
        || *c == ':'
        || *c == '='
        || *c == '?'
        || *c == '|'
}
 */
/*
pub fn name(input: Span) -> PResult<String> {
    verify(
        fold_many0(
            alt((escaped_char, name_char)),
            String::new,
            |mut s, c| {
                s.push(c);
                s
            },
        ),
        |s: &str| !s.is_empty() && s != "-",
    )(input)
}

pub fn unitname(input: Span) -> PResult<String> {
    let (input, first) =
        verify(alt((escaped_char, name_char)), |c| c.is_alphabetic())(input)?;
    fold_many0(
        verify(alt((escaped_char, name_char)), |c| c.is_alphanumeric()),
        move || first.to_string(),
        |mut s, c| {
            s.push(c);
            s
        },
    )(input)
}
pub fn name_char(input: Span) -> PResult<char> {
    verify(take_char, is_name_char)(input)
}
*/

fn escaped_char(input: Span) -> PResult<char> {
    preceded(
        tag("\\"),
        alt((
            value('\\', tag("\\")),
            map_opt(
                map_res(
                    map_res(
                        terminated(
                            recognize(many_m_n(
                                1,
                                6,
                                one_of("0123456789ABCDEFabcdef"),
                            )),
                            opt(tag(" ")),
                        ),
                        input_to_str,
                    ),
                    |s| u32::from_str_radix(s, 16),
                ),
                std::char::from_u32,
            ),
            take_char,
        )),
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
/*
fn is_name_char(c: &char) -> bool {
    c.is_alphanumeric() || *c == '_' || *c == '-'
}
*/
