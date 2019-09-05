use super::value::value_expression;
use super::{input_to_str, input_to_string};
use crate::sass::{SassString, StringPart};
use crate::value::Quotes;
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take};
use nom::character::complete::{alphanumeric1, one_of};
use nom::combinator::{
    map, map_opt, map_res, not, opt, peek, recognize, value, verify,
};
use nom::multi::{fold_many0, fold_many1, many0, many1, many_m_n};
use nom::sequence::{delimited, pair, preceded, terminated};
use nom::IResult;

pub fn sass_string(input: &[u8]) -> IResult<&[u8], SassString> {
    let (input, parts) = many1(alt((
        string_part_interpolation,
        map(selector_string, StringPart::Raw),
    )))(input)?;
    Ok((input, SassString::new(parts, Quotes::None)))
}

pub fn sass_string_ext(input: &[u8]) -> IResult<&[u8], SassString> {
    let (input, parts) =
        many1(alt((string_part_interpolation, extended_part)))(input)?;
    Ok((input, SassString::new(parts, Quotes::None)))
}

pub fn special_args(input: &[u8]) -> IResult<&[u8], SassString> {
    let (input, parts) = special_arg_parts(input)?;
    Ok((input, SassString::new(parts, Quotes::None)))
}

pub fn special_arg_parts(input: &[u8]) -> IResult<&[u8], Vec<StringPart>> {
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
        map(map_res(is_not("#()\""), input_to_str), |s| {
            vec![StringPart::from(s)]
        }),
    )))(input)?;
    Ok((input, parts.into_iter().flatten().collect()))
}

pub fn sass_string_dq(input: &[u8]) -> IResult<&[u8], SassString> {
    let (input, parts) = dq_parts(input)?;
    Ok((input, SassString::new(parts, Quotes::Double)))
}

fn dq_parts(input: &[u8]) -> IResult<&[u8], Vec<StringPart>> {
    delimited(
        tag("\""),
        many0(alt((
            simple_qstring_part,
            string_part_interpolation,
            map(hash_no_interpolation, StringPart::from),
            value(StringPart::Raw("\"".to_string()), tag("\\\"")),
            value(StringPart::Raw("'".to_string()), tag("'")),
            extra_escape,
        ))),
        tag("\""),
    )(input)
}

pub fn sass_string_sq(input: &[u8]) -> IResult<&[u8], SassString> {
    let (input, parts) = delimited(
        tag("'"),
        many0(alt((
            simple_qstring_part,
            string_part_interpolation,
            map(hash_no_interpolation, StringPart::from),
            value(StringPart::from("'"), tag("\\'")),
            value(StringPart::from("\""), tag("\"")),
            extra_escape,
        ))),
        tag("'"),
    )(input)?;
    Ok((input, SassString::new(parts, Quotes::Single)))
}

fn string_part_interpolation(input: &[u8]) -> IResult<&[u8], StringPart> {
    let (input, expr) =
        delimited(tag("#{"), value_expression, tag("}"))(input)?;
    Ok((input, StringPart::Interpolation(expr)))
}

fn simple_qstring_part(input: &[u8]) -> IResult<&[u8], StringPart> {
    let (input, part) = map_res(is_not("\\#'\""), input_to_string)(input)?;
    Ok((input, StringPart::Raw(part)))
}

pub fn selector_string(input: &[u8]) -> IResult<&[u8], String> {
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
        String::new(),
        |mut acc: String, item: String| {
            acc.push_str(&item);
            acc
        },
    )(input)
}

fn selector_plain_part(input: &[u8]) -> IResult<&[u8], &str> {
    map_res(is_not("\n\t >$\"'\\#+*/()[]{}:;,=!&@"), input_to_str)(input)
}

fn hash_no_interpolation(input: &[u8]) -> IResult<&[u8], &str> {
    map_res(terminated(tag("#"), peek(not(tag("{")))), input_to_str)(input)
}

fn extra_escape(input: &[u8]) -> IResult<&[u8], StringPart> {
    let (input, s) = map_res(
        preceded(
            tag("\\"),
            alt((
                alphanumeric1,
                tag(b" "),
                tag("'"),
                tag("\""),
                tag("\\"),
                tag("#"),
            )),
        ),
        input_to_string,
    )(input)?;
    Ok((input, StringPart::Raw(format!("\\{}", s))))
}

pub fn extended_part(input: &[u8]) -> IResult<&[u8], StringPart> {
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

pub fn name(input: &[u8]) -> IResult<&[u8], String> {
    map_opt(
        fold_many0(
            alt((escaped_char, name_char)),
            String::new(),
            |mut s, c| {
                s.push(c);
                s
            },
        ),
        |s: String| if s != "" && s != "-" { Some(s) } else { None },
    )(input)
}

pub fn name_char(input: &[u8]) -> IResult<&[u8], char> {
    verify(take_char, is_name_char)(input)
}

fn escaped_char(input: &[u8]) -> IResult<&[u8], char> {
    preceded(
        tag("\\"),
        alt((
            value('\\', tag("\\")),
            map(
                terminated(
                    recognize(many_m_n(
                        1,
                        6,
                        one_of("0123456789ABCDEFabcdef"),
                    )),
                    opt(tag(" ")),
                ),
                |hp| {
                    std::char::from_u32(
                        u32::from_str_radix(input_to_str(hp).unwrap(), 16)
                            .unwrap(),
                    )
                    .unwrap()
                },
            ),
            take_char,
        )),
    )(input)
}

fn take_char(input: &[u8]) -> IResult<&[u8], char> {
    alt((
        map_opt(take(1usize), single_char),
        map_opt(take(2usize), single_char),
        map_opt(take(3usize), single_char),
        map_opt(take(4usize), single_char),
        map_opt(take(5usize), single_char),
    ))(input)
}

fn single_char(data: &[u8]) -> Option<char> {
    use std::str::from_utf8;
    from_utf8(&data).ok().and_then(|s| s.chars().next())
}

fn is_name_char(c: &char) -> bool {
    c.is_alphanumeric() || *c == '_' || *c == '-'
}
