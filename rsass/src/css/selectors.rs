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
use crate::parser::css::{selector, selector_parts, selectors};
use crate::parser::{input_span, ParseError};
use crate::value::ListSeparator;
use nom::combinator::all_consuming;
use std::fmt;
use std::io::Write;

mod attribute;
mod cssselectorset;
mod logical;
mod pseudo;
mod selectorset;

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

    /// Get the first of these selectors (or the root selector if empty).
    pub(crate) fn one(&self) -> OldSelector {
        self.s.first().cloned().unwrap_or_else(OldSelector::root)
    }

    /// Create the full selector for when self is used inside a parent selector.
    pub(crate) fn inside(&self, parent: &OldSelectorCtx) -> Self {
        OldSelectorCtx::from(self.clone()).inside(parent).real()
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

    /// Get these selectors with a specific backref selector.
    ///
    /// Used to create `@at-root` contexts, to have `&` work in them.
    pub(crate) fn with_backref(self, context: OldSelector) -> OldSelectorCtx {
        OldSelectorCtx::from(self).inside(&OldSelectorCtx {
            s: Self::root(),
            backref: context,
        })
    }
    /// Return true if any of these selectors ends with a combinator
    pub fn has_trailing_combinator(&self) -> bool {
        self.s.iter().any(OldSelector::has_trailing_combinator)
    }
}

impl From<OldSelectors> for Value {
    /// Create a css `Value` representing a set of selectors.
    ///
    /// The result will be a comma-separated [list](Value::List) of
    /// space-separated lists of strings, or [null](Value::Null) if
    /// this is a root (empty) selector.
    fn from(sel: OldSelectors) -> Self {
        if sel.is_root() {
            return Self::Null;
        }
        let content = sel
            .s
            .iter()
            .map(|s: &OldSelector| {
                let (mut v, last) = s.0.iter().fold(
                    (vec![], Option::<String>::None),
                    |(mut v, mut last), part| {
                        match part {
                            OldSelectorPart::Descendant => {
                                if let Some(last) = last.take() {
                                    v.push(last.into());
                                }
                            }
                            OldSelectorPart::RelOp(op) => {
                                if let Some(last) = last.take() {
                                    v.push(last.into());
                                }
                                v.push(char::from(*op).to_string().into());
                            }
                            part => {
                                last = Some(match last {
                                    Some(last) => format!("{last}{part}"),
                                    None => part.to_string(),
                                });
                            }
                        }
                        (v, last)
                    },
                );
                if let Some(last) = last {
                    v.push(last.into());
                }
                Self::List(v, Some(ListSeparator::Space), false)
            })
            .collect::<Vec<_>>();
        Self::List(content, Some(ListSeparator::Comma), false)
    }
}

/// A full set of selectors with a separate backref.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct OldSelectorCtx {
    /// The actual selectors.
    s: OldSelectors,
    backref: OldSelector,
}

impl OldSelectorCtx {
    /// Create a root (empty) selector.
    pub fn root() -> Self {
        OldSelectors::root().into()
    }
    pub(crate) fn root_with_backref(context: OldSelector) -> Self {
        Self {
            s: OldSelectors::root(),
            backref: context,
        }
    }
    /// Return true if this is a root (empty) selector.
    pub fn is_root(&self) -> bool {
        self.s.is_root() && self.backref == OldSelector::root()
    }

    // I think any backrefs is guaranteed to be already resolved in self.s!
    pub(crate) fn real_new(&self) -> CssSelectorSet {
        if self.s.is_root() {
            return CssSelectorSet {
                s: SelectorSet {
                    s: vec![Selector::default()],
                },
            };
        }
        let span = input_span(self.s.to_string());
        let (_, value) = all_consuming(parser::selector_set)(span.borrow())
            .unwrap_or_else(|e| panic!("Bad selector {span:?}: {e}"));
        let msg = format!("Bad real selectors {value:?}");
        value.try_into().expect(&msg)
    }
    pub(crate) fn real(&self) -> OldSelectors {
        self.s.clone()
    }

    /// Remove the first of these selectors (or the root selector if empty).
    pub(crate) fn one(&self) -> OldSelector {
        self.s.one()
    }

    /// Create the full selector for when self is used inside a parent selector.
    pub(crate) fn inside(&self, parent: &Self) -> Self {
        let mut result = Vec::new();
        for p in &parent.s.s {
            for s in &self.s.s {
                result.push(p.join(s, &parent.backref));
            }
        }
        Self {
            s: OldSelectors::new(result),
            backref: parent.backref.clone(),
        }
    }
}

impl From<OldSelectors> for OldSelectorCtx {
    fn from(value: OldSelectors) -> Self {
        Self {
            s: value,
            backref: OldSelector::root(),
        }
    }
}

impl TryFrom<Value> for OldSelectorCtx {
    type Error = BadSelector;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        OldSelectors::try_from(v).map(Into::into)
    }
}
impl TryFrom<Value> for OldSelectors {
    type Error = BadSelector;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        value_to_selectors(&v).map_err(move |e| e.ctx(v))
    }
}
fn value_to_selectors(v: &Value) -> Result<OldSelectors, BadSelector0> {
    match v {
        Value::List(vv, s, _) => match s {
            Some(ListSeparator::Comma) => {
                let vv = vv
                    .iter()
                    .map(value_to_selector)
                    .collect::<Result<_, _>>()?;
                Ok(OldSelectors::new(vv))
            }
            Some(ListSeparator::Space) => {
                let (mut outer, last) = vv.iter().try_fold(
                    (vec![], vec![]),
                    |(mut outer, mut a), v: &Value| {
                        if let Ok(ref mut s) = check_selector_str(v) {
                            push_descendant(&mut a, s);
                        } else {
                            let mut s = parse_selectors_str(v)?;
                            if let Some(f) = s.s.first_mut() {
                                push_descendant(&mut a, f);
                                std::mem::swap(&mut a, &mut f.0);
                            }
                            if let Some(last) = s.s.pop() {
                                a = last.0;
                            }
                            outer.extend(s.s);
                        }
                        Result::<_, BadSelector0>::Ok((outer, a))
                    },
                )?;
                outer.push(OldSelector(last));
                Ok(OldSelectors::new(outer))
            }
            _ => Err(BadSelector0::Value),
        },
        Value::Literal(s) => {
            if s.value().is_empty() {
                Ok(OldSelectors::root())
            } else {
                let span = input_span(s.value());
                Ok(ParseError::check(selectors(span.borrow()))?)
            }
        }
        _ => Err(BadSelector0::Value),
    }
}

