use super::SourcePos;
use std::fmt;

/// The name of a source file.
///
/// This also contains the information if this was the root stylesheet
/// or where it was imported from.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct SourceName {
    name: String,
    pub(crate) imported: SourceKind,
}

impl SourceName {
    /// Create a new top-level SourceName.
    pub fn root<T: ToString>(name: T) -> Self {
        SourceName {
            name: name.to_string(),
            imported: SourceKind::Root,
        }
    }

    /// Create a name for a mixin called from a specific pos.
    pub fn called<T: ToString>(name: T, from: SourcePos) -> Self {
        SourceName {
            name: from.file_url().into(),
            imported: SourceKind::Call(name.to_string(), from),
        }
    }

    /// Get the name of this source.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// True if this is the position of something built-in.
    pub fn is_builtin(&self) -> bool {
        // Note: maybe implement this as a sepate source kind?
        self.name.starts_with("sass:") || self.name.is_empty()
    }
}

/// The kind of loading, or why a load is attempted.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum SourceKind {
    /// The root file that `rsass` loads.
    Root,
    /// An `@import` rule at the given position.
    Import(SourcePos),
    /// An `@use` rule at the given position.
    Use(SourcePos),
    /// An `@forward` rule at the given position.
    Forward(SourcePos),
    /// A call to a named function at the given position.
    Call(String, SourcePos),
}

impl SourceKind {
    /// A `load-css` call is attempted at the given pos.
    pub fn load_css(pos: &SourcePos) -> Self {
        SourceKind::Call("load-css".into(), pos.clone())
    }
    pub(crate) fn url(self, url: &str) -> SourceName {
        SourceName {
            name: url.to_string(),
            imported: self,
        }
    }
    pub(crate) fn next(&self) -> Option<&SourcePos> {
        match self {
            SourceKind::Root => None,
            SourceKind::Import(pos) => Some(pos),
            SourceKind::Use(pos) => Some(pos),
            SourceKind::Forward(pos) => Some(pos),
            SourceKind::Call(_, pos) => Some(pos),
        }
    }

    pub(crate) fn is_import(&self) -> bool {
        matches!(self, SourceKind::Import(_))
    }
}

impl fmt::Display for SourceKind {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SourceKind::Root => out.write_str("root stylesheet"),
            SourceKind::Import(_) => out.write_str("@import"),
            SourceKind::Use(_) => out.write_str("@use"),
            SourceKind::Forward(_) => out.write_str("@forward"),
            SourceKind::Call(name, _) => write!(out, "{}()", name),
        }
    }
}
