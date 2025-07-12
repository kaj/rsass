use super::{CssSelectorSet, Opt, SelectorSet};
use crate::output::CssBuf;

/// A pseudo-class or a css2 pseudo-element (:foo)
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Pseudo {
    /// The name of the pseudo-class
    name: String,
    /// Arguments to the pseudo-class
    arg: Arg,
    /// True if this is a `::psedu-element`, false for a `:pseudo-class`.
    element: bool,
}

impl Pseudo {
    pub(crate) fn no_placeholder(&self) -> Opt<Self> {
        let arg = match &self.arg {
            Arg::Selector(s) => {
                match (s.no_placeholder(), self.name_in(&["not"])) {
                    (Opt::Some(t), _) => {
                        if self.name_in(&["is"]) {
                            match t.no_leading_combinator() {
                                Opt::Some(t) => Arg::Selector(t),
                                Opt::Any => return Opt::Any,
                                Opt::None => return Opt::None,
                            }
                        } else {
                            Arg::Selector(t)
                        }
                    }
                    (Opt::Any, false) | (Opt::None, true) => return Opt::Any,
                    (Opt::None, false) | (Opt::Any, true) => {
                        return Opt::None
                    }
                }
            }
            arg => arg.clone(),
        };
        Opt::Some(Self {
            arg,
            name: self.name.clone(),
            element: self.element,
        })
    }

    pub(crate) fn is_superselector(&self, b: &Self) -> bool {
        if self.is_element() != b.is_element() || self.name != b.name {
            return false;
        }
        // Note: A better implementetation of is/matches/any would be
        // different from has, host, and host-context.
        if self.name_in(&["not"]) {
            b.arg.is_superselector(&self.arg) // NOTE: Reversed!
        } else if self.name_in(&["current"]) {
            self.arg == b.arg
        } else {
            self.arg.is_superselector(&b.arg)
        }
    }

    pub(super) fn is_element(&self) -> bool {
        self.element || is_pseudo_element(&self.name)
    }
    pub(super) fn is_hover(&self) -> bool {
        self.name == "hover"
    }
    pub(super) fn is_host(&self) -> bool {
        self.name_in(&["host", "host-context"])
    }
    pub(super) fn is_rootish(&self) -> bool {
        self.name_in(&["host", "host-context", "root", "scope"])
    }

    pub(super) fn has_backref(&self) -> bool {
        if let Arg::Selector(s) = &self.arg {
            s.has_backref()
        } else {
            false
        }
    }
    pub(super) fn resolve_ref(mut self, ctx: &CssSelectorSet) -> Self {
        self.arg = match self.arg {
            Arg::Selector(s) => Arg::Selector(s.resolve_ref(ctx)),
            x => x,
        };
        self
    }
    pub(super) fn replace(
        mut self,
        original: &SelectorSet,
        replacement: &SelectorSet,
    ) -> Self {
        if self.name_in(&[
            "is",
            "matches",
            "not",
            "any",
            "where",
            "has",
            "host",
            "host-context",
        ]) {
            self.arg = match self.arg {
                Arg::Selector(s) => {
                    Arg::Selector(s.replace(original, replacement).unwrap())
                }
                x => x,
            };
        }
        self
    }

    pub(super) fn write_to(&self, buf: &mut CssBuf) {
        buf.add_char(':');
        if self.element {
            buf.add_char(':');
        }
        buf.add_str(&self.name);
        // Note: This is an ugly workaround for lack of proper support
        // for "nth" type of pseudoclass arguments.
        if self.name_in(&[
            "nth-child",
            "nth-last-child",
            "nth-last-of-type",
            "nth-of-type",
        ]) {
            let mut t = CssBuf::new(buf.format());
            self.arg.write_to(&mut t);
            buf.add_str(
                &String::from_utf8_lossy(&t.take()).replacen(" + ", "+", 1),
            );
        } else {
            self.arg.write_to(buf);
        }
    }
    fn name_in(&self, names: &[&str]) -> bool {
        name_in(&self.name, names)
    }
}

fn is_pseudo_element(n: &str) -> bool {
    let pse = [
        "after",
        "before",
        "file-selector-button",
        "first-letter",
        "first-line",
        "grammar-error",
        "marker",
        "placeholder",
        "selection",
        "spelling-error",
        "target-text",
    ];
    pse.binary_search(&n).is_ok()
}

fn name_in(name: &str, known: &[&str]) -> bool {
    if name.starts_with('-') {
        known.iter().any(|end| {
            name.strip_suffix(end).map_or(false, |s| s.ends_with('-'))
        })
    } else {
        known.contains(&name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Arg {
    Selector(SelectorSet),
    Other(String),
    None,
}

impl Arg {
    fn is_superselector(&self, b: &Self) -> bool {
        match (self, b) {
            (Self::Selector(a), Self::Selector(b)) => a.is_superselector(b),
            (Self::Other(a), Self::Other(b)) => a == b,
            (Self::None, Self::None) => true,
            _ => false,
        }
    }
    pub(super) fn write_to(&self, buf: &mut CssBuf) {
        match self {
            Self::Selector(s) => {
                buf.add_char('(');
                s.write_to(buf);
                buf.add_char(')');
            }
            Self::Other(a) => {
                buf.add_char('(');
                buf.add_str(a);
                buf.add_char(')');
            }
            Self::None => (),
        }
    }
}

pub(super) mod parser {
    use super::super::parser::selector_set;
    use super::{Arg, Pseudo};
    use crate::parser::css::strings::{
        css_string_nohash, custom_value_inner,
    };
    use crate::parser::{PResult, Span};
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::combinator::{map, value};
    use nom::sequence::delimited;
    use nom::Parser as _;

    pub(crate) fn pseudo(input: Span) -> PResult<Pseudo> {
        map(
            (
                alt((value(true, tag("::")), value(false, tag(":")))),
                css_string_nohash,
                // Note: The accepted type of selector should probably
                // depend on the name, so that known pseudo attributes
                // requires the correct kind of arguments.
                alt((
                    map(
                        delimited(tag("("), selector_set, tag(")")),
                        Arg::Selector,
                    ),
                    map(
                        delimited(tag("("), custom_value_inner, tag(")")),
                        Arg::Other,
                    ),
                    map(tag(""), |_| Arg::None),
                )),
            ),
            |(element, name, arg)| Pseudo { name, arg, element },
        )
        .parse(input)
    }
}
