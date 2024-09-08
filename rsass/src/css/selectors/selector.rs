use super::compound::CompoundSelector;
use super::error::BadSelector0;
use super::{BadSelector, CssSelectorSet, Opt, SelectorSet};
use crate::css::Value;
use crate::output::CssBuf;
use crate::parser::input_span;
use crate::sass::CallError;
use crate::value::ListSeparator;
use crate::{Invalid, ParseError};
use core::fmt;
use std::iter::once;

type RelBox = Box<(RelKind, Selector)>;

/// A css selector.
///
/// A selector is a sequence of compound selectors, joined by
/// relational operators (where the "ancestor" relation is just
/// whitespace in the text representation).
#[derive(Default, Clone, PartialEq, Eq)]
pub struct Selector {
    rel_of: Option<RelBox>,
    compound: CompoundSelector,
}

impl Selector {
    pub(crate) fn no_placeholder(&self) -> Opt<Self> {
        let compound = match self.compound.no_placeholder() {
            Opt::Some(compound) => compound,
            Opt::Any => CompoundSelector::default(),
            Opt::None => return Opt::None,
        };
        if self.is_local_empty() && self.rel_of.is_some() {
            eprintln!("Deprecated dobule empty relation");
            return Opt::None;
        }

        let rel_of = if let Some((kind, rel)) = self.rel_of.as_deref() {
            match rel.no_placeholder() {
                Opt::Some(rel) => Some(Box::new((*kind, rel))),
                Opt::Any => None,
                Opt::None => return Opt::None,
            }
        } else {
            None
        };
        Opt::Some(Self { rel_of, compound })
    }
    pub(crate) fn no_leading_combinator(&self) -> Opt<Self> {
        if self.has_leading_combinator() {
            Opt::None
        } else {
            Opt::Some(self.clone())
        }
    }
    fn has_leading_combinator(&self) -> bool {
        self.rel_of.as_deref().map_or(false, |(_k, r)| {
            if r.rel_of.is_none() {
                r.is_local_empty()
            } else {
                r.has_leading_combinator()
            }
        })
    }

