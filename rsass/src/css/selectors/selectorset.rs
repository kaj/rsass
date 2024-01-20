use super::{logical::Selector, BadSelector, BadSelector0, CssSelectorSet};
use crate::parser::input_span;
use crate::value::ListSeparator;
use crate::ParseError;
use crate::{css::Value, Invalid};

/// A set of selectors.
/// This is the normal top-level selector, which can be a single
/// [`Selector`] or a comma-separated list (set) of such selectors.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct SelectorSet {
    pub(super) s: Vec<Selector>,
}

impl SelectorSet {
    pub(super) fn is_superselector(&self, other: &Self) -> bool {
        other
            .s
            .iter()
            .all(|sub| self.s.iter().any(|sup| sup.is_superselector(sub)))
    }

    pub(crate) fn replace(
        self,
        original: &SelectorSet,
        replacement: &SelectorSet,
    ) -> Result<Self, Invalid> {
        for original in &original.s {
            if original.is_complex() {
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
    pub(super) fn write_to_buf(&self, buf: &mut String) {
        fn write_one(s: &Selector, buf: &mut String) {
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
        SelectorSet {
            s: self
                .s
                .into_iter()
                .flat_map(|o| o.resolve_ref(ctx))
                .collect::<Vec<_>>(),
        }
    }
}

impl From<SelectorSet> for Value {
    fn from(value: SelectorSet) -> Self {
        let v = value.s.into_iter().map(Value::from).collect::<Vec<_>>();
        if v.is_empty() {
            Value::Null
        } else {
            Value::List(v, Some(ListSeparator::Comma), false)
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
