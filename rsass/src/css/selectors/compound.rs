use super::{Attribute, CssSelectorSet, ElemType, Opt, Pseudo, SelectorSet};
use crate::{output::CssBuf, parser::input_span, ParseError};
use lazy_static::lazy_static;
use std::fmt;

#[derive(Default, Clone, PartialEq, Eq)]
pub(crate) struct CompoundSelector {
    pub(super) backref: Option<()>,
    element: Option<ElemType>,
    placeholders: Vec<String>,
    classes: Vec<String>,
    id: Option<String>,
    attr: Vec<Attribute>,
    pseudo: Vec<Pseudo>,
}

impl CompoundSelector {
    pub fn is_empty(&self) -> bool {
        self.element.is_none()
            && self.backref.is_none()
            && self.placeholders.is_empty()
            && self.classes.is_empty()
            && self.id.is_none()
            && self.attr.is_empty()
            && self.pseudo.is_empty()
    }

    pub(super) fn has_backref(&self) -> bool {
        self.backref.is_some() || self.pseudo.iter().any(Pseudo::has_backref)
    }
    pub(super) fn has_id(&self) -> bool {
        self.id.is_some()
    }
    pub(super) fn cant_append(&self) -> bool {
        self.is_empty()
            || self.element.as_ref().map_or(false, ElemType::cant_append)
    }
    pub(super) fn append(&self, other: &Self) -> Result<Self, ParseError> {
        let mut s = CssBuf::new(Default::default());
        self.write_to(&mut s);
        other.write_to(&mut s);
        let s = s.take();
        if s.is_empty() {
            Ok(Self::default())
        } else {
            let span = input_span(s);
            Ok(ParseError::check(parser::compound_selector(span.borrow()))?)
        }
    }

    pub(super) fn is_rootish(&self) -> bool {
        self.pseudo.iter().any(Pseudo::is_rootish)
    }

    /// Return true if these compound selectors can't meaningfully
    /// appear in the same selector.
    pub(super) fn must_not_inherit(&self, other: &Self) -> bool {
        if self.id.is_some() && self.id == other.id {
            return true;
        }
        if let Some(pseudo) = self.pseudo_element() {
            if other.pseudo_element() == Some(pseudo) {
                return true;
            }
        }
        false
    }

    pub fn no_placeholder(&self) -> Opt<Self> {
        if !self.placeholders.is_empty() {
            return Opt::None;
        }
        let pseudo = match Opt::collect_neg(
            self.pseudo.iter().map(Pseudo::no_placeholder),
        ) {
            Opt::Some(p) => p,
            Opt::Any => vec![],
            Opt::None => return Opt::None,
        };
        Opt::Some(Self {
            pseudo,
            ..self.clone()
        })
    }

    pub(super) fn dedup(&mut self, original: &Self) {
        if original.element == self.element
            && !self.element.as_ref().map_or(true, ElemType::is_any)
        {
            self.element = None;
        }
        self.placeholders
            .retain(|p| !original.placeholders.iter().any(|o| o == p));
        if original.id == self.id {
            self.id = None;
        }
        self.attr.retain(|a| !original.attr.iter().any(|o| a == o));
        self.classes
            .retain(|c| !original.classes.iter().any(|o| c == o));
        self.pseudo
            .retain(|p| !original.pseudo.iter().any(|o| p == o));
    }

    pub(super) fn resolve_ref_in_pseudo(&mut self, ctx: &CssSelectorSet) {
        self.pseudo =
            self.pseudo.drain(..).map(|p| p.resolve_ref(ctx)).collect();
    }

    pub(super) fn replace_in_pseudo(
        &mut self,
        original: &SelectorSet,
        replacement: &SelectorSet,
    ) {
        self.pseudo = self
            .pseudo
            .drain(..)
            .map(|p| p.replace(original, replacement))
            .collect();
    }

    pub fn is_superselector(&self, sub: &Self) -> bool {
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

    pub fn pseudo_element(&self) -> Option<&Pseudo> {
        self.pseudo.iter().find(|p| p.is_element())
    }

    pub(crate) fn simple_selectors(&self) -> Vec<String> {
        let mut result = Vec::new();
        if let Some(element) = &self.element {
            result.push(element.to_string());
        }
        result.extend(self.placeholders.iter().map(|p| format!("%{p}")));
        result.extend(self.classes.iter().map(|c| format!(".{c}")));
        result.extend(self.id.iter().map(|id| format!("#{id}")));
        result.extend(self.attr.iter().map(|a| {
            let mut s = CssBuf::new(Default::default());
            a.write_to(&mut s);
            String::from_utf8_lossy(&s.take()).to_string()
        }));
        result.extend(self.pseudo.iter().map(|p| {
            let mut s = CssBuf::new(Default::default());
            p.write_to(&mut s);
            String::from_utf8_lossy(&s.take()).to_string()
        }));
        result
    }

    pub fn write_to(&self, buf: &mut CssBuf) {
        if self.backref.is_some() {
            buf.add_char('&');
        }
        if let Some(e) = &self.element {
            if !e.is_any()
                || (self.classes.is_empty()
                    && self.placeholders.is_empty()
                    && self.id.is_none()
                    && self.pseudo.is_empty())
            {
                e.write_to(buf);
            }
        }
        for p in &self.placeholders {
            buf.add_char('%');
            buf.add_str(p);
        }
        if let Some(id) = &self.id {
            buf.add_char('#');
            buf.add_str(id);
        }
        for c in &self.classes {
            buf.add_char('.');
            buf.add_str(c);
        }
        for attr in &self.attr {
            attr.write_to(buf);
        }
        for pseudo in &self.pseudo {
            pseudo.write_to(buf);
        }
    }
    pub fn unify(mut self, mut other: Self) -> Option<Self> {
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
                || !self.classes.is_empty())
        {
            return None;
        }
        Some(self)
    }
}

impl fmt::Debug for CompoundSelector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("CompoundSelector");
        if self.backref.is_some() {
            s.field("backref", &"&");
        }
        if let Some(elem) = &self.element {
            s.field("element", &elem);
        }
        if !self.placeholders.is_empty() {
            s.field("placeholders", &self.placeholders);
        }
        if let Some(id) = &self.id {
            s.field("id", &id);
        }
        if !self.classes.is_empty() {
            s.field("classes", &self.classes);
        }
        if !self.attr.is_empty() {
            s.field("attr", &self.attr);
        }
        if !self.pseudo.is_empty() {
            s.field("pseudo", &self.pseudo);
        }
        s.finish()
    }
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

pub(crate) mod parser {
    use super::super::parser::{attribute, elem_name, keyframe_stop, pseudo};
    use super::CompoundSelector;
    use crate::parser::css::strings::css_string_nohash as css_string;
    use crate::parser::{PResult, Span};
    use nom::bytes::complete::tag;
    use nom::combinator::{opt, value, verify};
    use nom::sequence::preceded;

    pub(crate) fn compound_selector(
        input: Span,
    ) -> PResult<CompoundSelector> {
        let mut result = CompoundSelector::default();
        if let PResult::Ok((rest, stop)) = keyframe_stop(input) {
            result.element = Some(stop);
            return Ok((rest, result));
        }
        let (rest, backref) = opt(value((), tag("&")))(input)?;
        result.backref = backref;
        let (mut rest, elem) = opt(elem_name)(rest)?;
        result.element = elem;

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
        verify(tag(""), |_| !result.is_empty())(rest)?;
        Ok((rest, result))
    }
}
