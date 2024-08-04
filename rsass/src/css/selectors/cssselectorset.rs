use super::selectorset::SelectorSet;
use super::{BadSelector, BadSelector0};
use crate::css::Value;
use crate::error::Invalid;
use crate::output::CssBuf;
use crate::parser::{input_span, ParseError};
use crate::sass::CallError;
use crate::value::ListSeparator;
use nom::Finish;

/// A `CssSelectorset` is like a [`Selectorset`] but valid in css.
///
/// The practical difference is that a `CssSelectorset` is guaranteed
/// not to contain backrefs (`&`), which may be present in a
/// `Selectorset`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CssSelectorSet {
    pub(super) s: SelectorSet,
}

impl CssSelectorSet {
    pub(crate) fn root() -> Self {
        Self {
            s: SelectorSet::root(),
        }
    }
    /// Return true if this is a root (empty) selector.
    pub fn is_root(&self) -> bool {
        self.s.is_root()
    }

    pub fn parse_value(value: Value) -> Result<Self, BadSelector> {
        fn join(value: &Value) -> Result<String, BadSelector0> {
            if let Value::List(vs, Some(ListSeparator::Comma), false) = value
            {
                vs.iter()
                    .map(join2)
                    .collect::<Result<Vec<_>, _>>()
                    .map(|v| v.join(", "))
            } else {
                join2(value)
            }
        }
        fn join2(value: &Value) -> Result<String, BadSelector0> {
            if let Value::List(vs, Some(ListSeparator::Space) | None, false) =
                value
            {
                vs.iter()
                    .map(join3)
                    .collect::<Result<Vec<_>, _>>()
                    .map(|v| v.join(" "))
            } else {
                join3(value)
            }
        }
        fn join3(value: &Value) -> Result<String, BadSelector0> {
            match value {
                Value::Literal(s) => Ok(s.value().to_string()),
                _ => Err(BadSelector0::Value),
            }
        }
        let selector = join(&value).map_err(|e| e.ctx(value))?;
        let span = input_span(selector);

        let (rest, value) = super::parser::selector_set(span.borrow())
            .finish()
            .map_err(ParseError::from)?;
        if rest.fragment().is_empty() {
            value.try_into()
        } else {
            Err(ParseError::new("expected selector.", rest.to_owned()).into())
        }
    }

    pub fn is_superselector(&self, sub: &Self) -> bool {
        self.s.is_superselector(&sub.s)
    }

    pub(crate) fn append(self, ext: Self) -> Result<Self, CallError> {
        Ok(Self {
            s: SelectorSet {
                s: self
                    .s
                    .s
                    .into_iter()
                    .flat_map(|b| {
                        ext.s.s.iter().map(move |e| {
                            b.append(e).map_err(|err| err.context(e, &b))
                        })
                    })
                    .collect::<Result<_, _>>()?,
            },
        })
    }

    pub(crate) fn extend(
        self,
        extendee: &Self,
        extender: &Self,
    ) -> Result<Self, Invalid> {
        self.s.extend(&extendee.s, &extender.s).map(|s| Self { s })
    }

    /// Nest `other` selectors inside this as though they were nested
    /// within one another in the stylesheet.
    pub(crate) fn nest(&self, other: SelectorSet) -> Self {
        let mut parts = other
            .s
            .into_iter()
            .map(|o| {
                if o.has_backref() {
                    o.resolve_ref(self)
                } else {
                    self.s.s.iter().map(|s| s.nest(&o)).collect()
                }
            })
            .map(Vec::into_iter)
            .collect::<Vec<_>>();

        let mut result = Vec::new();
        let mut empty = false;
        while !empty {
            empty = true;
            for i in &mut parts {
                if let Some(next) = i.next() {
                    result.push(next);
                    empty = false;
                }
            }
        }

        Self {
            s: SelectorSet { s: result },
        }
    }

    pub(crate) fn replace(
        self,
        original: &Self,
        replacement: &Self,
    ) -> Result<Self, Invalid> {
        self.s
            .replace(&original.s, &replacement.s)
            .map(|s| Self { s })
    }

    pub(crate) fn unify(self, other: Self) -> Self {
        Self {
            s: SelectorSet {
                s: self
                    .s
                    .s
                    .into_iter()
                    .flat_map(|s| {
                        other
                            .s
                            .s
                            .iter()
                            .flat_map(move |o| s.clone().unify(o.clone()))
                    })
                    .collect(),
            },
        }
    }

    pub fn write_to(&self, buf: &mut CssBuf) {
        self.s.write_to(buf)
    }
}

impl From<CssSelectorSet> for SelectorSet {
    fn from(value: CssSelectorSet) -> Self {
        value.s
    }
}

impl TryFrom<SelectorSet> for CssSelectorSet {
    type Error = BadSelector;

    fn try_from(value: SelectorSet) -> Result<Self, Self::Error> {
        for s in &value.s {
            if s.has_backref() {
                let sel = s.clone().into_string_vec().join(" ");
                return Err(BadSelector::Backref(input_span(sel)));
            }
        }
        Ok(Self { s: value })
    }
}

impl TryFrom<Value> for CssSelectorSet {
    type Error = BadSelector;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        SelectorSet::try_from(value)?.try_into()
    }
}

impl From<CssSelectorSet> for Value {
    fn from(value: CssSelectorSet) -> Self {
        value.s.into()
    }
}
