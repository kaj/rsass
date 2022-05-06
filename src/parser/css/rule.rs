use std::str::from_utf8;

use super::super::util::opt_spacelike;
use super::super::{PResult, Span};
use super::{comment, import2, selectors, strings, values};
use crate::css::{BodyItem, Property, Rule};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{into, map, opt};
use nom::multi::many_till;
use nom::sequence::{pair, preceded, terminated};

pub fn rule(input: Span) -> PResult<Rule> {
    map(
        pair(
            terminated(selectors, terminated(tag("{"), opt_spacelike)),
            many_till(terminated(body_item, opt_spacelike), tag("}")),
        ),
        |(selectors, (body, _))| Rule { selectors, body },
    )(input)
}

fn body_item(input: Span) -> PResult<BodyItem> {
    alt((
        into(comment),
        into(preceded(tag("@import"), import2)),
        into(property),
    ))(input)
}

pub fn property(input: Span) -> PResult<Property> {
    map(
        pair(
            terminated(property_name, terminated(tag(":"), opt_spacelike)),
            terminated(values::any, opt(tag(";"))),
        ),
        |(name, val)| Property::new(name, val),
    )(input)
}

fn property_name(input: Span) -> PResult<String> {
    map(
        pair(alt((tag("*"), tag(":"), tag(""))), strings::css_string),
        |(pre, main)| {
            let mut main = main;
            main.insert_str(0, from_utf8(pre.fragment()).unwrap());
            main
        },
    )(input)
}
