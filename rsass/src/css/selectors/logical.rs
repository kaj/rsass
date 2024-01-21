//! A logical selector is a css selector, but representend in a way
//! that I hope make implementing the sass selector functions easier.
//!
//! In the future, I might use this as the primary (only) css selector
//! implementation.  But as that is a major breaking change, I keep
//! these types internal for now.
use super::attribute::Attribute;
use super::pseudo::Pseudo;
use super::selectorset::SelectorSet;
use super::{BadSelector, BadSelector0, CssSelectorSet};
use crate::css::Value;
use crate::parser::input_span;
use crate::sass::CallError;
use crate::value::ListSeparator;
use crate::{Invalid, ParseError};
use lazy_static::lazy_static;
use std::iter::once;

type RelBox = Box<(RelKind, Selector)>;

/// A selector more aimed at making it easy to implement selector functions.
///
/// A logical selector is fully resolved (cannot contain an `&` backref).
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(crate) struct Selector {
    backref: Option<()>,
    element: Option<ElemType>,
    placeholders: Vec<String>,
    classes: Vec<String>,
    id: Option<String>,
    attr: Vec<Attribute>,
    pseudo: Vec<Pseudo>,
    rel_of: Option<RelBox>,
}

impl Selector {
    pub(super) fn append(&self, other: &Self) -> Result<Self, AppendError> {
        if self.is_local_empty() {
            return Err(AppendError::Parent);
        }

        let mut result = other.to_owned();
        if let Some(rel) = result.rel_of.take() {
            result.rel_of = Some(Box::new((rel.0, self.append(&rel.1)?)));
            Ok(result)
        } else {
            let rel = self.rel_of.clone();
            if result.is_local_empty() {
                return Err(AppendError::Sub);
            }
            let s = self.clone().last_compound_str();
            let other = result.last_compound_str();
            if other
                .bytes()
                .next()
                .map_or(true, |c| c == b'*' || c == b'|')
            {
                return Err(AppendError::Sub);
            }
            let s = s + &other;
            let span = input_span(s);
            let mut result =
                ParseError::check(parser::compound_selector(span.borrow()))?;
            result.rel_of = rel;
            Ok(result)
        }
    }

    pub(super) fn extend(
        self,
        extendee: &SelectorSet,
        extender: &SelectorSet,
    ) -> Vec<Self> {
        let mut result = vec![self];
        for original in &extendee.s {
            result = result
                .into_iter()
                .flat_map(|mut s| {
                    if original.is_superselector(&s) {
                        let base = s.clone();
                        if original.element == s.element
                            && !s
                                .element
                                .as_ref()
                                .map_or(true, |e| e.s == "*")
                        {
                            s.element = None;
                        }
                        s.placeholders.retain(|p| {
                            !original.placeholders.iter().any(|o| o == p)
                        });
                        if original.id == s.id {
                            s.id = None;
                        }
                        s.attr.retain(|a| {
                            !original.attr.iter().any(|o| a == o)
                        });
                        s.classes.retain(|c| {
                            !original.classes.iter().any(|o| c == o)
                        });
                        s.pseudo.retain(|p| {
                            !original.pseudo.iter().any(|o| p == o)
                        });
                        let mut result = extender
                            .s
                            .iter()
                            .flat_map(|r| s.unify_extend(r))
                            .collect::<Vec<_>>();
                        result.retain(|r| !base.is_superselector(r));
                        result.insert(0, base);
                        result
                    } else {
                        vec![s]
                    }
                })
                .collect();
        }
        result
    }

    pub(super) fn nest(&self, other: &Self) -> Self {
        let mut result = other.clone();
        if let Some(rel) = result.rel_of.take() {
            result.rel_of = Some(Box::new((rel.0, self.nest(&rel.1))));
        } else {
            result.rel_of = Some(Box::new((RelKind::Ancestor, self.clone())));
        }
        result
    }

    pub(super) fn resolve_ref(mut self, ctx: &CssSelectorSet) -> Vec<Self> {
        self = self.resolve_ref_in_pseudo(ctx);
        let rel_of = self.rel_of.take();

        let result = if self.backref.is_some() {
            self.backref = None;
            ctx.s
                .s
                .iter()
                .flat_map(|s| {
                    let mut slf = self.clone();
                    let mut s = s.clone();
                    slf.element = match (s.element.take(), slf.element.take())
                    {
                        (None, None) => None,
                        (Some(e), None) | (None, Some(e)) => Some(e),
                        (Some(a), Some(b)) => Some(ElemType {
                            s: format!("{}{}", a.s, b.s),
                        }),
                    };
                    slf.unify(s)
                })
                .collect()
        } else {
            vec![self]
        };
        if let Some(rel_of) = rel_of {
            let rels = rel_of.1.resolve_ref(ctx);
            rels.into_iter()
                .flat_map(|rel| {
                    result
                        .iter()
                        .map(|r| {
                            let mut r = r.to_owned();
                            let mut t = &mut r.rel_of;
                            loop {
                                if let Some(next) = t {
                                    t = &mut next.1.rel_of;
                                } else {
                                    let _ = t.insert(Box::new((
                                        rel_of.0,
                                        rel.clone(),
                                    )));
                                    break;
                                }
                            }
                            r
                        })
                        .collect::<Vec<_>>()
                })
                .collect()
        } else {
            result
        }
    }

