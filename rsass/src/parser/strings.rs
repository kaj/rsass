use super::value::value_expression;
use super::{input_to_str, input_to_string, PResult, Span};
use crate::sass::{SassString, StringPart};
use crate::value::Quotes;
use nom::branch::alt;
use nom::bytes::complete::{is_a, is_not, tag, take};
use nom::character::complete::{alphanumeric1, char, one_of};
use nom::combinator::{
    cut, map, map_opt, map_res, not, opt, peek, recognize, value, verify,
};
use nom::error::context;
use nom::multi::{fold_many0, fold_many1, many0, many1, many_m_n};
use nom::sequence::{delimited, pair, preceded, terminated, tuple};
use std::str::from_utf8;

pub fn sass_string(input: Span) -> PResult<SassString> {
    verify(sass_string_allow_dash, |s| {
        s.single_raw().map_or(true, |s| s != "-")
    })(input)
}

pub fn sass_string_allow_dash(input: Span) -> PResult<SassString> {
    let (input, first) = alt((
        string_part_interpolation,
        map(unquoted_first_part, StringPart::Raw),
    ))(input)?;
    let (input, parts) = fold_many0(
        alt((
            string_part_interpolation,
            map(unquoted_part, StringPart::Raw),
        )),
        || vec![first.clone()],
        |mut acc, item| {
            acc.push(item);
            acc
        },
    )(input)?;
    Ok((input, SassString::new(parts, Quotes::None)))
}

pub fn custom_value(input: Span) -> PResult<SassString> {
    map(
        context("Expected token.", custom_value_inner),
        |mut parts| {
            if let Some(StringPart::Raw(last)) = parts.last_mut() {
                if last.ends_with('\n') {
                    last.pop();
                    last.push(' ');
                }
            }
            SassString::new(parts, Quotes::None)
        },
    )(input)
}
pub fn custom_value_inner(input: Span) -> PResult<Vec<StringPart>> {
    fold_many1(
        alt((
            |input| custom_value_paren('[', ']', input),
            |input| custom_value_paren('{', '}', input),
            |input| custom_value_paren('(', ')', input),
            map(sass_string_dq, |mut s| {
                s.prepend("\"");
                s.append_str("\"");
                s.parts()
            }),
            map(sass_string_sq, |mut s| {
                s.prepend("'");
                s.append_str("'");
                s.parts()
            }),
            map(string_part_interpolation, |s| vec![s]),
            map(unquoted_part, |s| vec![StringPart::Raw(s)]),
            map_opt(is_not("\"'#\\;{}()[]"), |s: Span| {
                if s.is_empty() {
                    None
                } else {
                    Some(vec![StringPart::from(input_to_str(s).ok()?)])
                }
            }),
        )),
        Vec::new,
        |mut acc, items: Vec<StringPart>| {
            acc.extend(items);
            acc
        },
    )(input)
}

fn custom_value_paren(
    start: char,
    end: char,
    input: Span,
) -> PResult<Vec<StringPart>> {
    map(
        delimited(
            char(start),
            fold_many0(
                alt((
                    map(tag(";"), |_| vec![StringPart::from(";")]),
                    custom_value_inner,
                )),
                || vec![StringPart::Raw(start.into())],
                |mut acc, items: Vec<StringPart>| {
                    acc.extend(items);
                    acc
                },
            ),
            cut(char(end)),
        ),
        |mut parts| {
            parts.push(StringPart::Raw(end.into()));
            parts
        },
    )(input)
}

pub fn sass_string_ext(input: Span) -> PResult<SassString> {
    let (input, parts) =
        many1(alt((string_part_interpolation, extended_part)))(input)?;
    Ok((input, SassString::new(parts, Quotes::None)))
}

/// An unquoted string that may contain the `$` sign.
pub fn sass_string_ext2(input: Span) -> PResult<SassString> {
    let (input, parts) =
        many0(alt((string_part_interpolation, extended_part2)))(input)?;
    Ok((input, SassString::new(parts, Quotes::None)))
}

