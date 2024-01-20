use super::{selectorset::SelectorSet, CssSelectorSet};

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

    pub(super) fn write_to_buf(&self, buf: &mut String) {
        buf.push(':');
        if self.element {
            buf.push(':');
        }
        buf.push_str(&self.name);
        // Note: This is an ugly workaround for lack of proper support
        // for "nth" type of pseudoclass arguments.
        if self.name_in(&[
            "nth-child",
            "nth-last-child",
            "nth-last-of-type",
            "nth-of-type",
        ]) {
            let mut t = String::new();
            self.arg.write_to_buf(&mut t);
            buf.push_str(&t.replacen(" + ", "+", 1));
        } else {
            self.arg.write_to_buf(buf);
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
        known.iter().any(|known| name == *known)
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
    pub(super) fn write_to_buf(&self, buf: &mut String) {
        match self {
            Arg::Selector(s) => {
                buf.push('(');
                s.write_to_buf(buf);
                buf.push(')');
            }
            Arg::Other(a) => {
                buf.push('(');
                buf.push_str(a);
                buf.push(')');
            }
            Arg::None => (),
        }
    }
}

pub(crate) mod parser {
    use super::super::selectorset::parser::selector_set;
    use super::{Arg, Pseudo};
    use crate::parser::css::strings::custom_value_inner;
    use crate::parser::{css::css_string, PResult, Span};
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::combinator::{map, value};
    use nom::sequence::{delimited, tuple};

    pub(crate) fn pseudo(input: Span) -> PResult<Pseudo> {
        map(
            tuple((
                alt((value(true, tag("::")), value(false, tag(":")))),
                css_string,
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
            )),
            |(element, name, arg)| Pseudo { name, arg, element },
        )(input)
    }
}
