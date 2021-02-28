use super::Span;
use std::fmt::{self, Write};
use std::str::from_utf8;

/// A position in sass input.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct SourcePos {
    /// The text of the source line containing this pos.
    pub line: String,
    /// The line number of this pos.
    pub line_no: u32,
    /// The position on the line.
    pub line_pos: usize,
    /// The source file name and from where it was loaded.
    pub file: SourceName,
}

impl SourcePos {
    /// Show this source position.
    ///
    /// Dislays the line containg the position, highlighting
    /// the position.
    pub fn show(&self, out: &mut impl Write, what: &str) -> fmt::Result {
        let line_no = self.line_no.to_string();
        write!(
            out,
            "{0:lnw$} ,\
             \n{ln} | {line}\
             \n{0:lnw$} |{0:>lpos$}^ {what}\
             \n{0:lnw$} '",
            "",
            line = self.line,
            ln = line_no,
            lnw = line_no.len(),
            lpos = self.line_pos,
            what = what,
        )?;
        let mut nextpos = Some(self);
        while let Some(pos) = nextpos {
            write!(
                out,
                "\n{0:lnw$} {file} {row}:{col}  {cause}",
                "",
                lnw = line_no.len(),
                file = pos.file.name(),
                row = pos.line_no,
                col = pos.line_pos,
                cause = if pos.file.imported_from().is_some() {
                    "import"
                } else {
                    "root stylesheet"
                },
            )?;
            nextpos = pos.file.imported_from();
        }
        Ok(())
    }
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
