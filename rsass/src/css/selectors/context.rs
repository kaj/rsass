use super::{CssSelectorSet, SelectorSet};

/// A full set of selectors with a separate backref.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelectorCtx {
    /// The actual selectors.
    s: CssSelectorSet,
    backref: CssSelectorSet,
}

impl SelectorCtx {
    pub fn root() -> Self {
        Self {
            s: CssSelectorSet::root(),
            backref: CssSelectorSet::root(),
        }
    }
    /// Return true if this is a root (empty) selector.
    pub fn is_root(&self) -> bool {
        self.s.is_root()
    }

    pub fn real(&self) -> CssSelectorSet {
        self.s.clone()
    }

    /// Evaluate selectors inside this context.
    pub(crate) fn nest(&self, selectors: SelectorSet) -> CssSelectorSet {
        if selectors.has_backref() {
            CssSelectorSet {
                s: selectors.resolve_ref(self.get_backref()),
            }
        } else {
            self.s.nest(selectors)
        }
    }
    pub(crate) fn at_root(&self, selectors: SelectorSet) -> Self {
        let backref = self.get_backref();
        Self {
            s: CssSelectorSet {
                s: selectors.resolve_ref(backref),
            },
            backref: backref.clone(),
        }
    }
    pub(crate) fn get_backref(&self) -> &CssSelectorSet {
        if self.s.is_root() {
            &self.backref
        } else {
            &self.s
        }
    }
}

impl From<CssSelectorSet> for SelectorCtx {
    fn from(value: CssSelectorSet) -> Self {
        Self {
            s: value,
            backref: CssSelectorSet::root(),
        }
    }
}
