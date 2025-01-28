pub(crate) mod media;
mod rule;
pub(crate) mod strings;
mod values;

use super::{PResult, Span};
use crate::css::{AtRule, Comment, Import, Item, MediaRule, Value};
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, tag_no_case};
use nom::character::complete::{multispace0, multispace1};
use nom::combinator::{
    all_consuming, into, map, map_res, not, opt, peek, recognize,
};
use nom::multi::{fold_many0, many0, many_till};
use nom::sequence::{delimited, preceded, terminated};
use nom::Parser as _;
use nom_language::error::{VerboseError, VerboseErrorKind};
use std::str::from_utf8;

pub fn file(input: Span) -> PResult<Vec<Item>> {
    preceded(
        alt((
            tag("\u{feff}".as_bytes()),
            tag_no_case(&b"@charset \"UTF-8\";\n"[..]),
            tag_no_case(&b"@charset \"ASCII\";\n"[..]),
            tag(""),
        )),
        map(
            many_till(
                preceded(opt_spacelike, top_level_item),
                all_consuming(opt_spacelike),
            ),
            |(v, _eof)| v,
        ),
    )
    .parse(input)
}

fn top_level_item(input: Span) -> PResult<Item> {
    let (rest, start) =
        alt((tag("@"), tag("/*"), tag("$"), tag(""))).parse(input)?;
    match start.fragment() {
        b"/*" => into(comment).parse(input),
        b"$" => {
            let (end, _) =
                preceded(tag("$"), strings::css_string).parse(input)?;
            let pos = input.up_to(&end);
            Err(nom_err("Sass variables aren't allowed in plain CSS.", pos))
        }
        b"@" => {
            let (input, name) = strings::css_string(rest)?;
            match name.as_ref() {
                "import" => into(import2).parse(input),
                "media" => {
                    let (input, args) =
                        preceded(spacelike, media::args).parse(input)?;
                    let (input, body) = preceded(
                        opt_spacelike,
                        delimited(
                            terminated(tag("{"), opt_spacelike),
                            many0(terminated(
                                map_res(top_level_item, TryInto::try_into),
                                opt_spacelike,
                            )),
                            tag("}"),
                        ),
                    )
                    .parse(input)?;
                    Ok((input, MediaRule::new(args, body).into()))
                }
                _ => {
                    let (input, args) = map_res(atrule_args, |s| {
                        std::str::from_utf8(s.fragment())
                    })
                    .parse(input)?;
                    let (input, body) = preceded(
                        opt_spacelike,
                        alt((
                            map(tag(";"), |_| None),
                            map(
                                delimited(
                                    terminated(tag("{"), opt_spacelike),
                                    many0(terminated(
                                        alt((
                                            into(comment),
                                            into(preceded(
                                                tag("@import"),
                                                import2,
                                            )),
                                            into(rule::rule),
                                            into(rule::property),
                                        )),
                                        opt_spacelike,
                                    )),
                                    tag("}"),
                                ),
                                Some,
                            ),
                        )),
                    )
                    .parse(input)?;
                    Ok((
                        input,
                        AtRule::new(name, args.trim().into(), body).into(),
                    ))
                }
            }
        }
        _ => into(rule::rule).parse(input),
    }
}

fn import2(input: Span) -> PResult<Import> {
    map(
        delimited(
            opt_spacelike,
            values::string_or_call,
            // TODO: Media arguments!
            opt(terminated(opt_spacelike, tag(";"))),
        ),
        |uri| Import::new(uri, Value::Null),
    )
    .parse(input)
}

// Arguments for unknwn at-rules.  Should probably be more permitting.
fn atrule_args(input: Span) -> PResult<Span> {
    recognize(opt(preceded(
        is_not("()/{}"),
        opt(terminated(
            delimited(tag("("), atrule_args, tag(")")),
            atrule_args,
        )),
    )))
    .parse(input)
}

pub fn comment(input: Span) -> PResult<Comment> {
    into(preceded(tag("/*"), comment2)).parse(input)
}

pub fn comment2(input: Span) -> PResult<String> {
    terminated(
        fold_many0(
            alt((
                map_res(is_not("*\r\n\u{c}"), |s: Span| {
                    from_utf8(s.fragment())
                }),
                map(
                    alt((tag("\r\n"), tag("\n"), tag("\r"), tag("\u{c}"))),
                    |_| "\n",
                ),
                map(terminated(tag("*"), peek(not(tag("/")))), |_| "*"),
            )),
            String::new,
            |mut acc, add| {
                acc.push_str(add);
                acc
            },
        ),
        tag("*/"),
    )
    .parse(input)
}

pub fn spacelike(input: Span) -> PResult<()> {
    map(multispace1, |_| ()).parse(input)
}

pub fn opt_spacelike(input: Span) -> PResult<()> {
    map(multispace0, |_| ()).parse(input)
}

fn nom_err<'a>(
    msg: &'static str,
    pos: Span<'a>,
) -> nom::Err<VerboseError<Span<'a>>> {
    nom::Err::Error(VerboseError {
        errors: vec![(pos, VerboseErrorKind::Context(msg))],
    })
}