    pub(super) fn append(&self, other: &Self) -> Result<Self, AppendError> {
        if self.is_local_empty() {
            Err(AppendError::Parent)
        } else if let Some(rel) = other.rel_of.clone() {
            Ok(Self {
                rel_of: Some(Box::new((rel.0, self.append(&rel.1)?))),
                compound: other.compound.clone(),
            })
        } else if other.compound.cant_append() {
            Err(AppendError::Sub)
        } else {
            Ok(Self {
                rel_of: self.rel_of.clone(),
                compound: self.compound.append(&other.compound)?,
            })
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
                        s.compound.dedup(&original.compound);
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
        if !self.is_local_empty() || self.rel_of.is_some() {
            result.rel_of = match result.rel_of.take().map(|b| *b) {
                Some((kind, rel)) => {
                    let rel = self.nest(&rel);
                    if rel.is_local_empty() {
                        match rel.rel_of.map(|b| *b) {
                            Some((rk, rr)) => match (kind, rk) {
                                (kind, RelKind::Ancestor) => {
                                    Some(Box::new((kind, rr)))
                                }
                                (kind, rk) => Some(Box::new((
                                    kind,
                                    Selector {
                                        rel_of: Some(Box::new((rk, rr))),
                                        ..Default::default()
                                    },
                                ))),
                            },
                            None => None,
                        }
                    } else {
                        Some(Box::new((kind, rel)))
                    }
                }
                None => Some(Box::new((RelKind::Ancestor, self.clone()))),
            };
        }
        result
    }

    pub(super) fn resolve_ref(mut self, ctx: &CssSelectorSet) -> Vec<Self> {
        self = self.resolve_ref_in_pseudo(ctx);
        let rel_of = self.rel_of.take();

        let result = if self.compound.backref.is_some() {
            self.compound.backref = None;
            ctx.s
                .s
                .iter()
                .flat_map(|s| {
                    Selector {
                        rel_of: s.rel_of.clone(),
                        compound: CompoundSelector::default(),
                    }
                    .unify(Selector {
                        rel_of: self.rel_of.clone(),
                        compound: s.compound.append(&self.compound).unwrap(),
                    })
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
        self.compound.resolve_ref_in_pseudo(ctx);
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
        self.compound.is_superselector(&sub.compound)
    }

    pub(super) fn replace(
        mut self,
        original: &SelectorSet,
        replacement: &SelectorSet,
    ) -> Vec<Self> {
        self.compound.replace_in_pseudo(original, replacement);

        let mut result = vec![self];
        for original in &original.s {
            result = result
                .into_iter()
                .flat_map(|mut s| {
                    if original.is_superselector(&s) {
                        s.compound.dedup(&original.compound);
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

    pub(crate) fn unify_extend(&self, other: &Self) -> Vec<Self> {
        self.clone()._unify(other.clone()).unwrap_or_default()
    }

    fn _unify(self, other: Self) -> Option<Vec<Self>> {
        let rel_of = match (self.rel_of, other.rel_of) {
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
        let compound = self.compound.unify(other.compound)?;

        Some(if rel_of.is_empty() {
            vec![compound.into()]
        } else if compound.is_empty() {
            vec![]
        } else if rel_of.len() == 1 {
            let mut rel_of = rel_of;
            vec![Selector {
                rel_of: rel_of.pop(),
                compound,
            }]
        } else {
            rel_of
                .into_iter()
                .map(|r| Selector {
                    rel_of: Some(r),
                    compound: compound.clone(),
                })
                .collect()
        })
    }

    /// Return true if this is a complex selector (has any relation).
    pub(super) fn is_complex(&self) -> bool {
        self.rel_of.is_some()
    }

    fn is_local_empty(&self) -> bool {
        self.compound.is_empty()
    }

    pub(super) fn has_backref(&self) -> bool {
        self.compound.has_backref()
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
        } else if self.compound.is_rootish() {
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
        Ok(self.compound.simple_selectors())
    }

    /// Write this `Selector` to a formatted buffer.
    pub fn write_to(&self, buf: &mut CssBuf) {
        if let Some((kind, sel)) = self.rel_of.as_deref() {
            sel.write_to(buf);
            if let Some(symbol) = kind.symbol() {
                if !sel.is_local_empty() {
                    buf.add_one(" ", "");
                }
                buf.add_str(symbol);
                buf.add_one(" ", "");
            } else {
                buf.add_str(" ");
            }
        }
        self.compound.write_to(buf);
    }

    pub(super) fn into_string_vec(self) -> Vec<String> {
        let mut vec = if let Some((kind, sel)) = self.rel_of.map(|b| *b) {
            let mut vec = sel.into_string_vec();
            if let Some(symbol) = kind.symbol() {
                vec.push(symbol.to_string());
            }
            vec
        } else {
            Vec::new()
        };
        if !self.compound.is_empty() {
            let mut buf = CssBuf::new(Default::default());
            self.compound.write_to(&mut buf);
            vec.push(String::from_utf8_lossy(&buf.take()).to_string());
        }
        vec
    }

    /// Internal (the api is [`TryFrom`]).
    pub(super) fn _try_from_value(v: &Value) -> Result<Self, BadSelector0> {
        match v {
            Value::List(list, None | Some(ListSeparator::Space), _) => list
                .iter()
                .try_fold(None, |a, v| {
                    let mut s = match v {
                        Value::Literal(s) => ParseError::check(
                            parser::selector(input_span(s.value()).borrow()),
                        )?,
                        _ => return Err(BadSelector0::Value),
                    };
                    if let Some(a) = a {
                        s.add_root_ancestor(a);
                    }
                    Ok(Some(s))
                })
                .map(Option::unwrap_or_default),
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

impl From<CompoundSelector> for Selector {
    fn from(compound: CompoundSelector) -> Self {
        Self {
            rel_of: None,
            compound,
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
            Self::Parent => {
                CallError::msg(
                    format!(
                        "Selector {:?} can't be used as a parent in a compound selector.",
                        show(b)
                    )
                )
            },
            Self::Sub => {
                CallError::msg(
                    format!("Can't append {} to {}.", show(e), show(b))
                )
            },
            Self::Selector(b) => b.into(),
        }
    }
}

fn show(s: &Selector) -> String {
    s.clone().into_string_vec().join(" ")
}

impl From<ParseError> for AppendError {
    fn from(value: ParseError) -> Self {
        Self::Selector(value.into())
    }
}

impl From<BadSelector> for AppendError {
    fn from(value: BadSelector) -> Self {
        Self::Selector(value)
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
    if a.0 == b.0 && a.1.compound.is_rootish() && b.1.compound.is_rootish() {
        return Some(as_rel_vec(a.0, b.1.unify(a.1)));
    }

    Some(match (*a, *b) {
        ((k @ AdjacentSibling, a), (AdjacentSibling, b))
        | ((k @ Parent, a), (Parent, b)) => as_rel_vec(k, b._unify(a)?),
        ((Ancestor, a), (Ancestor, b)) => {
            if b.is_local_superselector(&a) {
                as_rel_vec(Ancestor, a._unify(b)?)
            } else if a.is_local_superselector(&b)
                || a.compound.must_not_inherit(&b.compound)
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
            } else if !a.compound.must_not_inherit(&b.compound) {
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
            } else if a_s.compound.has_id() || b_s.compound.has_id() {
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

impl TryFrom<Value> for Selector {
    type Error = BadSelector;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        Self::_try_from_value(&value).map_err(move |e| e.ctx(value))
    }
}

impl From<Selector> for Value {
    fn from(value: Selector) -> Self {
        Self::List(
            value
                .into_string_vec()
                .into_iter()
                .map(Self::from)
                .collect(),
            Some(ListSeparator::Space),
            false,
        )
    }
}

impl fmt::Debug for Selector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("Selector");
        if let Some((kind, rel)) = self.rel_of.as_deref() {
            s.field(&format!("{kind:?}"), &rel);
        }
        s.field("compound", &NoAlt { t: &self.compound });
        s.finish()
    }
}
struct NoAlt<T> {
    t: T,
}
impl<T: fmt::Debug> fmt::Debug for NoAlt<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.t)
    }
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
            Self::Ancestor => None,
            Self::Parent => Some(">"),
            Self::Sibling => Some("~"),
            Self::AdjacentSibling => Some("+"),
        }
    }
}

pub(crate) mod parser {
    use super::super::parser::compound_selector;
    use super::{RelKind, Selector};
    use crate::parser::util::{opt_spacelike, spacelike};
    use crate::parser::{PResult, Span};
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::combinator::{into, opt, value, verify};
    use nom::multi::fold_many0;
    use nom::sequence::{delimited, pair, preceded, terminated};

    pub(crate) fn selector(input: Span) -> PResult<Selector> {
        let (input, prerel) =
            preceded(opt_spacelike, opt(explicit_rel_kind))(input)?;
        let (input, first) = if let Some(prerel) = prerel {
            let (input, first) = opt(compound_selector)(input)?;
            let first = Selector {
                rel_of: Some(Box::new((prerel, Selector::default()))),
                compound: first.unwrap_or_default(),
            };
            (input, first)
        } else {
            into(compound_selector)(input)?
        };
        terminated(
            fold_many0(
                verify(pair(rel_kind, opt(compound_selector)), |(k, s)| {
                    k.symbol().is_some() || s.is_some()
                }),
                move || first.clone(),
                |rel, (kind, e)| Selector {
                    compound: e.unwrap_or_default(),
                    rel_of: Some(Box::new((kind, rel))),
                },
            ),
            opt_spacelike,
        )(input)
    }

    fn explicit_rel_kind(input: Span) -> PResult<RelKind> {
        delimited(
            opt_spacelike,
            alt((
                value(RelKind::AdjacentSibling, tag("+")),
                value(RelKind::Sibling, tag("~")),
                value(RelKind::Parent, tag(">")),
            )),
            opt_spacelike,
        )(input)
    }

    fn rel_kind(input: Span) -> PResult<RelKind> {
        alt((explicit_rel_kind, value(RelKind::Ancestor, spacelike)))(input)
    }
}
