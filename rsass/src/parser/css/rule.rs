use super::super::util::opt_spacelike;
use super::super::{PResult, Span};
use super::strings::custom_value;
use super::{comment, import2, strings, values};
use crate::css::parser::selector_set;
use crate::css::{BodyItem, CustomProperty, Property, Rule};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::one_of;
use nom::combinator::{into, map, opt};
use nom::multi::many_till;
use nom::sequence::{delimited, pair, preceded, terminated};

pub fn rule(input: Span) -> PResult<Rule> {
    map(
        pair(
            terminated(selector_set, terminated(tag("{"), opt_spacelike)),
            many_till(terminated(body_item, opt_spacelike), tag("}")),
        ),
        |(selectors, (body, _))| Rule { selectors, body },
    )(input)
}

fn body_item(input: Span) -> PResult<BodyItem> {
    alt((
        into(comment),
        into(preceded(tag("@import"), import2)),
        property,
    ))(input)
}

pub fn property(input: Span) -> PResult<BodyItem> {
    let (rest, name) = terminated(property_name, tag(":"))(input)?;
    if name.starts_with("--") {
        let (rest, value) = terminated(custom_value, opt(tag(";")))(rest)?;
        Ok((rest, CustomProperty::new(name, value).into()))
    } else {
        let (rest, value) =
            delimited(opt_spacelike, values::any, opt(tag(";")))(rest)?;
        Ok((rest, Property::new(name, value).into()))
    }
}

fn property_name(input: Span) -> PResult<String> {
    map(
        pair(opt(one_of("*:.")), strings::css_string),
        |(pre, mut main)| {
            if let Some(pre) = pre {
                main.insert(0, pre);
            }
            main
        },
    )(input)
}
