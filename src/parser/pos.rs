use super::Span;
use std::str::from_utf8;

/// A position in sass input.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct SourcePos {
    pub line: String,
    pub line_no: u32,
    pub line_pos: usize,
    pub file: SourceName,
}

impl From<Span<'_>> for SourcePos {
    fn from(span: Span) -> Self {
        SourcePos {
            line: from_utf8(span.get_line_beginning())
                .unwrap_or("<<failed to display line>>")
                .to_string(),
            line_no: span.location_line(),
            line_pos: span.get_utf8_column(),
            file: SourceName::root("-"), // span.extra.clone(),
        }
    }
}

/// The name of a scss source file.
///
/// Currently this is just a String.
/// In a future version it should also contain the information if this
/// was the root stylesheet or where it was imported from.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct SourceName {
    name: String,
}

impl SourceName {
    pub fn root<T: ToString>(name: T) -> Self {
        SourceName {
            name: name.to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
