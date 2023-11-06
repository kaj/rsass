pub(crate) mod media;
mod rule;
mod selectors;
mod strings;
mod values;

pub(crate) use self::selectors::{selector, selector_parts, selectors};

use super::util::{opt_spacelike, spacelike};
use super::{PResult, Span};
use crate::css::{AtRule, Comment, Import, Item, MediaRule, Value};
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, tag_no_case};
use nom::combinator::{
    all_consuming, into, map, map_res, not, opt, peek, recognize,
};
use nom::multi::{fold_many0, many0, many_till};
use nom::sequence::{delimited, preceded, terminated};
use std::str::from_utf8;

pub fn file(input: Span) -> PResult<Vec<Item>> {
    preceded(
        alt((
            tag("\u{feff}".as_bytes()),
            tag_no_case(b"@charset \"UTF-8\";\n"),
            tag_no_case(b"@charset \"ASCII\";\n"),
            tag(""),
        )),
        map(
            many_till(
                preceded(opt_spacelike, top_level_item),
                all_consuming(opt_spacelike),
            ),
            |(v, _eof)| v,
        ),
    )(input)
}

fn top_level_item(input: Span) -> PResult<Item> {
    let (rest, start) = alt((tag("@"), tag("/*"), tag("")))(input)?;
    match start.fragment() {
        b"/*" => into(comment)(input),
        b"@" => {
            let (input, name) = strings::css_string(rest)?;
            match name.as_ref() {
                "import" => into(import2)(input),
                "media" => {
                    let (input, args) =
                        preceded(spacelike, media::args)(input)?;
                    let (input, body) = preceded(
                        opt_spacelike,
                        delimited(
                            terminated(tag("{"), opt_spacelike),
                            many0(terminated(
                                alt((into(comment), into(rule::rule))),
                                opt_spacelike,
                            )),
                            tag("}"),
                        ),
                    )(input)?;
                    Ok((input, MediaRule::new(args, body).into()))
                }
                _ => {
                    let (input, args) = map_res(atrule_args, |s| {
                        std::str::from_utf8(s.fragment())
                    })(input)?;
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
                    )(input)?;
                    Ok((
                        input,
                        AtRule::new(name, args.trim().into(), body).into(),
                    ))
                }
            }
        }
        _ => into(rule::rule)(input),
    }
}

fn import2(input: Span) -> PResult<Import> {
    map(
        delimited(
            opt_spacelike,
            strings::css_string_any,
            // TODO: Media arguments!
            opt(terminated(opt_spacelike, tag(";"))),
        ),
        |uri| Import::new(uri, Value::Null),
    )(input)
}

// Arguments for unknwn at-rules.  Should probably be more permitting.
fn atrule_args(input: Span) -> PResult<Span> {
    recognize(preceded(
        is_not("()/{}"),
        opt(terminated(
            delimited(tag("("), atrule_args, tag(")")),
            atrule_args,
        )),
    ))(input)
}

pub fn comment(input: Span) -> PResult<Comment> {
    into(preceded(tag("/*"), comment2))(input)
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
    )(input)
}
