use super::Span;
use crate::sass::Name;
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
    /// The length of the interesting part.
    length: usize,
    /// The source file name and from where it was loaded.
    pub file: SourceName,
}

impl SourcePos {
    /// Create a new SourcePos from a start and an end Span.
    pub fn from_to(start: Span, end: Span) -> Self {
        let mut result = Self::from(start);
        result.length =
            std::cmp::max(1, end.location_offset() - start.location_offset());
        result
    }
    /// ...
    pub fn mock_function(name: Name, args: &[Name], module: &Name) -> Self {
        let args = args
            .iter()
            .map(|a| format!("${}", a))
            .collect::<Vec<_>>()
            .join(", ");
        SourcePos {
            line: format!("@function {}({}) {{", name, args),
            line_no: 1,
            line_pos: 11,
            length: name.as_ref().chars().count() + args.chars().count() + 2,
            file: SourceName::root(module.as_ref()),
        }
    }
    /// ...
    pub fn length(mut self, l: usize) -> Self {
        self.length = l;
        self
    }
    /// Show this source position.
    ///
    /// Dislays the line containg the position, highlighting
    /// the position.
    pub fn show(&self, out: &mut impl Write) -> fmt::Result {
        self.show_impl(out, "", '^', "")?;
        self.show_files(out)
    }
    /// Show this source position.
    ///
    /// Dislays the line containg the position, highlighting
    /// the position.
    pub fn show_detail(
        &self,
        out: &mut impl Write,
        marker: char,
        what: &str,
    ) -> fmt::Result {
        self.show_impl(out, &format!("--> {}", self.file.name), marker, what)
    }
    fn show_impl(
        &self,
        out: &mut impl Write,
        arrow: &str,
        marker: char,
        what: &str,
    ) -> fmt::Result {
        let line_no = self.line_no.to_string();
        write!(
            out,
            "{0:lnw$} ,{arrow}\
             \n{ln} | {line}\
             \n{0:lnw$} |{0:>lpos$}{mark}{what}\
             \n{0:lnw$} '",
            "",
            arrow = arrow,
            line = self.line,
            ln = line_no,
            lnw = line_no.len(),
            lpos = self.line_pos,
            mark = marker.to_string().repeat(self.length),
            what = what,
        )
    }
    /// ...
    pub fn show_files(&self, out: &mut impl Write) -> fmt::Result {
        let mut nextpos = Some(self);
        while let Some(pos) = nextpos {
            write!(
                out,
                "\n{0:lnw$} {file} {row}:{col}  {cause}",
                "",
                lnw = self.line_no.to_string().len(),
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
            length: 1,
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
    /// Create a new top-level SourceName.
    pub fn root<T: ToString>(name: T) -> Self {
        SourceName {
            name: name.to_string(),
            imported: None,
        }
    }
    /// Create a name for a file imported from a specific pos.
    pub fn imported<T: ToString>(name: T, from: SourcePos) -> Self {
        SourceName {
            name: name.to_string(),
            imported: Some(Box::new(from)),
        }
    }

    /// Get the name of this source.
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Get where this source is imported from, if not top-level.
    pub fn imported_from(&self) -> Option<&SourcePos> {
        self.imported.as_ref().map(|b| b.as_ref())
    }
}
