//! A logical selector is a css selector, but representend in a way
//! that I hope make implementing the sass selector functions easier.
//!
//! In the future, I might use this as the primary (only) css selector
//! implementation.  But as that is a major breaking change, I keep
//! these types internal for now.
use super::{BadSelector, SelectorPart, Selectors};
use crate::css::{CssString, Value};
use crate::parser::input_span;
use crate::value::ListSeparator;
use crate::Invalid;
use lazy_static::lazy_static;
use std::iter::once;
use std::mem::swap;

/// A set of selectors.
/// This is the normal top-level selector, which can be a single
/// [`Selector`] or a comma-separated list (set) of such selectors.
#[derive(Clone, Debug)]
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
    pub fn replace(
        self,
        original: &SelectorSet,
        replacement: &SelectorSet,
    ) -> Result<Self, Invalid> {
        for original in &original.s {
            if original.rel_of.is_some() {
                let s = original.clone().into_string_vec().join(" ");
                return Err(Invalid::AtError(format!(
                    "Can\'t extend complex selector {s}."
                )));
            }
        }
        let result = self
            .s
            .into_iter()
            .flat_map(|s| s.replace(original, replacement))
            .collect();
        Ok(Self { s: result })
    }

    pub fn unify(self, other: SelectorSet) -> SelectorSet {
        SelectorSet {
            s: self
                .s
                .into_iter()
                .flat_map(|s| {
                    other
                        .s
                        .iter()
                        .flat_map(move |o| s.clone().unify(o.clone()))
                })
                .collect(),
        }
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
impl TryFrom<SelectorSet> for Selectors {
    type Error = BadSelector;

    fn try_from(value: SelectorSet) -> Result<Self, Self::Error> {
        Value::from(value).try_into()
    }
}

impl From<SelectorSet> for Value {
    fn from(value: SelectorSet) -> Self {
        let mut v = value.s.into_iter().map(Value::from).collect::<Vec<_>>();
        match v.len() {
            0 => Value::Null,
            1 => v.pop().unwrap(),
            _ => Value::List(v, Some(ListSeparator::Comma), false),
        }
    }
}

type RelBox = Box<(RelKind, Selector)>;

/// A selector more aimed at making it easy to implement selector functions.
///
/// A logical selector is fully resolved (cannot contain an `&` backref).
#[derive(Debug, Default, Clone)]
pub(crate) struct Selector {
    element: Option<ElemType>,
    placeholders: Vec<String>,
    classes: Vec<String>,
    id: Option<String>,
    attr: Vec<Attribute>,
    pseudo: Vec<Pseudo>,
    rel_of: Option<RelBox>,
}

impl Selector {
    /// Return true iff this selector is a superselector of `sub`.
    fn is_superselector(&self, sub: &Self) -> bool {
        self.is_local_superselector(sub)
            && self.rel_of.as_deref().map_or(true, |(kind, s)| {
                match kind {
                    RelKind::Ancestor => {
                        let mut t = sub.rel_of.as_deref();
                        while let Some((ss_kind, ss)) = t {
                            match ss_kind {
                                RelKind::Ancestor | RelKind::Parent => {
                                    if s.is_superselector(ss) {
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
                            s.is_superselector(ss)
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
                            if s.is_superselector(ss) {
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
                            s.is_superselector(ss)
                        } else {
                            false
                        }
                    }
                }
            })
    }

    fn is_local_superselector(&self, sub: &Self) -> bool {
        fn elem_or_default(e: &Option<ElemType>) -> &ElemType {
            lazy_static! {
                static ref DEF: ElemType = ElemType::default();
            }
            e.as_ref().unwrap_or(&DEF)
        }
        elem_or_default(&self.element)
            .is_superselector(elem_or_default(&sub.element))
            && all_any(&self.placeholders, &sub.placeholders, PartialEq::eq)
            && all_any(&self.classes, &sub.classes, PartialEq::eq)
            && self.id.iter().all(|id| sub.id.as_ref() == Some(id))
            && all_any(&self.attr, &sub.attr, Attribute::is_superselector)
            && all_any(&self.pseudo, &sub.pseudo, Pseudo::is_superselector)
            && self.pseudo_element().as_ref().map_or_else(
                || sub.pseudo_element().is_none(),
                |aa| {
                    sub.pseudo_element()
                        .as_ref()
                        .map_or(false, |ba| aa.is_superselector(ba))
                },
            )
    }

    fn replace(
        mut self,
        original: &SelectorSet,
        replacement: &SelectorSet,
    ) -> Vec<Self> {
        self.pseudo = self
            .pseudo
            .into_iter()
            .map(|p| p.replace(original, replacement))
            .collect();

        let mut result = vec![self];
        for original in &original.s {
            result = result
                .into_iter()
                .flat_map(|mut s| {
                    if original.is_superselector(&s) {
                        if original.element == s.element {
                            s.element = None;
                        }
                        s.classes.retain(|c| {
                            !original.classes.iter().any(|o| c == o)
                        });
                        replacement
                            .s
                            .iter()
                            .flat_map(|r| s.clone().unify(r.clone()))
                            .collect()
                    } else {
                        vec![s]
                    }
                })
                .collect();
        }
        result
    }

    fn unify(self, other: Selector) -> Vec<Selector> {
        self._unify(other).unwrap_or_default()
    }

    fn _unify(mut self, mut other: Selector) -> Option<Vec<Selector>> {
        let rel_of = match (self.rel_of.take(), other.rel_of.take()) {
            (None, None) => vec![],
            (None, Some(rel)) | (Some(rel), None) => vec![rel],
            (Some(a), Some(b)) => {
                let v = unify_relbox(a, b)?;
                if v.is_empty() {
                    return None;
                }
                v
            }
        };
        let pseudo_element =
            match (self.pseudo_element(), other.pseudo_element()) {
                (None, None) => None,
                (Some(pe), None) | (None, Some(pe)) => Some(pe),
                (Some(a), Some(b)) => {
                    if b.is_superselector(a) {
                        Some(a)
                    } else if a.is_superselector(b) {
                        Some(b)
                    } else {
                        return None;
                    }
                }
            }
            .cloned();
        self.pseudo.retain(|p| !p.is_element());
        other.pseudo.retain(|p| !p.is_element());

        self.element = match (self.element, other.element) {
            (None, None) => None,
            (None, Some(e)) | (Some(e), None) => Some(e),
            (Some(a), Some(b)) => Some(a.unify(&b)?),
        };
        for c in other.placeholders {
            if !self.placeholders.iter().any(|sc| sc == &c) {
                self.placeholders.push(c);
            }
        }
        for c in other.classes {
            if !self.classes.iter().any(|sc| sc == &c) {
                self.classes.push(c);
            }
        }
        self.id = match (self.id, other.id) {
            (None, None) => None,
            (None, Some(id)) | (Some(id), None) => Some(id),
            (Some(s_id), Some(o_id)) => {
                if s_id == o_id {
                    Some(s_id)
                } else {
                    return None;
                }
            }
        };

        combine_vital(
            &mut self.attr,
            &mut other.attr,
            Attribute::is_superselector,
        );
        combine_vital(
            &mut self.pseudo,
            &mut other.pseudo,
            Pseudo::is_superselector,
        );
        if let Some(pseudo_element) = pseudo_element {
            self.pseudo.push(pseudo_element);
        }
        if self.pseudo.iter().any(Pseudo::is_host)
            && (self.pseudo.iter().any(|p| p.name.value() == "hover")
                || self.element.is_some()
                || !self.classes.is_empty()
                || !rel_of.is_empty())
        {
            return None;
        }

        Some(if rel_of.is_empty() {
            vec![self]
        } else if self.is_local_empty() {
            vec![]
        } else if rel_of.len() == 1 {
            let mut rel_of = rel_of;
            self.rel_of = Some(rel_of.pop().unwrap());
            vec![self]
        } else {
            rel_of
                .into_iter()
                .map(|r| {
                    let mut t = self.clone();
                    t.rel_of = Some(r);
                    t
                })
                .collect()
        })
    }

    fn is_local_empty(&self) -> bool {
        self.element.is_none()
            && self.placeholders.is_empty()
            && self.classes.is_empty()
            && self.id.is_none()
            && self.attr.is_empty()
            && self.pseudo.is_empty()
    }

    fn with_rel_of(mut self, rel: RelKind, other: Selector) -> Vec<Selector> {
        if self.rel_of.is_some() {
            self.unify(Selector {
                rel_of: Some(Box::new((rel, other))),
                ..Default::default()
            })
        } else if self.pseudo.iter().any(Pseudo::is_rootish) {
            vec![]
        } else {
            self.rel_of = Some(Box::new((rel, other)));
            vec![self]
        }
    }

    fn into_string_vec(mut self) -> Vec<String> {
        let mut vec =
            if let Some((kind, sel)) = self.rel_of.take().map(|b| *b) {
                let mut vec = sel.into_string_vec();
                if let Some(symbol) = kind.symbol() {
                    vec.push(symbol.to_string());
                }
                vec
            } else {
                Vec::new()
            };
        vec.push(self.last_compound_str());
        vec
    }

    fn last_compound_str(self) -> String {
        let mut buf = String::new();
        if let Some(e) = self.element {
            if !e.is_any()
                || (self.classes.is_empty()
                    && self.placeholders.is_empty()
                    && self.id.is_none()
                    && self.attr.is_empty()
                    && self.pseudo.is_empty())
            {
                buf.push_str(&e.s);
            }
        }
        for p in &self.placeholders {
            buf.push('%');
            buf.push_str(p);
        }
        if let Some(id) = &self.id {
            buf.push('#');
            buf.push_str(id);
        }
        for c in &self.classes {
            buf.push('.');
            buf.push_str(c);
        }
        for attr in &self.attr {
            attr.write_to_buf(&mut buf);
        }
        for pseudo in &self.pseudo {
            pseudo.write_to_buf(&mut buf);
        }
        buf
    }

    fn pseudo_element(&self) -> Option<&Pseudo> {
        self.pseudo.iter().find(|p| p.is_element())
    }
}

// NOTE: I think returning an empty vector here should be equivalent
// to returning None, so maybe just return a Vec and check for
// emtpyness at the call site?
#[allow(clippy::boxed_local)]
fn unify_relbox(a: RelBox, b: RelBox) -> Option<Vec<RelBox>> {
    use RelKind::*;
    fn as_rel_vec(
        kind: RelKind,
        s: impl IntoIterator<Item = Selector>,
    ) -> Vec<RelBox> {
        s.into_iter().map(|s| Box::new((kind, s))).collect()
    }
    if a.0 == b.0
        && a.1.pseudo.iter().any(Pseudo::is_rootish)
        && b.1.pseudo.iter().any(Pseudo::is_rootish)
    {
        return Some(as_rel_vec(a.0, b.1.unify(a.1)));
    }

    Some(match (*a, *b) {
        ((k @ AdjacentSibling, a), (AdjacentSibling, b))
        | ((k @ Parent, a), (Parent, b)) => as_rel_vec(k, b._unify(a)?),
        ((Ancestor, a), (Ancestor, b)) => {
            if b.is_local_superselector(&a) {
                as_rel_vec(Ancestor, a._unify(b)?)
            } else if a.is_local_superselector(&b)
                || have_same(&a.id, &b.id)
                || have_same(&a.pseudo_element(), &b.pseudo_element())
            {
                as_rel_vec(Ancestor, b._unify(a)?)
            } else {
                as_rel_vec(
                    Ancestor,
                    b.clone()
                        .with_rel_of(Ancestor, a.clone())
                        .into_iter()
                        .chain(a.with_rel_of(Ancestor, b)),
                )
            }
        }
        ((k @ Sibling, a), (Sibling, b)) => {
            if a.is_superselector(&b) {
                as_rel_vec(Sibling, once(b))
            } else if b.is_superselector(&a) {
                as_rel_vec(Sibling, once(a))
            } else if !have_same(&a.id, &b.id) {
                as_rel_vec(
                    Sibling,
                    b.clone()
                        .with_rel_of(k, a.clone())
                        .into_iter()
                        .chain(a.clone().with_rel_of(k, b.clone()))
                        .chain(b.unify(a)),
                )
            } else {
                as_rel_vec(Sibling, b.unify(a))
            }
        }
        ((a_k @ AdjacentSibling, a_s), (Sibling, b_s))
        | ((Sibling, b_s), (a_k @ AdjacentSibling, a_s)) => {
            if b_s.is_superselector(&a_s) {
                as_rel_vec(a_k, once(a_s))
            } else if a_s.id.is_some() || b_s.id.is_some() {
                as_rel_vec(a_k, a_s.with_rel_of(Sibling, b_s))
            } else {
                as_rel_vec(
                    a_k,
                    a_s.clone()
                        .with_rel_of(Sibling, b_s.clone())
                        .into_iter()
                        .chain(a_s.unify(b_s)),
                )
            }
        }
        ((a_k @ (AdjacentSibling | Sibling), a_s), (b_k, b_s))
        | ((b_k, b_s), (a_k @ (AdjacentSibling | Sibling), a_s)) => {
            as_rel_vec(a_k, a_s.with_rel_of(b_k, b_s))
        }
        ((Parent, p), (Ancestor, a)) | ((Ancestor, a), (Parent, p)) => {
            if a.is_superselector(&p) {
                as_rel_vec(Parent, once(p))
            } else {
                as_rel_vec(Parent, p.with_rel_of(RelKind::Ancestor, a))
            }
        }
    })
}

fn have_same<T: Eq>(one: &Option<T>, other: &Option<T>) -> bool {
    one.is_some() && one == other
}

/// Return true if all elements of `one` has an element in `other` for
/// whitch `cond` is true.
fn all_any<T, F>(one: &[T], other: &[T], cond: F) -> bool
where
    F: Fn(&T, &T) -> bool,
{
    one.iter().all(|a| other.iter().any(|b| cond(a, b)))
}

// Combine all alements from `v` that is not made redundant by `other`
// with those from `other` that is not redunant with `v`, into `v`
// (leaving `other` empty).
fn combine_vital<T, Q>(v: &mut Vec<T>, other: &mut Vec<T>, q: Q)
where
    Q: Fn(&T, &T) -> bool,
{
    v.retain(|a| !other.iter().any(|b| q(b, a)));
    other.retain(|a| !v.iter().any(|b| q(b, a)));
    v.append(other);
}

impl TryFrom<Value> for Selector {
    type Error = BadSelector;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        Self::try_from(&super::Selector::try_from(value)?.css_ok()?)
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
                    } else if let Some(ph) = part.strip_prefix('%') {
                        result.placeholders.push(ph.into());
                    } else {
                        // FIXME: Check that it was none before!
                        result.element = Some(ElemType::new(part));
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
                    result.pseudo.push(Pseudo {
                        name: name.clone(),
                        arg: arg.clone(),
                        element: false,
                    });
                }
                SelectorPart::PseudoElement { name, arg } => {
                    assert!(result.pseudo_element().is_none()); // ?
                    result.pseudo.push(Pseudo {
                        name: name.clone(),
                        arg: arg.clone(),
                        element: true,
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

impl From<Selector> for Value {
    fn from(value: Selector) -> Self {
        Value::List(
            value
                .into_string_vec()
                .into_iter()
                .map(Value::from)
                .collect(),
            Some(ListSeparator::Space),
            false,
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
struct ElemType {
    s: String,
}

impl Default for ElemType {
    fn default() -> Self {
        ElemType { s: "*".into() }
    }
}

impl ElemType {
    fn new(s: &str) -> Self {
        ElemType { s: s.to_owned() }
    }
    fn is_any(&self) -> bool {
        self.s == "*"
    }

    fn is_superselector(&self, sub: &Self) -> bool {
        let (e_ns, e_name) = self.split_ns();
        let (sub_ns, sub_name) = sub.split_ns();
        match_name(e_ns.unwrap_or("*"), sub_ns.unwrap_or("*"))
            && match_name(e_name, sub_name)
    }

    fn unify(&self, other: &Self) -> Option<Self> {
        let (e_ns, e_name) = self.split_ns();
        let (o_ns, o_name) = other.split_ns();
        let ns = match (e_ns, o_ns) {
            (None, None) => None,
            (Some("*"), ns) => ns,
            (ns, Some("*")) => ns,
            (Some(e), Some(o)) if e == o => Some(e),
            _ => return None,
        };
        let name = match (e_name, o_name) {
            ("*", name) => name,
            (name, "*") => name,
            (e, o) if e == o => e,
            _ => return None,
        };
        Some(ElemType {
            s: if let Some(ns) = ns {
                format!("{ns}|{name}")
            } else {
                name.to_string()
            },
        })
    }

    fn split_ns(&self) -> (Option<&str>, &str) {
        let mut e = self.s.splitn(2, '|');
        match (e.next(), e.next()) {
            (Some(ns), Some(elem)) => (Some(ns), elem),
            (Some(elem), None) => (None, elem),
            _ => unreachable!(),
        }
    }
}

fn match_name(a: &str, b: &str) -> bool {
    a == "*" || a == b
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RelKind {
    Ancestor,
    Parent,
    Sibling,
    AdjacentSibling,
}

impl RelKind {
    fn symbol(self) -> Option<&'static str> {
        match self {
            RelKind::Ancestor => None,
            RelKind::Parent => Some(">"),
            RelKind::Sibling => Some("~"),
            RelKind::AdjacentSibling => Some("+"),
        }
    }
}

/// A logical attribute selector.
#[derive(Debug, Clone)]
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

    fn write_to_buf(&self, buf: &mut String) {
        buf.push('[');
        use std::fmt::Write;
        write!(buf, "{}{}{}", self.name, self.op, self.val).unwrap();
        if let Some(m) = self.modifier {
            buf.push(' ');
            buf.push(m);
        }
        buf.push(']');
    }
}

/// A pseudo-class or a css2 pseudo-element (:foo)
#[derive(Debug, Clone, PartialEq, Eq)]
struct Pseudo {
    /// The name of the pseudo-class
    name: CssString,
    /// Arguments to the pseudo-class
    arg: Option<Selectors>,
    /// True if this is a `::psedu-element`, false for a `:pseudo-class`.
    element: bool,
}

impl Pseudo {
    fn is_superselector(&self, b: &Self) -> bool {
        fn map_sel(s: &Option<Selectors>) -> Option<SelectorSet> {
            s.as_ref().and_then(|s| SelectorSet::try_from(s).ok())
        }
        if self.is_element() != b.is_element() || self.name != b.name {
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
            if self.arg == b.arg {
                true
            } else if let (Some(a), Some(b)) =
                (map_sel(&self.arg), map_sel(&b.arg))
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

    fn is_element(&self) -> bool {
        self.element || is_pseudo_element(self.name.value())
    }
    fn is_host(&self) -> bool {
        name_in(self.name.value(), &["host", "host-context"])
    }
    fn is_rootish(&self) -> bool {
        name_in(
            self.name.value(),
            &["host", "host-context", "root", "scope"],
        )
    }

    pub fn replace(
        mut self,
        original: &SelectorSet,
        replacement: &SelectorSet,
    ) -> Self {
        if name_in(
            self.name.value(),
            &[
                "is",
                "matches",
                "not",
                "any",
                "where",
                "has",
                "host",
                "host-context",
            ],
        ) {
            self.arg = self.arg.map(|s| {
                SelectorSet::try_from(&s)
                    .unwrap()
                    .replace(original, replacement)
                    .unwrap()
                    .try_into()
                    .unwrap()
            });
        }
        self
    }

    fn write_to_buf(&self, buf: &mut String) {
        buf.push(':');
        if self.element {
            buf.push(':');
        }
        buf.push_str(self.name.value());
        if let Some(arg) = &self.arg {
            use std::fmt::Write;
            write!(buf, "({arg})").unwrap();
        }
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
