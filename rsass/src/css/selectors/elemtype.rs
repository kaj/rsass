use crate::output::CssBuf;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ElemType {
    s: String,
}

impl Default for ElemType {
    fn default() -> Self {
        Self { s: "*".into() }
    }
}

impl ElemType {
    pub fn is_any(&self) -> bool {
        self.s == "*"
    }
    pub(super) fn cant_append(&self) -> bool {
        self.s.starts_with('*') || self.s.starts_with('|')
    }

    pub fn is_superselector(&self, sub: &Self) -> bool {
        let (e_ns, e_name) = self.split_ns();
        let (sub_ns, sub_name) = sub.split_ns();
        match_name(e_ns.unwrap_or("*"), sub_ns.unwrap_or("*"))
            && match_name(e_name, sub_name)
    }

    pub fn unify(&self, other: &Self) -> Option<Self> {
        let (e_ns, e_name) = self.split_ns();
        let (o_ns, o_name) = other.split_ns();
        let ns = match (e_ns, o_ns) {
            (None, None) => None,
            (Some("*"), ns) | (ns, Some("*")) => ns,
            (Some(e), Some(o)) if e == o => Some(e),
            _ => return None,
        };
        let name = match (e_name, o_name) {
            ("*", name) | (name, "*") => name,
            (e, o) if e == o => e,
            _ => return None,
        };
        Some(Self {
            s: if let Some(ns) = ns {
                format!("{ns}|{name}")
            } else {
                name.to_string()
            },
        })
    }

    fn split_ns(&self) -> (Option<&str>, &str) {
        match self.s.split_once('|') {
            Some((ns, name)) => (Some(ns), name),
            None => (None, &self.s),
        }
    }

    pub fn write_to(&self, buf: &mut CssBuf) {
        buf.add_str(&self.s);
    }
}
impl fmt::Display for ElemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.s.fmt(f)
    }
}

fn match_name(a: &str, b: &str) -> bool {
    a == "*" || a == b
}

pub(super) mod parser {
    use super::ElemType;
    use crate::parser::css::strings::css_string_nohash as css_string;
    use crate::parser::{PResult, Span};
    use nom::branch::alt;
    use nom::bytes::complete::{is_a, tag};
    use nom::combinator::{map, opt, recognize, value};
    use nom::sequence::{pair, preceded, tuple};

    pub(crate) fn elem_name(input: Span) -> PResult<ElemType> {
        map(name_opt_ns, |s| ElemType { s })(input)
    }

    /// Recognize a keyframe stop as an element selector.
    ///
    /// This is the way keyframes are currently supported, may change
    /// in the future.
    pub(crate) fn keyframe_stop(input: Span) -> PResult<ElemType> {
        map(
            recognize(tuple((
                is_a("0123456789."),
                opt(tuple((is_a("eE"), opt(tag("-")), is_a("0123456789")))),
                tag("%"),
            ))),
            |stop: Span| ElemType {
                s: String::from_utf8_lossy(stop.fragment()).to_string(),
            },
        )(input)
    }

    pub(crate) fn name_opt_ns(input: Span) -> PResult<String> {
        fn name_part(input: Span) -> PResult<String> {
            alt((value(String::from("*"), tag("*")), css_string))(input)
        }
        alt((
            map(preceded(tag("|"), name_part), |mut s| {
                s.insert(0, '|');
                s
            }),
            map(
                pair(name_part, opt(preceded(tag("|"), name_part))),
                |(a, b)| {
                    if let Some(b) = b {
                        format!("{a}|{b}")
                    } else {
                        a
                    }
                },
            ),
        ))(input)
    }
}
