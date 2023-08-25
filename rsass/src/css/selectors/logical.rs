//! A logical selector is a css selector, but representend in a way
//! that I hope make implementing the sass selector functions easier.
//!
//! In the future, I might use this as the primary (only) css selector
//! implementation.  But as that is a major breaking change, I keep
//! these types internal for now.
use super::{BadSelector, SelectorPart, Selectors};
use crate::css::{CssString, Value};
use crate::parser::input_span;
use std::mem::swap;

/// A set of selectors.
/// This is the normal top-level selector, which can be a single
/// [`Selector`] or a comma-separated list (set) of such selectors.
pub(crate) struct SelectorSet {
    s: Vec<Selector>,
}

impl SelectorSet {
    pub fn is_superselector(&self, other: &Self) -> bool {
        other
            .s
            .iter()
            .all(|sub| self.s.iter().any(|sup| sup.is_superselector(sub)))
    }
}

impl TryFrom<Value> for SelectorSet {
    type Error = BadSelector;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        Self::try_from(&Selectors::try_from(value)?.css_ok()?)
    }
}
impl TryFrom<&Selectors> for SelectorSet {
    type Error = BadSelector;

    fn try_from(value: &Selectors) -> Result<Self, Self::Error> {
        value
            .s
            .iter()
            .map(Selector::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map(|s| SelectorSet { s })
    }
}

/// A selector more aimed at making it easy to implement selector functions.
///
/// A logical selector is fully resolved (cannot contain an `&` backref).
#[derive(Debug, Default)]
struct Selector {
    element: Option<String>,
    classes: Vec<String>,
    id: Option<String>,
    attr: Vec<Attribute>,
    pseudo: Vec<Pseudo>,
    pseudo_element: Option<Pseudo>,
    rel_of: Option<Box<(RelKind, Selector)>>,
}

impl Selector {
    /// Return true iff this selector is a superselector of `sub`.
    fn is_superselector(&self, sub: &Self) -> bool {
        self.element
            .iter()
            .all(|e| elem_matches(e, sub.element.as_deref().unwrap_or("*")))
            && self
                .classes
                .iter()
                .all(|ac| sub.classes.iter().any(|bc| ac == bc))
            && self.id.iter().all(|id| sub.id.as_ref() == Some(id))
            && self
                .attr
                .iter()
                .all(|aa| sub.attr.iter().any(|ba| aa.is_superselector(ba)))
            && self
                .pseudo
                .iter()
                .all(|aa| sub.pseudo.iter().any(|ba| aa.is_superselector(ba)))
            && dbg!(self.pseudo_element.as_ref()).map_or_else(
                || sub.pseudo_element.is_none(),
                |aa| {
                    dbg!(sub.pseudo_element.as_ref())
                        .map_or(false, |ba| aa.is_superselector(ba))
                },
            )
            && self.rel_of.as_deref().map_or(true, |(kind, s)| {
                match dbg!(kind) {
                    RelKind::Ancestor => {
                        let mut t = sub.rel_of.as_deref();
                        while let Some((ss_kind, ss)) = t {
                            match ss_kind {
                                RelKind::Ancestor | RelKind::Parent => {
                                    if dbg!(s).is_superselector(dbg!(ss)) {
                                        return true;
                                    }
                                    // else keep looking up the hieararchy!
                                }
                                RelKind::Sibling
                                | RelKind::AdjacentSibling => (), // the siblings parent is our parent
                            }
                            t = ss.rel_of.as_deref();
                        }
                        false
                    }
                    RelKind::Parent => {
                        let t = sub.rel_of.as_deref();
                        if let Some((RelKind::Parent, ss)) = t {
                            dbg!(s).is_superselector(dbg!(ss))
                        } else {
                            false
                        }
                    }
                    RelKind::Sibling => {
                        let mut t = sub.rel_of.as_deref();
                        while let Some((ss_kind, ss)) = t {
                            if ss_kind != &RelKind::Sibling
                                && ss_kind != &RelKind::AdjacentSibling
                            {
                                return false;
                            }
                            if dbg!(s).is_superselector(dbg!(ss)) {
                                return true;
                            }
                            t = ss.rel_of.as_deref();
                        }
                        false
                    }
                    RelKind::AdjacentSibling => {
                        // Only the closest relative can be an adjacent sibling
                        if let Some((RelKind::AdjacentSibling, ss)) =
                            sub.rel_of.as_deref()
                        {
                            dbg!(s).is_superselector(dbg!(ss))
                        } else {
                            false
                        }
                    }
                }
            })
    }
}

fn elem_matches(e: &str, sub: &str) -> bool {
    let (e_ns, e_name) = split_ns(e);
    let (sub_ns, sub_name) = split_ns(sub);
    match_name(e_ns, sub_ns) && match_name(e_name, sub_name)
}

fn match_name(a: &str, b: &str) -> bool {
    a == "*" || a == b
}

fn split_ns(name: &str) -> (&str, &str) {
    let mut e = name.splitn(2, '|');
    match (e.next(), e.next()) {
        (Some(ns), Some(elem)) => (ns, elem),
        (Some(elem), None) => ("*", elem),
        _ => unreachable!(),
    }
}

impl TryFrom<&super::Selector> for Selector {
    type Error = BadSelector;