    pub(super) fn resolve_ref_in_pseudo(
        mut self,
        ctx: &CssSelectorSet,
    ) -> Self {
        self.rel_of = self.rel_of.map(|mut rel| {
            rel.1 = rel.1.resolve_ref_in_pseudo(ctx);
            rel
        });
        self.pseudo = self
            .pseudo
            .into_iter()
            .map(|p| p.resolve_ref(ctx))
            .collect();
        self
    }
    /// Return true iff this selector is a superselector of `sub`.
    pub(super) fn is_superselector(&self, sub: &Self) -> bool {
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

    pub(super) fn replace(
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

    pub(super) fn unify(self, other: Self) -> Vec<Self> {
        self._unify(other).unwrap_or_default()
    }

    pub fn unify_extend(&self, other: &Self) -> Vec<Self> {
        self.clone()._unify(other.clone()).unwrap_or_default()
    }

    fn _unify(mut self, mut other: Self) -> Option<Vec<Self>> {
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
            && (self.pseudo.iter().any(Pseudo::is_hover)
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

    /// Return true if this is a complex selector (has any relation).
    pub(super) fn is_complex(&self) -> bool {
        self.rel_of.is_some()
    }

    fn is_local_empty(&self) -> bool {
        self.element.is_none()
            && self.backref.is_none()
            && self.placeholders.is_empty()
            && self.classes.is_empty()
            && self.id.is_none()
            && self.attr.is_empty()
            && self.pseudo.is_empty()
    }

    pub(super) fn has_backref(&self) -> bool {
        self.backref.is_some()
            || self.pseudo.iter().any(Pseudo::has_backref)
            || self
                .rel_of
                .as_deref()
                .map_or(false, |(_, s)| s.has_backref())
    }

    fn add_root_ancestor(&mut self, ancestor: Self) {
        if let Some(rel) = self.rel_of.as_mut() {
            if rel.1.is_local_empty() && !rel.1.is_complex() {
                rel.1 = ancestor;
            } else {
                rel.1.add_root_ancestor(ancestor);
            }
        } else {
            self.rel_of = Some(Box::new((RelKind::Ancestor, ancestor)));
        }
    }

    fn with_rel_of(mut self, rel: RelKind, other: Self) -> Vec<Self> {
        if self.rel_of.is_some() {
            self.unify(Self {
                rel_of: Some(Box::new((rel, other))),
                ..Self::default()
            })
        } else if self.pseudo.iter().any(Pseudo::is_rootish) {
            vec![]
        } else {
            self.rel_of = Some(Box::new((rel, other)));
            vec![self]
        }
    }

    pub(crate) fn simple_selectors(&self) -> Result<Vec<String>, Invalid> {
        if self.rel_of.is_some() {
            return Err(Invalid::AtError(
                "Combinators not allowed in simple-selectors.".into(),
            ));
        }
        let mut result = Vec::new();
        if let Some(element) = &self.element {
            result.push(element.s.clone());
        }
        result.extend(self.placeholders.iter().map(|p| format!("%{p}")));
        result.extend(self.classes.iter().map(|c| format!(".{c}")));
        result.extend(self.id.iter().map(|id| format!("#{id}")));
        result.extend(self.attr.iter().map(|a| {
            let mut s = String::new();
            a.write_to_buf(&mut s);
            s
        }));
        result.extend(self.pseudo.iter().map(|p| {
            let mut s = String::new();
            p.write_to_buf(&mut s);
            s
        }));
        Ok(result)
    }

    pub(super) fn into_string_vec(mut self) -> Vec<String> {
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
        let last = self.last_compound_str();
        if !last.is_empty() {
            vec.push(last);
        }
        vec
    }

    fn last_compound_str(self) -> String {
        let mut buf = String::new();
        if self.backref.is_some() {
            buf.push('&');
        }
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

    /// Internal (the api is [`TryFrom`]).
    pub(super) fn _try_from_value(v: &Value) -> Result<Self, BadSelector0> {
        match v {
            Value::List(list, None | Some(ListSeparator::Space), _) => {
                list.iter()
                    .try_fold(None, |a, v| {
                        let mut s = match v {
                            Value::Literal(s) => {
                                ParseError::check(parser::selector(
                                    input_span(s.value()).borrow(),
                                ))?
                            }
                            _ => return Err(BadSelector0::Value),
                        };
                        // TODO: Handle operator at end of a!
                        if let Some(a) = a {
                            s.add_root_ancestor(a);
                        }
                        Ok(Some(s))
                    })
                    .map(Option::unwrap_or_default)
            }
            Value::Literal(s) => {
                if s.value().is_empty() {
                    Ok(Self::default())
                } else {
                    let span = input_span(s.value());
                    Ok(ParseError::check(parser::selector(span.borrow()))?)
                }
            }
            _ => Err(BadSelector0::Value),
        }
    }
}

pub(super) enum AppendError {
    Parent,
    Sub,
    Selector(BadSelector),
}

impl AppendError {
    pub(super) fn context(self, e: &Selector, b: &Selector) -> CallError {
        match self {
            AppendError::Parent => {
                CallError::msg(
                    format!(
                        "Selector {:?} can't be used as a parent in a compound selector.",
                        show(b)
                    )
                )
            },
            AppendError::Sub => {
                CallError::msg(
                    format!("Can't append {} to {}.", show(e), show(b))
                )
            },
            AppendError::Selector(b) => b.into(),
        }
    }
}

fn show(s: &Selector) -> String {
    s.clone().into_string_vec().join(" ")
}

impl From<ParseError> for AppendError {
    fn from(value: ParseError) -> Self {
        AppendError::Selector(value.into())
    }
}

impl From<BadSelector> for AppendError {
    fn from(value: BadSelector) -> Self {
        AppendError::Selector(value)
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
        Self::_try_from_value(&value).map_err(move |e| e.ctx(value))
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

#[derive(Clone, Debug, PartialEq, Eq)]
struct ElemType {
    s: String,
}

impl Default for ElemType {
    fn default() -> Self {
        ElemType { s: "*".into() }
    }
}

impl ElemType {
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

pub(crate) mod parser {
    use super::super::attribute::parser::attribute;
    use super::super::pseudo::parser::pseudo;
    use super::{ElemType, RelKind, Selector};
    use crate::parser::css::strings::css_string_nohash as css_string;
    use crate::parser::util::{opt_spacelike, spacelike};
    use crate::parser::{PResult, Span};
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::combinator::{map, opt, value, verify};
    use nom::multi::fold_many0;
    use nom::sequence::{delimited, pair, preceded};

    pub(crate) fn selector(input: Span) -> PResult<Selector> {
        let (input, prerel) = opt(rel_kind)(input)?;
        let (input, first) = if let Some(prerel) = prerel {
            let (input, first) = opt(compound_selector)(input)?;
            let mut first = first.unwrap_or_default();
            first.rel_of = Some(Box::new((prerel, Selector::default())));
            (input, first)
        } else {
            compound_selector(input)?
        };
        fold_many0(
            pair(rel_kind, opt(compound_selector)),
            move || first.clone(),
            |rel, (kind, e)| {
                let mut e = e.unwrap_or_default();
                e.rel_of = Some(Box::new((kind, rel)));
                e
            },
        )(input)
    }

    fn rel_kind(input: Span) -> PResult<RelKind> {
        alt((
            delimited(
                opt_spacelike,
                alt((
                    value(RelKind::AdjacentSibling, tag("+")),
                    value(RelKind::Sibling, tag("~")),
                    value(RelKind::Parent, tag(">")),
                )),
                opt_spacelike,
            ),
            value(RelKind::Ancestor, spacelike),
        ))(input)
    }

    pub(crate) fn compound_selector(input: Span) -> PResult<Selector> {
        let mut result = Selector::default();
        let (rest, backref) = opt(value((), tag("&")))(input)?;
        result.backref = backref;
        let (mut rest, elem) = opt(name_opt_ns)(rest)?;
        result.element = elem.map(|s| ElemType { s });

        loop {
            rest = match rest.first() {
                Some(b'#') => {
                    let (r, id) = preceded(tag("#"), css_string)(rest)?;
                    result.id = Some(id);
                    r
                }
                Some(b'%') => {
                    let (r, p) = preceded(tag("%"), css_string)(rest)?;
                    result.placeholders.push(p);
                    r
                }
                Some(b'.') => {
                    let (r, c) = preceded(tag("."), css_string)(rest)?;
                    result.classes.push(c);
                    r
                }
                Some(b':') => {
                    let (r, p) = pseudo(rest)?;
                    result.pseudo.push(p);
                    r
                }
                Some(b'[') => {
                    let (r, c) = attribute(rest)?;
                    result.attr.push(c);
                    r
                }
                _ => break,
            };
        }
        verify(tag(""), |_| !result.is_local_empty())(rest)?;
        Ok((rest, result))
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
