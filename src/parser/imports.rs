//! Support for `@import`, `@use`, and `@forward`.
use super::strings::{
    name, sass_string, sass_string_dq, sass_string_sq, special_url,
};
use super::util::{ignore_comments, opt_spacelike, semi_or_end};
use super::value::space_list;
use super::{media_args, PResult, Span};
use crate::sass::{Expose, Item, Name, SassString, UseAs, Value};
use crate::SourcePos;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt, value};
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::{delimited, pair, preceded, terminated, tuple};
use nom_locate::position;
use std::collections::BTreeSet;

/// What follows the `@import` tag.
pub fn import2(input: Span) -> PResult<Item> {
    map(
        terminated(
            tuple((
                separated_list0(
                    comma,
                    alt((
                        sass_string_dq,
                        sass_string_sq,
                        special_url,
                        sass_string,
                    )),
                ),
                opt(media_args),
                position,
            )),
            semi_or_end,
        ),
        |(import, args, end)| {
            let pos = SourcePos::from_to(input, end);
            Item::Import(import, args.unwrap_or(Value::Null), pos)
        },
    )(input)
}

pub fn use2(input: Span) -> PResult<Item> {
    map(
        terminated(
            tuple((
                terminated(any_sass_string, opt_spacelike),
                opt(preceded(
                    terminated(tag("with"), opt_spacelike),
                    with_arg,
                )),
                opt(preceded(terminated(tag("as"), opt_spacelike), as_arg)),
                position,
            )),
            semi_or_end,
        ),
        |(s, w, n, end)| {
            Item::Use(
                s,
                n.unwrap_or(UseAs::KeepName),
                w.unwrap_or_default(),
                SourcePos::from_to(input, end),
            )
        },
    )(input)
}

pub fn forward2(input: Span) -> PResult<Item> {
    let (mut end, path) = terminated(any_sass_string, opt_spacelike)(input)?;
    let mut found_as = None;
    let mut expose = Expose::All;
    let mut found_with = None;
    while let Ok((rest, arg)) = terminated(name, opt_spacelike)(end) {
        end = match arg.as_ref() {
            "as" if found_as.is_none() => {
                let (i, a) = as_arg(rest)?;
                found_as = Some(a);
                i
            }
            "hide" if expose == Expose::All => {
                let (i, (funs, vars)) = exposed_names(rest)?;
                expose = Expose::Hide(funs, vars);
                i
            }
            "show" if expose == Expose::All => {
                let (i, (funs, vars)) = exposed_names(rest)?;
                expose = Expose::Show(funs, vars);
                i
            }
            "with" if found_with.is_none() => {
                let (i, w) = with_arg(rest)?;
                found_with = Some(w);
                i
            }
            _ => {
                return Err(nom::Err::Error(nom::error::Error::new(
                    end,
                    nom::error::ErrorKind::MapRes,
                )));
            }
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
            SourcePos::from_to(input, end),
        ),
    ))
}

fn exposed_names(input: Span) -> PResult<(BTreeSet<Name>, BTreeSet<Name>)> {
    map(
        separated_list1(
            terminated(tag(","), opt_spacelike),
            pair(
                map(opt(tag("$")), |v| v.is_some()),
                map(terminated(name, opt_spacelike), Name::from),
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
    )(input)
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
    )(input)
}

fn with_arg(input: Span) -> PResult<Vec<(Name, Value, bool)>> {
    delimited(
        terminated(tag("("), opt_spacelike),
        separated_list0(
            comma,
            tuple((
                delimited(
                    tag("$"),
                    map(name, Name::from),
                    delimited(opt_spacelike, tag(":"), opt_spacelike),
                ),
                terminated(space_list, opt_spacelike),
                map(opt(terminated(tag("!default"), opt_spacelike)), |o| {
                    o.is_some()
                }),
            )),
        ),
        delimited(opt(comma), tag(")"), opt_spacelike),
    )(input)
}

fn any_sass_string(input: Span) -> PResult<SassString> {
    alt((sass_string_dq, sass_string_sq, sass_string))(input)
}

fn comma(input: Span) -> PResult<()> {
    map(terminated(tag(","), ignore_comments), |_| ())(input)
}