    fn try_from(value: &super::Selector) -> Result<Self, Self::Error> {
        Selector::try_from(&value.0[..])
    }
}

impl TryFrom<&[SelectorPart]> for Selector {
    type Error = BadSelector;

    fn try_from(value: &[SelectorPart]) -> Result<Self, Self::Error> {
        let mut result = Self::default();
        for part in value {
            match part {
                SelectorPart::Simple(part) => {
                    if let Some(cls) = part.strip_prefix('.') {
                        result.classes.push(cls.into());
                    } else if let Some(id) = part.strip_prefix('#') {
                        result.id = Some(id.into());
                    } else {
                        result.element.replace(part.into());
                    }
                }
                SelectorPart::Attribute {
                    name,
                    op,
                    val,
                    modifier,
                } => {
                    result.attr.push(Attribute {
                        name: name.clone(),
                        op: op.clone(),
                        val: val.clone(),
                        modifier: *modifier,
                    });
                }
                SelectorPart::Pseudo { name, arg } => {
                    if arg.is_none() && is_pseudo_element(name.value()) {
                        let mut t = Self::default();
                        swap(&mut result, &mut t);
                        result.rel_of = Some(Box::new((RelKind::Parent, t)));
                        result.element = Some(format!(":{name}"));
                    } else {
                        result.pseudo.push(Pseudo {
                            name: name.clone(),
                            arg: arg.clone(),
                        });
                    }
                }
                SelectorPart::PseudoElement { name, arg } => {
                    assert!(result.pseudo_element.is_none());
                    result.pseudo_element = Some(Pseudo {
                        name: name.clone(),
                        arg: arg.clone(),
                    });
                }
                SelectorPart::Descendant => {
                    let mut t = Self::default();
                    swap(&mut result, &mut t);
                    result.rel_of = Some(Box::new((RelKind::Ancestor, t)));
                }
                SelectorPart::RelOp(op) => {
                    let kind = match op {
                        b'+' => RelKind::AdjacentSibling,
                        b'~' => RelKind::Sibling,
                        b'>' => RelKind::Parent,
                        op => {
                            eprintln!(
                                "WARNING: Unsupported css relation {op:?}. \
                                 Treating it as '>'"
                            );
                            RelKind::Parent
                        }
                    };
                    let mut t = Self::default();
                    swap(&mut result, &mut t);
                    result.rel_of = Some(Box::new((kind, t)));
                }
                SelectorPart::BackRef => {
                    // I hope backrefs should be resolved before here.
                    // The real span should be retained in the selector.
                    return Err(BadSelector::Backref(input_span("&")));
                }
            }
        }
        Ok(result)
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

#[derive(Debug, Eq, PartialEq)]
enum RelKind {
    Ancestor,
    Parent,
    Sibling,
    AdjacentSibling,
}

/// A logical attribute selector.
#[derive(Debug)]
struct Attribute {
    /// The attribute name
    // TODO: Why not a raw String?
    name: CssString,
    /// An operator
    op: String,
    /// A value to match.
    val: CssString,
    /// Optional modifier.
    modifier: Option<char>,
}

impl Attribute {
    fn is_superselector(&self, b: &Self) -> bool {
        self.name == b.name
            && self.op == b.op
            && self.val == b.val
            && self.modifier == b.modifier
    }
}

/// A pseudo-class or a css2 pseudo-element (:foo)
#[derive(Debug)]
struct Pseudo {
    /// The name of the pseudo-class
    name: CssString,
    /// Arguments to the pseudo-class
    arg: Option<Selectors>,
}

impl Pseudo {
    fn is_superselector(&self, b: &Self) -> bool {
        fn map_sel(s: &Option<Selectors>) -> Option<SelectorSet> {
            s.as_ref().and_then(|s| SelectorSet::try_from(s).ok())
        }
        if self.name != b.name {
            return false;
        }
        // Note: A better implementetation of is/matches/any would be
        // different from has, host, and host-context.
        if name_in(
            self.name.value(),
            &[
                "is",
                "matches",
                "any",
                "where",
                "has",
                "host",
                "host-context",
            ],
        ) {
            if let (Some(a), Some(b)) = (map_sel(&self.arg), map_sel(&b.arg))
            {
                a.is_superselector(&b)
            } else {
                false
            }
        } else if name_in(self.name.value(), &["not"]) {
            if let (Some(a), Some(b)) = (map_sel(&self.arg), map_sel(&b.arg))
            {
                b.is_superselector(&a) // NOTE: Reversed!
            } else {
                false
            }
        } else {
            self.name == b.name && self.arg == b.arg
        }
    }
}

fn name_in(name: &str, known: &[&str]) -> bool {
    for end in known {
        if name == *end
            || (name.starts_with('-') && name.ends_with(&format!("-{end}")))
        {
            return true;
        }
    }
    false
}
