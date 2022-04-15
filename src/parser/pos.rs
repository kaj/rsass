use super::Span;
use crate::sass::{FormalArgs, Name};
use std::fmt::{self, Write};
use std::str::from_utf8;
use std::sync::Arc;

/// A position in sass input.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct SourcePos {
    p: Arc<SourcePosImpl>,
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
struct SourcePosImpl {
    /// The line number of this pos.
    line_no: u32,
    /// The position on the line.
    line_pos: usize,
    /// The length of the interesting part.
    length: usize,
    /// The text of the source line containing this pos.
    line: String,
    /// The source file name and from where it was loaded.
    file: SourceName,
}

impl SourcePos {
    /// Create a new SourcePos from a start and an end Span.
    pub fn from_to(start: Span, end: Span) -> Self {
        let mut result = SourcePosImpl::from(start);
        result.length =
            std::cmp::max(1, end.location_offset() - start.location_offset());
        SourcePos {
            p: Arc::new(result),
        }
    }

    pub(crate) fn in_call(&self, name: &str) -> Self {
        SourcePos {
            p: Arc::new(SourcePosImpl {
                file: SourceName::called(name, self.clone()),
                ..(*self.p).clone()
            }),
        }
    }

    pub(crate) fn mock_function(
        name: &Name,
        args: &FormalArgs,
        module: &str,
    ) -> Self {
        SourcePos::mock_impl(name, "@function", args, module)
    }
    pub(crate) fn mock_mixin(
        name: &Name,
        args: &FormalArgs,
        module: &str,
    ) -> Self {
        SourcePos::mock_impl(name, "@mixin", args, module)
    }
    fn mock_impl(
        name: &Name,
        kind: &str,
        args: &FormalArgs,
        module: &str,
    ) -> Self {
        let line = format!("{} {}{} {{", kind, name, args);
        let line_pos = kind.chars().count() + 2;
        let length = line.chars().count() - 1 - line_pos;
        SourcePos {
            p: Arc::new(SourcePosImpl {
                line,
                line_no: 1,
                line_pos,
                length,
                file: SourceName::root(module),
            }),
        }
    }

    /// Show this source position.
    ///
    /// Dislays the line containg the position, highlighting
    /// the position.
    /// This is typically used when there is one source position
    /// relevant for an error.
    /// This includes [Self::show_files].
    pub fn show(&self, out: &mut impl Write) -> fmt::Result {
        self.show_impl(out, "", '^', "")?;
        self.show_files(out)
    }
    /// Show this source position.
    ///
    /// Dislays the line containg the position, highlighting
    /// the position with a specific identifier.
    /// This is typically used when there is more than one source
    /// position relevant for an error.
    /// This does not include [Self::show_files].
    pub fn show_detail(
        &self,
        out: &mut impl Write,
        marker: char,
        what: &str,
    ) -> fmt::Result {
        let filename = if self.p.file.name.is_empty() {
            String::new()
        } else {
            format!("--> {}", self.p.file.name)
        };
        self.show_impl(out, &filename, marker, what)
    }
    fn show_impl(
        &self,
        out: &mut impl Write,
        arrow: &str,
        marker: char,
        what: &str,
    ) -> fmt::Result {
        let lnw = self.p.line_no.to_string().len();
        write!(out, "{0:lnw$} ,{arrow}", "", arrow = arrow, lnw = lnw)?;
        self.show_inner(out, lnw, marker, what)?;
        write!(out, "{0:lnw$} '", "", lnw = lnw)
    }
    pub(crate) fn show_inner(
        &self,
        out: &mut impl Write,
        lnw: usize,
        marker: char,
        what: &str,
    ) -> fmt::Result {
        writeln!(
            out,
            "\n{ln:<lnw$} | {line}\
             \n{0:lnw$} |{0:>lpos$}{mark}{what}",
            "",
            line = self.p.line,
            ln = self.p.line_no,
            lnw = lnw,
            lpos = self.p.line_pos,
            mark = marker.to_string().repeat(self.p.length),
            what = what,
        )
    }
    /// Show the file name of this pos and where it was imported from.
    pub fn show_files(&self, out: &mut impl Write) -> fmt::Result {
        let mut nextpos = Some(self);
        let mut lines = Vec::new();
        while let Some(pos) = nextpos {
            lines.push((
                format!(
                    "{file} {row}:{col}",
                    file = pos.p.file.name(),
                    row = pos.p.line_no,
                    col = pos.p.line_pos,
                ),
                pos.p.file.imported.to_string(),
            ));
            nextpos = match &pos.p.file.imported {
                SourceKind::Root => None,
                SourceKind::Imported(pos) => Some(pos),
                SourceKind::Used(pos) => Some(pos),
                SourceKind::Called(_, pos) => Some(pos),
            };
        }
        if let Some(whatw) = lines.iter().map(|(what, _why)| what.len()).max()
        {
            for (what, why) in lines {
                write!(
                    out,
                    "\n{0:lnw$} {what:whatw$}  {why}",
                    "",
                    lnw = self.p.line_no.to_string().len(),
                    what = what,
                    whatw = whatw,
                    why = why,
                )?;
            }
        }
        Ok(())
    }