fn unquoted_first_part(input: Span) -> PResult<String> {
    let (input, first) = alt((
        map(str_plain_part, String::from),
        normalized_first_escaped_char,
        map(hash_no_interpolation, String::from),
    ))(input)?;
    fold_many0(
        // Note: This could probably be a whole lot more efficient,
        // but try to get stuff correct before caring too much about that.
        alt((
            map(str_plain_part, String::from),
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
fn unquoted_part(input: Span) -> PResult<String> {
    fold_many1(
        // Note: This could probably be a whole lot more efficient,
        // but try to get stuff correct before caring too much about that.
        alt((
            map(str_plain_part, String::from),
            normalized_escaped_char,
            map(hash_no_interpolation, String::from),
        )),
        String::new,
        |mut acc: String, item: String| {
            acc.push_str(&item);
            acc
        },
    )(input)
}

fn normalized_first_escaped_char(input: Span) -> PResult<String> {
    let (rest, c) = escaped_char(input)?;
    let result = if c.is_alphabetic() || u32::from(c) >= 0xa1 {
        format!("{c}")
    } else if !c.is_control() && !c.is_numeric() && c != '\n' && c != '\t' {
        format!("\\{c}")
    } else {
        format!("\\{:x} ", u32::from(c))
    };
    Ok((rest, result))
}
fn normalized_escaped_char(input: Span) -> PResult<String> {
    let (rest, c) = escaped_char(input)?;
    let result = if c.is_alphanumeric() || c == '-' || u32::from(c) >= 0xa1 {
        format!("{c}")
    } else if !c.is_control() && c != '\n' && c != '\t' {
        format!("\\{c}")
    } else {
        format!("\\{:x} ", u32::from(c))
    };
    Ok((rest, result))
}

/// Some special css functions are parsed as plain strings.
///
/// For the `calc` function, it is parsed as a string if the arguments
/// contain a string interpolation, otherwise it is refused by this parser,
/// so that it can end up beeing parsed as a normal function call.
pub fn special_function_misc(input: Span) -> PResult<SassString> {
    let (input, (start, mut args, end)) = verify(
        tuple((
            recognize(terminated(
                alt((
                    map(
                        tuple((
                            opt(delimited(tag("-"), alphanumeric1, tag("-"))),
                            alt((
                                tag("calc"),
                                tag("element"),
                                tag("env"),
                                tag("expression"),
                            )),
                        )),
                        |_| (),
                    ),
                    map(
                        preceded(
                            tag("progid:"),
                            many1(alt((
                                map(tag("."), |_| ()),
                                map(selector_string, |_| ()),
                            ))),
                        ),
                        |_| (),
                    ),
                )),
                tag("("),
            )),
            special_args,
            alt((tag(")"), tag(""))),
        )),
        |(start, args, _end)| {
            start.fragment() != b"calc(" || args.is_interpolated()
        },
    )(input)?;

    args.prepend(from_utf8(start.fragment()).unwrap());
    args.append_str(from_utf8(end.fragment()).unwrap());
    Ok((input, args))
}

fn special_args(input: Span) -> PResult<SassString> {
    let (input, parts) = special_arg_parts(input)?;
    Ok((input, SassString::new(parts, Quotes::None)))
}
pub fn special_arg_parts(input: Span) -> PResult<Vec<StringPart>> {
    let (input, parts) = many0(alt((
        map(string_part_interpolation, |part| vec![part]),
        map(hash_no_interpolation, |s| vec![StringPart::from(s)]),
        map(dq_parts, |mut v| {
            v.insert(0, StringPart::from("\""));
            v.push(StringPart::from("\""));
            v
        }),
        map(delimited(tag("("), special_arg_parts, tag(")")), |mut v| {
            v.insert(0, StringPart::from("("));
            v.push(StringPart::from(")"));
            v
        }),
        map(tag("\\)"), |_| vec![StringPart::from("\\)")]),
        value(vec![StringPart::from(" ")], is_a("\n ")),
        map(map_res(is_not("#()\"\\;\n "), input_to_str), |s| {
            vec![StringPart::from(s)]
        }),
    )))(input)?;
    Ok((input, parts.into_iter().flatten().collect()))
}

pub fn special_url(input: Span) -> PResult<SassString> {
    let (input, _start) = tag("url(")(input)?;
    let (input, _trim) = many0(is_a(" "))(input)?;
    let (input, mut parts) = many1(alt((
        string_part_interpolation,
        map(unquoted_part, StringPart::Raw),
        map(
            map_res(is_a("\":.;,!+/="), input_to_string),
            StringPart::Raw,
        ),
    )))(input)?;
    let (input, _trim) = many0(is_a(" "))(input)?;
    let (input, _end) = tag(")")(input)?;
    parts.insert(0, "url(".into());
    parts.push(")".into());
    Ok((input, SassString::new(parts, Quotes::None)))
}

pub fn sass_string_dq(input: Span) -> PResult<SassString> {
    let (input, mut parts) = dq_parts(input)?;
    cleanup_escape_ws(&mut parts);
    Ok((input, SassString::new(parts, Quotes::Double)))
}

fn dq_parts(input: Span) -> PResult<Vec<StringPart>> {
    delimited(
        tag("\""),
        many0(alt((
            simple_qstring_part,
            string_part_interpolation,
            map(hash_no_interpolation, StringPart::from),
            value(StringPart::Raw("\"".to_string()), tag("\\\"")),
            value(StringPart::Raw("'".to_string()), tag("'")),
            map(normalized_escaped_char_q, StringPart::Raw),
        ))),
        tag("\""),
    )(input)
}

pub fn sass_string_sq(input: Span) -> PResult<SassString> {
    let (input, mut parts) = delimited(
        tag("'"),
        many0(alt((
            simple_qstring_part,
            string_part_interpolation,
            map(hash_no_interpolation, StringPart::from),
            value(StringPart::from("'"), tag("\\'")),
            value(StringPart::from("\""), tag("\"")),
            value(StringPart::from(""), tag("\\\n")),
            map(normalized_escaped_char_q, StringPart::Raw),
        ))),
        tag("'"),
    )(input)?;
    cleanup_escape_ws(&mut parts);
    Ok((input, SassString::new(parts, Quotes::Single)))
}

fn cleanup_escape_ws(parts: &mut [StringPart]) {
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
}

fn normalized_escaped_char_q(input: Span) -> PResult<String> {
    let (rest, c) = escaped_char(input)?;
    let result = if c == '\0' {
        char::REPLACEMENT_CHARACTER.to_string()
    } else if c.is_control() && c != '\t' {
        format!("\\{:x} ", u32::from(c))
    } else if c == '-' || c == '\\' || c == ' ' {
        format!("\\{c}")
    } else {
        c.to_string()
    };
    Ok((rest, result))
}

pub fn string_part_interpolation(input: Span) -> PResult<StringPart> {
    let (input, expr) =
        delimited(tag("#{"), value_expression, tag("}"))(input)?;
    Ok((input, StringPart::Interpolation(expr)))
}

fn simple_qstring_part(input: Span) -> PResult<StringPart> {
    let (input, part) =
        map_res(is_not("\\#'\"\n\r"), input_to_string)(input)?;
    Ok((input, StringPart::Raw(part)))
}

fn selector_string(input: Span) -> PResult<String> {
    fold_many1(
        // Note: This could probably be a whole lot more efficient,
        // but try to get stuff correct before caring too much about that.
        alt((
            selector_plain_part,
            map(tag("\\ "), |_| "\\ ".to_string()),
            map(tag("\\\""), |_| "\\\"".to_string()),
            map(tag("\\\'"), |_| "\\\'".to_string()),
            map(tag("\\\\"), |_| "\\\\".to_string()),
            map(escaped_char, |c| format!("{c}")),
            map(hash_no_interpolation, String::from),
        )),
        String::new,
        |mut acc: String, item: String| {
            acc.push_str(&item);
            acc
        },
    )(input)
}

fn selector_plain_part(input: Span) -> PResult<String> {
    fold_many1(
        verify(take_char, |ch| {
            ch.is_alphanumeric() || *ch == '-' || *ch == '_'
        }),
        String::new,
        |mut acc, chr: char| {
            acc.push(chr);
            acc
        },
    )(input)
}

fn str_plain_part(input: Span) -> PResult<&str> {
    // TODO: This should probably be based on unicode alphanumeric.
    map_res(is_not("\r\n\t %<>$\"'\\#+*/()[]{}:;,=!&@~"), input_to_str)(input)
}

fn hash_no_interpolation(input: Span) -> PResult<&str> {
    map_res(terminated(tag("#"), peek(not(tag("{")))), input_to_str)(input)
}

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

fn extended_part2(input: Span) -> PResult<StringPart> {
    let (input, part) = map_res(
        recognize(pair(
            verify(take_char, |c| is_ext_str_start_char(c) || *c == '$'),
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
        || *c == '<'
        || *c == '>'
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
        || *c == '<'
        || *c == '>'
        || *c == '\\'
}

pub fn name(input: Span) -> PResult<String> {
    let (input, first) =
        verify(alt((escaped_char, take_char)), is_name_start_char)(input)?;
    verify(
        fold_many0(
            alt((escaped_char, name_char)),
            move || String::from(first),
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

fn escaped_char(input: Span) -> PResult<char> {
    preceded(
        tag("\\"),
        alt((
            value('\\', tag("\\")),
            map_opt(
                pair(
                    recognize(many_m_n(
                        1,
                        6,
                        one_of("0123456789ABCDEFabcdef"),
                    )),
                    alt((
                        value(true, tag(" ")),
                        value(
                            true,
                            peek(not(one_of("0123456789ABCDEFabcdef"))),
                        ),
                        value(false, tag("")),
                    )),
                ),
                |(code, term): (Span, bool)| {
                    if term || code.len() == 6 {
                        std::char::from_u32(
                            u32::from_str_radix(input_to_str(code).ok()?, 16)
                                .ok()?,
                        )
                    } else {
                        None
                    }
                },
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

fn is_name_char(c: &char) -> bool {
    c.is_alphanumeric() || *c == '_' || *c == '-'
}
fn is_name_start_char(c: &char) -> bool {
    c.is_alphabetic() || *c == '_' || *c == '-'
}
