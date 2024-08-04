//! This module contains types for the selectors of a rule.
//!
//! Basically, in a rule like `p.foo, .foo p { some: thing; }` there
//! is a `Selectors` object which contains two `Selector` objects, one
//! for `p.foo` and one for `.foo p`.
//!
//! This _may_ change to a something like a tree of operators with
//! leafs of simple selectors in some future release.
use super::{is_not, CssString, Value};
use crate::input::SourcePos;
use crate::parser::{input_span, ParseError};
use std::fmt;
use std::io::Write;

mod attribute;
mod cssselectorset;
mod logical;
mod opt;
mod pseudo;
mod selectorset;

pub(crate) use self::opt::Opt;
pub(crate) use cssselectorset::CssSelectorSet;
pub use logical::Selector;
pub use selectorset::SelectorSet;

pub(crate) mod parser {
    pub(crate) use super::selectorset::parser::selector_set;
}

/// A full set of selectors.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct OldSelectors {
    s: Vec<OldSelector>,
}

impl OldSelectors {
    /// Create a root (empty) selector.
    pub fn root() -> Self {
        Self {
            s: vec![OldSelector::root()],
        }
    }
    /// Return true if this is a root (empty) selector.
    pub fn is_root(&self) -> bool {
        self.s == [OldSelector::root()]
    }
    /// Create a new `Selectors` from a vec of selectors.
    pub fn new(s: Vec<OldSelector>) -> Self {
        if s.is_empty() {
            Self::root()
        } else {
            Self { s }
        }
    }

    /// Validate that this selector is ok to use in css.
    ///
    /// `Selectors` can contain backref (`&`), but those must be
    /// resolved before using the `Selectors` in css.
    pub fn css_ok(self) -> Result<Self, BadSelector> {
        if self.has_backref() {
            let sel = self.to_string();
            Err(BadSelector::Backref(input_span(sel)))
        } else {
            Ok(self)
        }
    }

    /// True if any of the selectors contains a backref (`&`).
    pub(crate) fn has_backref(&self) -> bool {
        self.s.iter().any(OldSelector::has_backref)
    }

    /// Get a vec of the non-placeholder selectors in self.
    pub fn no_placeholder(&self) -> Option<Self> {
        let s = self
            .s
            .iter()
            .filter_map(OldSelector::no_placeholder)
            .collect::<Vec<_>>();
        if s.is_empty() {
            None
        } else {
            Some(Self { s })
        }
    }

    fn no_leading_combinator(mut self) -> Option<Self> {
        self.s.retain(|s| !s.has_leading_combinator());
        if self.s.is_empty() {
            None
        } else {
            Some(self)
        }
    }

    /// Return true if any of these selectors ends with a combinator
    pub fn has_trailing_combinator(&self) -> bool {
        self.s.iter().any(OldSelector::has_trailing_combinator)
    }
}

/// A full set of selectors with a separate backref.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelectorCtx {
    /// The actual selectors.
    s: CssSelectorSet,
    backref: CssSelectorSet,
}

impl SelectorCtx {
    pub fn root() -> Self {
        Self {
            s: CssSelectorSet::root(),
            backref: CssSelectorSet::root(),
        }
    }
    /// Return true if this is a root (empty) selector.
    pub fn is_root(&self) -> bool {
        // TODO: Just s?  Or really both s and backref?
        self.s.is_root() && self.backref.is_root()
    }

    pub fn real(&self) -> CssSelectorSet {
        self.s.clone()
    }

    /// Evaluate selectors inside this context.
    pub(crate) fn nest(&self, selectors: SelectorSet) -> CssSelectorSet {
        if selectors.has_backref() {
            let backref = if self.s.is_root() {
                &self.backref
            } else {
                &self.s
            };
            CssSelectorSet {
                s: selectors.resolve_ref(backref),
            }
        } else {
            self.s.nest(selectors)
        }
    }
    pub(crate) fn at_root(&self, selectors: SelectorSet) -> Self {
        let backref = if self.s.is_root() {
            &self.backref
        } else {
            &self.s
        };
        let s = selectors
            .s
            .into_iter()
            .flat_map(|s| {
                if s.has_backref() {
                    s.resolve_ref(backref)
                } else {
                    vec![s]
                }
            })
            .collect();
        Self {
            s: CssSelectorSet {
                s: SelectorSet { s },
            },
            backref: backref.clone(),
        }
    }
}

impl From<CssSelectorSet> for SelectorCtx {
    fn from(value: CssSelectorSet) -> Self {
        Self {
            s: value,
            backref: CssSelectorSet::root(),
        }
    }
}

/// A single css selector.
///
/// A selector does not contain `,`.  If it does, it is a `Selectors`,
/// where each of the parts separated by the comma is a `Selector`.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct OldSelector(pub(crate) Vec<OldSelectorPart>);

impl OldSelector {
    /// Get the root (empty) selector.
    pub fn root() -> Self {
        Self(vec![])
    }

