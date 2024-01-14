use super::{logical::SelectorSet, CssSelectorSet};
use crate::css::{CssString, Selectors};

/// A pseudo-class or a css2 pseudo-element (:foo)
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Pseudo {
    /// The name of the pseudo-class
    name: String,
    /// Arguments to the pseudo-class
    arg: Option<SelectorSet>,
    /// True if this is a `::psedu-element`, false for a `:pseudo-class`.
    element: bool,
}

impl Pseudo {
    /// Named constructor for pseudo classes.
    pub(crate) fn class(name: &CssString, arg: &Option<Selectors>) -> Self {
        Pseudo {
            name: name.value().into(),
            arg: arg.as_ref().and_then(|s| SelectorSet::try_from(s).ok()),
            element: false,
        }
    }

    /// Named constructor for psedo elements.
    pub(crate) fn element(name: &CssString, arg: &Option<Selectors>) -> Self {
        Pseudo {
            name: name.value().into(),
            arg: arg.as_ref().and_then(|s| SelectorSet::try_from(s).ok()),
            element: true,
        }
    }

    pub(crate) fn is_superselector(&self, b: &Self) -> bool {
        if self.is_element() != b.is_element() || self.name != b.name {
            return false;
        }
        // Note: A better implementetation of is/matches/any would be
        // different from has, host, and host-context.
        if self.name_in(&[
            "is",
            "matches",
            "any",
            "where",
            "has",
            "host",
            "host-context",
        ]) {
            if self.arg == b.arg {
                true
            } else if let (Some(a), Some(b)) = (&self.arg, &b.arg) {
                a.is_superselector(b)
            } else {
                false
            }
        } else if self.name_in(&["not"]) {
            if let (Some(a), Some(b)) = (&self.arg, &b.arg) {
                b.is_superselector(a) // NOTE: Reversed!
            } else {
                false
            }
        } else {
            self.name == b.name && self.arg == b.arg
        }
    }

    pub(super) fn is_element(&self) -> bool {
        self.element || is_pseudo_element(&self.name)
    }
    pub(super) fn is_hover(&self) -> bool {
        self.name == "hover"
    }
    pub(super) fn is_host(&self) -> bool {
        self.name_in(&["host", "host-context"])
    }
    pub(super) fn is_rootish(&self) -> bool {
        self.name_in(&["host", "host-context", "root", "scope"])
    }

    pub(super) fn has_backref(&self) -> bool {
        self.arg.as_ref().map_or(false, SelectorSet::has_backref)
    }
    pub(super) fn resolve_ref(mut self, ctx: &CssSelectorSet) -> Self {
        self.arg = self.arg.map(|arg| arg.resolve_ref(ctx));
        self
    }
    pub(super) fn replace(
        mut self,
        original: &SelectorSet,
        replacement: &SelectorSet,
    ) -> Self {
        if self.name_in(&[
            "is",
            "matches",
            "not",
            "any",
            "where",
            "has",
            "host",
            "host-context",
        ]) {
            self.arg =
                self.arg.map(|s| s.replace(original, replacement).unwrap());
        }
        self
    }

    pub(super) fn write_to_buf(&self, buf: &mut String) {
        buf.push(':');
        if self.element {
            buf.push(':');
        }
        buf.push_str(&self.name);
        if let Some(arg) = &self.arg {
            buf.push('(');
            if self.name_in(&[
                "nth-child",
                "nth-last-child",
                "nth-last-of-type",
                "nth-of-type",
            ]) {
                let mut t = String::new();
                arg.write_to_buf(&mut t);
                buf.push_str(&t.replacen(" + ", "+", 1));
            } else {
                arg.write_to_buf(buf);
            }
            buf.push(')');
        }
    }
    fn name_in(&self, names: &[&str]) -> bool {
        name_in(&self.name, names)
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