fn check_selector_str(v: &Value) -> Result<OldSelector, BadSelector0> {
    match v {
        Value::Literal(s) => {
            if s.value().is_empty() {
                Ok(OldSelector::root())
            } else {
                let span = input_span(s.value());
                Ok(ParseError::check(selector(span.borrow()))?)
            }
        }
        _ => Err(BadSelector0::Value),
    }
}
fn parse_selectors_str(v: &Value) -> Result<OldSelectors, BadSelector0> {
    match v {
        Value::Literal(s) => {
            if s.value().is_empty() {
                Ok(OldSelectors::root())
            } else {
                let span = input_span(s.value());
                Ok(ParseError::check(selectors(span.borrow()))?)
            }
        }
        _ => Err(BadSelector0::Value),
    }
}

fn push_descendant(to: &mut Vec<OldSelectorPart>, from: &mut OldSelector) {
    if !to.is_empty() {
        to.push(OldSelectorPart::Descendant);
    }
    to.append(&mut from.0);
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

    fn join(&self, other: &Self, alt_context: &Self) -> Self {
        if other.has_backref() {
            let mut result = Vec::new();
            let context = if self.0.is_empty() { alt_context } else { self };
            for p in &other.0 {
                result.extend(p.clone_in(context));
            }
            Self(result)
        } else {
            let mut result = self.0.clone();
            if !result.is_empty()
                && !other
                    .0
                    .first()
                    .map_or(false, OldSelectorPart::is_operator)
            {
                result.push(OldSelectorPart::Descendant);
            }
            result.extend(other.0.iter().cloned());
            Self(result)
        }
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

impl TryFrom<Value> for OldSelector {
    type Error = BadSelector;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        value_to_selector(&value).map_err(move |e| e.ctx(value))
    }
}
// Internal, the api is try_into.
fn value_to_selector(v: &Value) -> Result<OldSelector, BadSelector0> {
    match v {
        Value::List(list, None | Some(ListSeparator::Space), _) => {
            list_to_selector(list)
        }
        Value::Literal(s) => {
            if s.value().is_empty() {
                Ok(OldSelector::root())
            } else {
                let span = input_span(s.value());
                Ok(ParseError::check(selector(span.borrow()))?)
            }
        }
        _ => Err(BadSelector0::Value),
    }
}

fn list_to_selector(list: &[Value]) -> Result<OldSelector, BadSelector0> {
    list.iter()
        .try_fold(vec![], |mut a, v| {
            let parts = value_to_selector_parts(v)?;
            if !parts.first().map_or(true, OldSelectorPart::is_operator)
                && !a.last().map_or(true, OldSelectorPart::is_operator)
            {
                a.push(OldSelectorPart::Descendant);
            }
            a.extend(parts);
            Ok(a)
        })
        .map(OldSelector)
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
    fn clone_in(&self, context: &OldSelector) -> Vec<Self> {
        match self {
            s @ (Self::Descendant
            | Self::RelOp(_)
            | Self::Simple(_)
            | Self::Attribute { .. }) => vec![s.clone()],
            Self::BackRef => context.0.clone(),
            Self::PseudoElement { name, arg } => {
                vec![Self::PseudoElement {
                    name: name.clone(),
                    arg: arg.as_ref().map(|a| {
                        a.inside(&OldSelectorCtx::root_with_backref(
                            context.clone(),
                        ))
                    }),
                }]
            }
            Self::Pseudo { name, arg } => {
                vec![Self::Pseudo {
                    name: name.clone(),
                    arg: arg.as_ref().map(|a| {
                        a.inside(&OldSelectorCtx::root_with_backref(
                            context.clone(),
                        ))
                    }),
                }]
            }
        }
    }
}

fn value_to_selector_parts(
    v: &Value,
) -> Result<Vec<OldSelectorPart>, BadSelector0> {
    match v {
        Value::Literal(s) => Ok(ParseError::check(selector_parts(
            input_span(s.value()).borrow(),
        ))?),
        _ => Err(BadSelector0::Value),
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn root_join() {
        let s = OldSelector(vec![OldSelectorPart::Simple("foo".into())]);
        assert_eq!(OldSelector::root().join(&s, &OldSelector::root()), s)
    }

    #[test]
    fn simple_join() {
        let s = OldSelector(vec![OldSelectorPart::Simple("foo".into())])
            .join(
                &OldSelector(vec![OldSelectorPart::Simple(".bar".into())]),
                &OldSelector::root(),
            );
        assert_eq!(format!("{}", s), "foo .bar")
    }

    #[test]
    fn backref_join() {
        let s = OldSelector(vec![OldSelectorPart::Simple("foo".into())])
            .join(
                &OldSelector(vec![
                    OldSelectorPart::BackRef,
                    OldSelectorPart::Simple(".bar".into()),
                ]),
                &OldSelector::root(),
            );
        assert_eq!(format!("{}", s), "foo.bar")
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
