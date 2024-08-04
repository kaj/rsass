use super::{logical::Selector, BadSelector, BadSelector0, CssSelectorSet};
use super::{OldSelectors, Opt};
use crate::output::CssBuf;
use crate::parser::input_span;
use crate::value::ListSeparator;
use crate::ParseError;
use crate::{css::Value, Invalid};

/// A set of selectors.
/// This is the normal top-level selector, which can be a single
/// [`Selector`] or a comma-separated list (set) of such selectors.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelectorSet {
    pub(super) s: Vec<Selector>,
}

impl SelectorSet {
    pub(crate) fn root() -> Self {
        Self {
            s: vec![Selector::default()],
        }
    }
    /// Return true if this is a root (empty) selector.
    pub fn is_root(&self) -> bool {
        self.s.len() == 1 && self.s[0] == Selector::default()
    }

    pub(crate) fn no_placeholder(&self) -> Opt<Self> {
        let s = self.s.iter().map(Selector::no_placeholder);
        Opt::collect_pos(s).map(|s| Self { s })
    }
    pub(crate) fn no_leading_combinator(&self) -> Opt<Self> {
        let s = self.s.iter().map(Selector::no_leading_combinator);
        Opt::collect_pos(s).map(|s| Self { s })
    }

    pub(crate) fn extend(
        self,
        extendee: &Self,
        extender: &Self,
    ) -> Result<Self, Invalid> {
        extendee.check_extend_complex()?;
        let result = self
            .s
            .into_iter()
            .flat_map(|s| s.extend(extendee, extender))
            .collect();
        Ok(Self { s: result })
    }

    pub(super) fn is_superselector(&self, other: &Self) -> bool {
        other
            .s
            .iter()
            .all(|sub| self.s.iter().any(|sup| sup.is_superselector(sub)))
    }

    pub(crate) fn replace(
        self,
        original: &Self,
        replacement: &Self,
    ) -> Result<Self, Invalid> {
        original.check_extend_complex()?;
        let result = self
            .s
            .into_iter()
            .flat_map(|s| s.replace(original, replacement))
            .collect();
        Ok(Self { s: result })
    }

    fn check_extend_complex(&self) -> Result<(), Invalid> {
        for extendee in &self.s {
            if extendee.is_complex() {
                let s = extendee.clone().into_string_vec().join(" ");
                return Err(Invalid::AtError(format!(
                    "Can\'t extend complex selector {s}."
                )));
            }
        }
        Ok(())
    }

    pub fn write_to(&self, buf: &mut CssBuf) {
        if let Some((first, rest)) = self.s.split_first() {
            first.write_to(buf);
            for one in rest {
                buf.add_one(", ", ",");
                one.write_to(buf);
            }
        }
    }

    // TODO: Get rid of this, use the above!
    pub(super) fn write_to_buf(&self, buf: &mut String) {
        fn write_one(s: &Selector, buf: &mut String) {
            // Note: this should be made much more efficient!
            buf.push_str(&s.clone().into_string_vec().join(" "));
        }
        if let Some((first, rest)) = self.s.split_first() {
            write_one(first, buf);
            for one in rest {
                buf.push_str(", "); // TODO: Only ',' is compressed!
                write_one(one, buf);
            }
        }
    }
    pub(super) fn has_backref(&self) -> bool {
        self.s.iter().any(Selector::has_backref)
    }
    pub(super) fn resolve_ref(self, ctx: &CssSelectorSet) -> Self {
        Self {
            s: self
                .s
                .into_iter()
                .flat_map(|o| o.resolve_ref(ctx))
                .collect::<Vec<_>>(),
        }
    }

    /* // Get these selectors with a specific backref selector.
    ///
    /// Used to create `@at-root` contexts, to have `&` work in them.
    pub(crate) fn with_backref(self, context: Selector) -> SelectorCtx {
        SelectorCtx {
            s: self.try_into().unwrap(),
            backref: CssSelectorSet,
        }.inside(&SelectorCtx {
            s: Self::root().try_into().unwrap(),
            backref: context,
        })
    }*/

    /// Get the first of these selectors (or the root selector if empty).
    pub(crate) fn one(&self) -> Selector {
        self.s.first().cloned().unwrap_or_else(Selector::default)
    }
}

impl TryFrom<&OldSelectors> for SelectorSet {
    type Error = BadSelector;

    fn try_from(s: &OldSelectors) -> Result<Self, Self::Error> {
        if s.is_root() {
            return Ok(SelectorSet {
                s: vec![Selector::default()],
            });
        }
        let formatted = s.to_string();
        let span = input_span(formatted.clone());
        ParseError::check(parser::selector_set(span.borrow()))?
            .try_into()
            .map_err(|_| BadSelector::Value(formatted.into()))
    }
}

impl From<SelectorSet> for Value {
    fn from(value: SelectorSet) -> Self {
        if value.is_root() {
            return Self::Null;
        }
        let v = value.s.into_iter().map(Self::from).collect::<Vec<_>>();
        if v.is_empty() {
            Self::Null
        } else {
            Self::List(v, Some(ListSeparator::Comma), false)
        }
    }
}

impl TryFrom<Value> for SelectorSet {
    type Error = BadSelector;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        value_to_selectors(&value).map_err(move |e| e.ctx(value))
    }
}

fn value_to_selectors(v: &Value) -> Result<SelectorSet, BadSelector0> {
    match v {
        Value::List(vv, Some(ListSeparator::Comma), _) => {
            let s = vv
                .iter()
                .map(Selector::_try_from_value)
                .collect::<Result<_, _>>()?;
            Ok(SelectorSet { s })
        }
        v @ Value::List(..) => Ok(SelectorSet {
            s: vec![Selector::_try_from_value(v)?],
        }),
        Value::Literal(s) => {
            if s.value().is_empty() {
                Ok(SelectorSet { s: vec![] })
            } else {
                let span = input_span(s.value());
                Ok(ParseError::check(parser::selector_set(span.borrow()))?)
            }
        }
        _ => Err(BadSelector0::Value),
    }
}

pub(crate) mod parser {
    use super::SelectorSet;
    use crate::parser::{util::opt_spacelike, PResult, Span};
    use nom::bytes::complete::tag;
    use nom::combinator::map;
    use nom::multi::separated_list1;
    use nom::sequence::delimited;

    pub(crate) fn selector_set(input: Span) -> PResult<SelectorSet> {
        map(
            separated_list1(
                delimited(opt_spacelike, tag(","), opt_spacelike),
                super::super::logical::parser::selector,
            ),
            |s| SelectorSet { s },
        )(input)
    }
}