    /// If self is preceded (on same line) by `s`, include `s` in self.
    pub(crate) fn opt_back(&mut self, s: &str) {
        let p: &mut SourcePosImpl = Arc::make_mut(&mut self.p);
        if p.line[..p.line_pos - 1].ends_with(s) {
            let len = s.chars().count();
            p.line_pos -= len;
            p.length += len;
        }
    }
    /// If the position is `calc(some-arg)`, change to only `some-arg`.
    ///
    /// This is only used to make errors from rsass more similar to
    /// dart-sass errors.
    pub(crate) fn opt_in_calc(mut self) -> Self {
        let p: &mut SourcePosImpl = Arc::make_mut(&mut self.p);
        let s = "calc(";
        let part = &p.line[p.line_pos - 1..];
        if part.starts_with(s) && part.chars().nth(p.length - 1) == Some(')')
        {
            let len = s.chars().count();
            p.line_pos += len;
            p.length -= len;
            p.length -= 1;
        }
        self
    }

    /// True if this is the position of something built-in.
    pub fn is_builtin(&self) -> bool {
        self.p.file.is_builtin()
    }
    pub(crate) fn same_file_as(&self, other: &Self) -> bool {
        self.p.file.name == other.p.file.name
    }
    pub(crate) fn file_url(&self) -> &str {
        &self.p.file.name
    }
}

impl From<Span<'_>> for SourcePos {
    fn from(span: Span) -> Self {
        SourcePos {
            p: Arc::new(SourcePosImpl::from(span)),
        }
    }
}
impl From<Span<'_>> for SourcePosImpl {
    fn from(span: Span) -> Self {
        SourcePosImpl {
            line: from_utf8(span.get_line_beginning())
                .unwrap_or("<<failed to display line>>")
                .trim_end()
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
    imported: SourceKind,
}

impl SourceName {
    /// Create a new top-level SourceName.
    pub fn root<T: ToString>(name: T) -> Self {
        SourceName {
            name: name.to_string(),
            imported: SourceKind::Root,
        }
    }
    /// Create a name for a file imported from a specific pos.
    pub fn imported<T: ToString>(name: T, from: SourcePos) -> Self {
        SourceName {
            name: name.to_string(),
            imported: SourceKind::Imported(from),
        }
    }
    /// Create a name for a file imported from a specific pos.
    pub fn used<T: ToString>(name: T, from: SourcePos) -> Self {
        SourceName {
            name: name.to_string(),
            imported: SourceKind::Used(from),
        }
    }
    /// Create a name for a mixin loaded by load_css from a specific pos.
    pub fn load_css<T: ToString>(name: T, from: SourcePos) -> Self {
        SourceName {
            name: name.to_string(),
            imported: SourceKind::Called("load-css".to_string(), from),
        }
    }
    /// Create a name for a mixin called from a specific pos.
    pub fn called<T: ToString>(name: T, from: SourcePos) -> Self {
        SourceName {
            name: from.p.file.name.clone(),
            imported: SourceKind::Called(name.to_string(), from),
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

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
enum SourceKind {
    Root,
    Imported(SourcePos),
    Used(SourcePos),
    Called(String, SourcePos),
}

impl fmt::Display for SourceKind {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SourceKind::Root => out.write_str("root stylesheet"),
            SourceKind::Imported(_) => out.write_str("@import"),
            SourceKind::Used(_) => out.write_str("@use"),
            SourceKind::Called(name, _) => write!(out, "{}()", name),
        }
    }
}
