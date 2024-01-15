use super::Selectors as OldSelectorSet;
use super::{logical::Selector, BadSelector, CssSelectorSet};
use crate::value::ListSeparator;
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

    pub fn replace(
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

impl TryFrom<Value> for SelectorSet {
    type Error = BadSelector;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        // This uses the old selector implementation as intermediary.
        // TODO: Implement direct parsing of the new selectors.
        Self::try_from(&OldSelectorSet::try_from(value)?)
    }
}

impl TryFrom<&OldSelectorSet> for SelectorSet {
    type Error = BadSelector;

    fn try_from(value: &OldSelectorSet) -> Result<Self, Self::Error> {
        value
            .s
            .iter()
            .map(Selector::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map(|s| SelectorSet { s })
    }
}
/// FIXME: Is this still needed?  Remove?
impl TryFrom<SelectorSet> for OldSelectorSet {
    type Error = BadSelector;

    fn try_from(value: SelectorSet) -> Result<Self, Self::Error> {
        Value::from(value).try_into()
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
