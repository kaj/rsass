//! Support for `@import`, `@use`, and `@forward`.
use super::strings::{
    name, sass_string, sass_string_dq, sass_string_sq, special_url,
};
use super::util::{ignore_comments, opt_spacelike, semi_or_end};
use super::value::{identifier, space_list};
use super::{media, position, PResult, Span};
use crate::sass::{Expose, Item, Name, SassString, UseAs, Value};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::{cut, map, opt, value};
use nom::error::context;
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::{delimited, pair, preceded, terminated};
use nom::Parser as _;
use std::collections::BTreeSet;

/// What follows the `@import` tag.
pub fn import2(input: Span) -> PResult<Item> {
    map(
        terminated(
            (
                separated_list0(
                    comma,
                    alt((
                        sass_string_dq,
                        sass_string_sq,
                        special_url,
                        sass_string,
                    )),
                ),
                opt(media::args),
                position,
            ),
            semi_or_end,
        ),
        |(import, args, end)| {
            let pos = input.up_to(&end).to_owned();
            Item::Import(import, args.unwrap_or(Value::Null), pos)
        },
    )
    .parse(input)
}

pub fn use2<'a>(start: Span, input: Span<'a>) -> PResult<'a, Item> {
    map(
        terminated(
            (
                context(
                    "Expected string.",
                    terminated(quoted_sass_string, ignore_comments),
                ),
                opt(preceded(
                    terminated(tag("with"), ignore_comments),
                    with_arg,
                )),
                opt(preceded(terminated(tag("as"), ignore_comments), as_arg)),
                position,
            ),
            semi_or_end,
        ),
        |(s, w, n, end)| {
            Item::Use(
                s,
                n.unwrap_or(UseAs::KeepName),
                w.unwrap_or_default(),
                start.up_to(&end).to_owned(),
            )
        },
    )
    .parse(input)
}

pub fn forward2<'a>(start: Span, input: Span<'a>) -> PResult<'a, Item> {
    let (mut end, path) = context(
        "Expected string.",
        terminated(quoted_sass_string, opt_spacelike),
    )
    .parse(input)?;
    let mut found_as = None;
    let mut expose = Expose::All;
    let mut found_with = None;
    while let Ok((rest, arg)) =
        delimited(ignore_comments, name, ignore_comments).parse(end)
    {
        end = match arg.as_ref() {
            "as" if found_as.is_none() && found_with.is_none() => {
                let (i, a) = fwd_as_arg(rest)?;
                found_as = Some(a);
                i
            }
            "hide" if expose == Expose::All && found_with.is_none() => {
                let (i, (funs, vars)) = exposed_names(rest)?;
                expose = Expose::Hide(funs, vars);
                i
            }
            "show" if expose == Expose::All && found_with.is_none() => {
                let (i, (funs, vars)) = exposed_names(rest)?;
                expose = Expose::Show(funs, vars);
                i
            }
            "with" if found_with.is_none() => {
                let (i, w) = with_arg(rest)?;
                found_with = Some(w);
                i
            }
            _ => break,
        };
    }
    let (rest, ()) = semi_or_end(end)?;
    Ok((
        rest,
        Item::Forward(
            path,
            found_as.unwrap_or(UseAs::Star),
            expose,
            found_with.unwrap_or_default(),
            start.up_to(&end).to_owned(),
        ),
    ))
}

fn exposed_names(input: Span) -> PResult<(BTreeSet<Name>, BTreeSet<Name>)> {
    map(
        separated_list1(
            terminated(tag(","), opt_spacelike),
            pair(
                map(opt(tag("$")), |v| v.is_some()),
                cut(context(
                    "Expected variable, mixin, or function name",
                    map(terminated(name, opt_spacelike), Name::from),
                )),
            ),
        ),
        |items| {
            let mut funs = BTreeSet::new();
            let mut vars = BTreeSet::new();
            for (v, n) in items {
                if v { &mut vars } else { &mut funs }.insert(n);
            }
            (funs, vars)
        },
    )
    .parse(input)
}

fn as_arg(input: Span) -> PResult<UseAs> {
    terminated(
        alt((
            map(pair(name, opt(tag("*"))), |(name, s)| match s {
                None => UseAs::Name(name),
                Some(_) => UseAs::Prefix(name),
            }),
            value(UseAs::Star, tag("*")),
        )),
        opt_spacelike,
    )
    .parse(input)
}

fn fwd_as_arg(input: Span) -> PResult<UseAs> {
    map(terminated(identifier, char('*')), UseAs::Prefix).parse(input)
}

fn with_arg(input: Span) -> PResult<Vec<(Name, Value, bool)>> {
    delimited(
        terminated(char('('), ignore_comments),
        separated_list1(
            comma,
            (
                delimited(
                    char('$'),
                    map(identifier, Name::from),
                    delimited(ignore_comments, char(':'), ignore_comments),
                ),
                terminated(space_list, ignore_comments),
                map(opt(terminated(tag("!default"), opt_spacelike)), |o| {
                    o.is_some()
                }),
            ),
        ),
        delimited(opt(comma), char(')'), opt_spacelike),
    )
    .parse(input)
}

fn quoted_sass_string(input: Span) -> PResult<SassString> {
    alt((sass_string_dq, sass_string_sq)).parse(input)
}

fn comma(input: Span) -> PResult<()> {
    delimited(ignore_comments, map(tag(","), |_| ()), ignore_comments)
        .parse(input)
}