    /// Validate that this selector is ok to use in css.
    ///
    /// `Selectors` can contain backref (`&`), but those must be
    /// resolved before using the `Selectors` in css.
    pub fn css_ok(self) -> Result<Self, BadSelector> {
        if self.has_backref() {
            let slf = self.to_string();
            Err(BadSelector::Backref(input_span(slf)))
        } else {
            Ok(self)
        }
    }

    fn has_backref(&self) -> bool {
        self.0.iter().any(OldSelectorPart::has_backref)
    }

    /// Return this selector without placeholders.
    ///
    /// For most plain selectors, this returns Some(clone of self).
    /// For placeholder selectors, it returns None.
    /// For some selectors containing e.g. `p:matches(%a,.foo)` it
    /// returns a modified selector (in that case, `p:matches(.foo)`).
    fn no_placeholder(&self) -> Option<Self> {
        let v = self
            .0
            .iter()
            .map(OldSelectorPart::no_placeholder)
            .collect::<Option<Vec<_>>>()?;
        let mut v2 = Vec::with_capacity(v.len());
        let mut has_sel = false;
        for part in v {
            if has_sel && part.is_wildcard() {
                continue;
            }
            has_sel = !part.is_operator();
            v2.push(part);
        }
        let result = Self(v2);
        if result.has_trailing_combinator() || result.has_double_combinator()
        {
            None
        } else {
            Some(result)
        }
    }
    fn has_leading_combinator(&self) -> bool {
        matches!(self.0.first(), Some(OldSelectorPart::RelOp(_)))
    }
    /// Return true if this selector ends with a combinator
    pub fn has_trailing_combinator(&self) -> bool {
        matches!(self.0.last(), Some(OldSelectorPart::RelOp(_)))
    }
    fn has_double_combinator(&self) -> bool {
        self.0.windows(2).any(|w| {
            matches!(
                w,
                [OldSelectorPart::RelOp(_), OldSelectorPart::RelOp(_)]
            )
        })
    }
}

/// A selector consist of a sequence of these parts.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub enum OldSelectorPart {
    /// A simple selector, eg a class, id or element name.
    Simple(String),
    /// The empty relational operator.
    ///
    /// The thing after this is a descendant of the thing before this.
    Descendant,
    /// A relational operator; `>`, `+`, `~`.
    RelOp(u8),
    /// An attribute selector
    Attribute {
        /// The attribute name
        // TODO: Why not a raw String?
        name: CssString,
        /// An operator
        op: String,
        /// A value to match.
        val: CssString,
        /// Optional modifier.
        modifier: Option<char>,
    },
    /// A css3 pseudo-element (::foo)
    PseudoElement {
        /// The name of the pseudo-element
        name: CssString,
        /// Arguments to the pseudo-element
        arg: Option<OldSelectors>,
    },
    /// A pseudo-class or a css2 pseudo-element (:foo)
    Pseudo {
        /// The name of the pseudo-class
        name: CssString,
        /// Arguments to the pseudo-class
        arg: Option<OldSelectors>,
    },
    /// A sass backref (`&`), to be replaced with outer selector.
    BackRef,
}

impl OldSelectorPart {
    pub(crate) fn is_operator(&self) -> bool {
        match *self {
            Self::Descendant | Self::RelOp(_) => true,
            Self::Simple(_)
            | Self::Attribute { .. }
            | Self::PseudoElement { .. }
            | Self::Pseudo { .. }
            | Self::BackRef => false,
        }
    }
    pub(crate) fn is_wildcard(&self) -> bool {
        if let Self::Simple(s) = self {
            s == "*"
        } else {
            false
        }
    }
    fn has_backref(&self) -> bool {
        match *self {
            Self::Descendant
            | Self::RelOp(_)
            | Self::Simple(_)
            | Self::Attribute { .. } => false,
            Self::BackRef => true,
            Self::PseudoElement { ref arg, .. }
            | Self::Pseudo { ref arg, .. } => arg
                .as_ref()
                .map_or(false, |a| a.s.iter().any(OldSelector::has_backref)),
        }
    }
    /// Return this selectorpart without placeholders.
    ///
    /// For most parts, this returns either Some(clone of self) or None if
    /// it was a placeholder selector, but some pseudoselectors are
    /// converted to a version without the placeholder parts.
    fn no_placeholder(&self) -> Option<Self> {
        match self {
            Self::Simple(s) => {
                if !s.starts_with('%') {
                    Some(Self::Simple(s.clone()))
                } else {
                    None
                }
            }
            Self::Pseudo { name, arg } => match name.value() {
                "is" => arg
                    .as_ref()
                    .and_then(OldSelectors::no_placeholder)
                    .and_then(OldSelectors::no_leading_combinator)
                    .map(|arg| Self::Pseudo {
                        name: name.clone(),
                        arg: Some(arg),
                    }),
                "matches" | "any" | "where" | "has" => arg
                    .as_ref()
                    .and_then(OldSelectors::no_placeholder)
                    .map(|arg| Self::Pseudo {
                        name: name.clone(),
                        arg: Some(arg),
                    }),
                "not" => {
                    if let Some(arg) =
                        arg.as_ref().and_then(OldSelectors::no_placeholder)
                    {
                        Some(Self::Pseudo {
                            name: name.clone(),
                            arg: Some(arg),
                        })
                    } else {
                        Some(Self::Simple("*".into()))
                    }
                }
                _ => Some(Self::Pseudo {
                    name: name.clone(),
                    arg: arg.clone(),
                }),
            },
            x => Some(x.clone()),
        }
    }
}

