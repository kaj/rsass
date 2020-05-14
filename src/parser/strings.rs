use super::value::value_expression;
use super::{input_to_str, input_to_string};
use crate::sass::{SassString, StringPart};
use crate::value::Quotes;
use nom::branch::alt;
use nom::bytes::complete::{is_a, is_not, tag, tag_no_case, take};
use nom::character::complete::{alphanumeric1, one_of};
use nom::combinator::{
    map, map_opt, map_res, not, opt, peek, recognize, value, verify,
};
use nom::multi::{fold_many0, fold_many1, many0, many1, many_m_n};
use nom::sequence::{delimited, pair, preceded, terminated, tuple};
use nom::IResult;
use std::str::from_utf8;

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

pub fn special_function_misc(input: &[u8]) -> IResult<&[u8], SassString> {
    let (input, start) = recognize(terminated(
        alt((
            map(
                tuple((
                    opt(delimited(
                        tag("-"),
                        alt((tag("moz"), tag("webkit"), tag("ms"))),
                        tag("-"),
                    )),
                    alt((
                        tag("calc"),
                        tag("element"),
                        tag("env"),
                        tag("expression"),
                        tag("var"),
                    )),
                )),
                |_| (),
            ),
            map(preceded(tag("progid:"), selector_string), |_| ()),
        )),
        tag("("),
    ))(input)?;
    let (input, mut args) = special_args(input)?;
    let (input, end) = tag(")")(input)?;

    args.prepend(from_utf8(start).unwrap());
    args.append_str(from_utf8(end).unwrap());
    Ok((input, args))
}

pub fn special_function_minmax(input: &[u8]) -> IResult<&[u8], SassString> {
    let (input, start) = recognize(terminated(
        alt((tag_no_case("max"), tag_no_case("min"))),
        tag("("),
    ))(input)?;
    let (input, mut args) = special_args_minmax(input)?;
    let (input, end) = tag(")")(input)?;

    args.prepend(&from_utf8(start).unwrap().to_lowercase());
    args.append_str(from_utf8(end).unwrap());
    Ok((input, args))
}

fn special_args(input: &[u8]) -> IResult<&[u8], SassString> {
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

fn special_args_minmax(input: &[u8]) -> IResult<&[u8], SassString> {
    let (input, parts) = special_arg_parts_minmax(input)?;
    Ok((input, SassString::new(parts, Quotes::None)))
}

fn special_arg_parts_minmax(input: &[u8]) -> IResult<&[u8], Vec<StringPart>> {
    let (input, parts) = many0(alt((
        map(special_function_misc, |s| s.into_parts()),
        map(special_function_minmax, |s| s.into_parts()),
        map(
            terminated(
                map_res(
                    is_not("\r\n\t >$%\"'\\#+*/()[]{}:;,=!&@"),
                    input_to_str,
                ),
                peek(is_not("(")),
            ),
            |s| vec![StringPart::from(s)],
        ),
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
        map(map_res(is_a("0123456789,.+-*/ "), input_to_str), |s| {
            vec![StringPart::from(s)]
        }),
        // '% is allowed as percentage unit, but not as modulo operator
        map(
            terminated(map_res(is_a("% "), input_to_str), peek(is_a(",)"))),
            |s| vec![StringPart::from(s)],
        ),
    )))(input)?;
    let parts = parts.into_iter().flatten().collect::<Vec<_>>();
    // Disallow trailing comma
    if let Some(StringPart::Raw(ref s)) = parts.last() {
        if s.trim().ends_with(',') {
            return Err(nom::Err::Error((
                input,
                nom::error::ErrorKind::SeparatedList,
            )));
        }
    }
    Ok((input, parts))
}

pub fn special_url(input: &[u8]) -> IResult<&[u8], SassString> {
    let (input, _start) = tag("url(")(input)?;
    let (input, mut parts) = many1(alt((
        string_part_interpolation,
        map(selector_string, StringPart::Raw),
        map(map_res(is_a(":.!/"), input_to_string), StringPart::Raw),
    )))(input)?;
    let (input, _end) = tag(")")(input)?;
    parts.insert(0, "url(".into());
    parts.push(")".into());
    Ok((input, SassString::new(parts, Quotes::None)))
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

fn selector_string(input: &[u8]) -> IResult<&[u8], String> {
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
    map_res(is_not("\r\n\t >$\"'\\#+*/()[]{}:;,=!&@"), input_to_str)(input)
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
                std::char::from_u32
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
    from_utf8(&data).ok().and_then(|s| s.chars().next())
}

fn is_name_char(c: &char) -> bool {
    c.is_alphanumeric() || *c == '_' || *c == '-'
}
