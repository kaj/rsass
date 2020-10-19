use crate::sass::{SassString, StringPart};
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::multispace1;
use nom::combinator::{map, map_res, not, opt};
use nom::multi::{fold_many0, fold_many1, many0};
use nom::sequence::{preceded, terminated};
use nom::IResult;
use std::str::from_utf8;

pub fn spacelike(input: &[u8]) -> IResult<&[u8], ()> {
    fold_many1(alt((ignore_space, ignore_lcomment)), (), |(), ()| ())(input)
}

pub fn spacelike2(input: &[u8]) -> IResult<&[u8], ()> {
    terminated(spacelike, ignore_comments)(input)
}

pub fn opt_spacelike(input: &[u8]) -> IResult<&[u8], ()> {
    fold_many0(alt((ignore_space, ignore_lcomment)), (), |(), ()| ())(input)
}

pub fn ignore_comments(input: &[u8]) -> IResult<&[u8], ()> {
    fold_many0(
        alt((ignore_space, ignore_lcomment, map(comment, |_| ()))),
        (),
        |(), ()| (),
    )(input)
}

pub fn comment(input: &[u8]) -> IResult<&[u8], SassString> {
    preceded(tag("/*"), comment2)(input)
}

pub fn comment2(input: &[u8]) -> IResult<&[u8], SassString> {
    use super::strings::string_part_interpolation;
    use crate::value::Quotes;
    use nom::combinator::peek;
    map(
        terminated(
            many0(alt((
                map(
                    map_res(is_not("*#\r\n\u{c}"), from_utf8),
                    StringPart::from,
                ),
                map(
                    alt((tag("\r\n"), tag("\n"), tag("\r"), tag("\u{c}"))),
                    |_| "\n".into(),
                ),
                map(
                    terminated(
                        map_res(tag("*"), from_utf8),
                        peek(not(tag("/"))),
                    ),
                    StringPart::from,
                ),
                string_part_interpolation,
                map(map_res(tag("#"), from_utf8), StringPart::from),
            ))),
            tag("*/"),
        ),
        |p| SassString::new(p, Quotes::None),
    )(input)
}

pub fn ignore_space(input: &[u8]) -> IResult<&[u8], ()> {
    map(multispace1, |_| ())(input)
}

fn ignore_lcomment(input: &[u8]) -> IResult<&[u8], ()> {
    map(terminated(tag("//"), opt(is_not("\n"))), |_| ())(input)
}

#[cfg(test)]
mod test {
    use super::comment;

    #[test]
    fn comment_simple() {
        assert_eq!(
            comment(b"/* hello */\n"),
            Ok((&b"\n"[..], " hello ".into())),
        )
    }

    #[test]
    fn comment_with_stars() {
        assert_eq!(
            comment(b"/**** hello ****/\n"),
            Ok((&b"\n"[..], "*** hello ***".into())),
        )
    }

    #[test]
    fn comment_with_stars2() {
        assert_eq!(
            comment(b"/* / * / * / * hello * \\ * \\ * \\ */\n"),
            Ok((&b"\n"[..], " / * / * / * hello * \\ * \\ * \\ ".into())),
        )
    }
}
