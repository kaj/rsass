use super::{PResult, Span};
use crate::sass::{SassString, StringPart};
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{char, multispace1};
use nom::combinator::{eof, map, map_res, not, opt, peek};
use nom::multi::{fold_many0, fold_many1, many0};
use nom::sequence::{preceded, terminated};
use nom::Parser;
use nom_language::error::VerboseError;
use std::str::from_utf8;

pub(crate) fn term_opt_space<'a, F, T>(
    f: F,
) -> impl Parser<Span<'a>, Output = T, Error = VerboseError<Span<'a>>>
where
    F: Parser<Span<'a>, Output = T, Error = VerboseError<Span<'a>>>,
{
    terminated(f, opt_spacelike)
}

pub fn semi_or_end(input: Span) -> PResult<()> {
    preceded(
        opt_spacelike,
        alt((
            map(terminated(ignore_comments, eof), |_| ()),
            map(peek(char('}')), |_| ()),
            map(char(';'), |_| ()),
        )),
    )
    .parse(input)
}

pub fn spacelike(input: Span) -> PResult<()> {
    fold_many1(alt((ignore_space, ignore_lcomment)), || (), |(), ()| ())
        .parse(input)
}

pub fn spacelike2(input: Span) -> PResult<()> {
    map_res(ignore_comments, |s| if s { Ok(()) } else { Err(()) })
        .parse(input)
}

pub fn opt_spacelike(input: Span) -> PResult<()> {
    fold_many0(alt((ignore_space, ignore_lcomment)), || (), |(), ()| ())
        .parse(input)
}

pub fn ignore_comments(input: Span) -> PResult<bool> {
    fold_many0(
        alt((
            map(ignore_space, |()| true),
            map(ignore_lcomment, |()| true),
            map(comment, |_| false),
        )),
        || false,
        |a, b| a || b,
    )
    .parse(input)
}

pub fn comment(input: Span) -> PResult<SassString> {
    preceded(tag("/*"), comment2).parse(input)
}

pub fn comment2(input: Span) -> PResult<SassString> {
    use super::strings::string_part_interpolation;
    use crate::value::Quotes;
    map(
        terminated(
            many0(alt((
                map(
                    map_res(is_not("*#\r\n\u{c}"), |s: Span| {
                        from_utf8(s.fragment())
                    }),
                    StringPart::from,
                ),
                map(
                    alt((tag("\r\n"), tag("\n"), tag("\r"), tag("\u{c}"))),
                    |_| "\n".into(),
                ),
                map(terminated(tag("*"), peek(not(tag("/")))), |_| {
                    StringPart::from("*")
                }),
                string_part_interpolation,
                map(
                    map_res(tag("#"), |s: Span| from_utf8(s.fragment())),
                    StringPart::from,
                ),
            ))),
            tag("*/"),
        ),
        |p| SassString::new(p, Quotes::None),
    )
    .parse(input)
}

pub fn ignore_space(input: Span) -> PResult<()> {
    map(multispace1, |_| ()).parse(input)
}

fn ignore_lcomment(input: Span) -> PResult<()> {
    map(terminated(tag("//"), opt(is_not("\n"))), |_| ()).parse(input)
}

#[cfg(test)]
mod test {
    use super::{super::check_parse, comment};

    #[test]
    fn comment_simple() {
        assert_eq!(
            check_parse(comment, b"/* hello */"),
            Ok(" hello ".into())
        );
    }

    #[test]
    fn comment_with_stars() {
        assert_eq!(
            check_parse(comment, b"/**** hello ****/"),
            Ok("*** hello ***".into())
        )
    }

    #[test]
    fn comment_with_stars2() {
        assert_eq!(
            check_parse(comment, b"/* / * / * / * hello * \\ * \\ * \\ */"),
            Ok(" / * / * / * hello * \\ * \\ * \\ ".into())
        )
    }
}