// TODO:  This shoule probably be on Formatted<Selectors> instead.
impl fmt::Display for OldSelectors {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        if let Some((first, rest)) = self.s.split_first() {
            first.fmt(out)?;
            let separator = if out.alternate() { "," } else { ", " };
            for item in rest {
                out.write_str(separator)?;
                item.fmt(out)?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for OldSelector {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        // Note: There should be smarter whitespace-handling here, avoiding
        // the need to clean up afterwards.
        let mut buf = vec![];
        for p in &self.0 {
            if out.alternate() {
                write!(&mut buf, "{p:#}").map_err(|_| fmt::Error)?;
            } else {
                write!(&mut buf, "{p}").map_err(|_| fmt::Error)?;
            }
        }
        if buf.ends_with(b"> ") {
            buf.pop();
        }
        while buf.first() == Some(&b' ') {
            buf.remove(0);
        }
        let buf = String::from_utf8(buf).map_err(|_| fmt::Error)?;
        out.write_str(&buf.replace("  ", " "))
    }
}

impl fmt::Display for OldSelectorPart {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Simple(ref s) => write!(out, "{s}"),
            Self::Descendant => write!(out, " "),
            Self::RelOp(ref c) => {
                if out.alternate() && *c != b'~' {
                    write!(out, "{}", *c as char)
                } else {
                    write!(out, " {} ", *c as char)
                }
            }
            Self::Attribute {
                ref name,
                ref op,
                ref val,
                ref modifier,
            } => write!(
                out,
                "[{name}{op}{val}{}]",
                modifier.map(|m| format!(" {m}")).unwrap_or_default()
            ),
            Self::PseudoElement { ref name, ref arg } => {
                write!(out, "::{name}")?;
                if let Some(ref arg) = *arg {
                    if out.alternate() {
                        write!(out, "({arg:#})")?;
                    } else {
                        write!(out, "({arg})")?;
                    }
                }
                Ok(())
            }
            Self::Pseudo { ref name, ref arg } => {
                let name = name.to_string();
                if let Some(ref arg) = *arg {
                    // It seems some pseudo-classes should always have
                    // their arg in compact form.  Maybe we need more
                    // hard-coded names here, or maybe the condition
                    // should be on the argument rather than the name?
                    if out.alternate() || name == "nth-of-type" {
                        write!(out, ":{name}({arg:#})",)
                    } else if name == "nth-child" {
                        let arg = format!("{arg:#}");
                        write!(out, ":{name}({})", arg.replace(',', ", "))
                    } else {
                        write!(out, ":{name}({arg})")
                    }
                } else {
                    write!(out, ":{name}")
                }
            }
            Self::BackRef => write!(out, "&"),
        }
    }
}

enum BadSelector0 {
    Value,
    Parse(ParseError),
}
impl BadSelector0 {
    fn ctx(self, v: Value) -> BadSelector {
        match self {
            Self::Value => BadSelector::Value(v),
            Self::Parse(err) => BadSelector::Parse(err),
        }
    }
}
impl From<ParseError> for BadSelector0 {
    fn from(e: ParseError) -> Self {
        Self::Parse(e)
    }
}

/// The error when a [Value] cannot be converted to a [Selectors] or [Selector].
#[derive(Debug)]
pub enum BadSelector {
    /// The value was not the expected type of list or string.
    Value(Value),
    /// There was an error parsing a string value.
    Parse(ParseError),
    /// A backref (`&`) were present but not allowed there.
    Backref(SourcePos),
    /// Cant append extenstion to base.
    Append(OldSelector, OldSelector),
}

impl fmt::Display for BadSelector {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Value(v) => out.write_str(&is_not(
                v,
                "a valid selector: it must be a string,\
                 \na list of strings, or a list of lists of strings",
            )),
            Self::Parse(e) => e.fmt(out),
            Self::Backref(pos) => {
                writeln!(out, "Parent selectors aren\'t allowed here.")?;
                pos.show(out)
            }
            Self::Append(e, b) => {
                write!(out, "Can't append {e} to {b}.")
            }
        }
    }
}
impl From<ParseError> for BadSelector {
    fn from(e: ParseError) -> Self {
        Self::Parse(e)
    }
}
