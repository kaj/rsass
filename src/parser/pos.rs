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
            file: span.extra.clone(),
        }
    }
}

/// The name of a scss source file.
///
/// This also contains the information if this was the root stylesheet
/// or where it was imported from.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct SourceName {
    name: String,
    imported: Option<Box<SourcePos>>,
}

impl SourceName {
    pub fn root<T: ToString>(name: T) -> Self {
        SourceName {
            name: name.to_string(),
            imported: None,
        }
    }
    pub fn imported<T: ToString>(name: T, from: SourcePos) -> Self {
        SourceName {
            name: name.to_string(),
            imported: Some(Box::new(from)),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn imported_from(&self) -> Option<&SourcePos> {
        self.imported.as_ref().map(|b| b.as_ref())
    }
}
