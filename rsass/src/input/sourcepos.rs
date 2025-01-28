use crate::input::{SourceFile, SourceName};
use crate::parser::{DebugBytes, Span};
use crate::sass::{FormalArgs, Name};
use std::cmp::Ordering;
use std::fmt::{self, Write};
use std::ops::Range;

/// A specific part of input.
///
/// This is represented as start and end offsets into [`SourceFile`].
/// The sourcefile is internally reference counted, so a `SourcePos` can
/// be cheaply cloned, but does not implement copy.
#[derive(Clone)]
pub struct SourcePos {
    start: usize,
    end: usize,
    source: SourceFile,
}

impl SourcePos {
    pub(crate) fn new_range(source: SourceFile, range: Range<usize>) -> Self {
        Self {
            start: range.start,
            end: range.end,
            source,
        }
    }
    pub(crate) fn borrow(&self) -> Span {
        Span::new_range(&self.source, self.range())
    }
    fn range(&self) -> Range<usize> {
        self.start..self.end
    }
    /// This span of input as actual bytes.
    pub fn fragment(&self) -> &[u8] {
        &self.source.data()[self.range()]
    }
    /// Return true if the pos is at the end of the source.
    pub fn is_at_end(&self) -> bool {
        self.start == self.source.data().len()
    }

    /// Return the line number for (the beginning of) this span.
    pub fn line_no(&self) -> usize {
        self.source.data()[0..self.start]
            .iter()
            .filter(|c| c == &&b'\n')
            .count()
            + 1
    }
    fn line_pos(&self) -> usize {
        let data = self.source.data();
        let start = (0..self.start)
            .rev()
            .find(|i| data.get(*i) == Some(&b'\n'))
            .map_or(0, |n| n + 1);
        self.start - start + 1
    }

    /// Show this source position.
    ///
    /// Dislays the line containg the position, highlighting
    /// the position.
    /// This is typically used when there is one source position
    /// relevant for an error.
    pub fn show(&self, out: &mut impl Write) -> fmt::Result {
        self.show_impl(out, "", '^', "")?;
        self.show_files(out)
    }

    /// Show two source positions together.
    pub fn show_two(
        out: &mut fmt::Formatter,
        one: &Self,
        one_name: &str,
        other: &Self,
        other_name: &str,
    ) -> fmt::Result {
        if one.file_url() == other.file_url() {
            if one < other {
                Self::show_in_file(out, one, one_name, other, other_name)
            } else {
                Self::show_in_file(out, other, other_name, one, one_name)
            }?;
        } else {
            one.show_detail(out, '^', one_name)?;
            writeln!(out)?;
            other.show_detail(out, '=', other_name)?;
        }
        one.show_files(out)
    }

    fn show_in_file(
        out: &mut fmt::Formatter,
        first: &Self,
        first_name: &str,
        second: &Self,
        second_name: &str,
    ) -> fmt::Result {
        let ellipsis = first.line_no() + 1 < second.line_no();
        let lnw = second.line_no().to_string().len();
        let lnw = if ellipsis { std::cmp::max(3, lnw) } else { lnw };
        writeln!(out, "{0:lnw$} ,", "", lnw = lnw)?;
        first.show_inner(out, lnw, '=', first_name)?;
        if ellipsis {
            writeln!(out, "... |")?;
        }
        second.show_inner(out, lnw, '^', second_name)?;
        write!(out, "{0:lnw$} '", "", lnw = lnw)?;
        Ok(())
    }

    fn show_detail(
        &self,
        out: &mut impl Write,
        marker: char,
        what: &str,
    ) -> fmt::Result {
        let filename = self.file_url();
        let filename = if filename.is_empty() {
            String::new()
        } else {
            format!("--> {filename}")
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
        let lnw = self.line_no().to_string().len();
        writeln!(out, "{0:lnw$} ,{arrow}", "", arrow = arrow, lnw = lnw)?;
        self.show_inner(out, lnw, marker, what)?;
        write!(out, "{0:lnw$} '", "", lnw = lnw)
    }
    fn show_inner(
        &self,
        out: &mut impl Write,
        lnw: usize,
        marker: char,
        what: &str,
    ) -> fmt::Result {
        let data = self.source.data();
        let start = (0..self.start)
            .rev()
            .find(|i| data.get(*i) == Some(&b'\n'))
            .map_or(0, |n| n + 1);
        let end = (self.start..)
            .find(|i| data.get(*i).map_or(true, |b| b == &b'\n'))
            .unwrap();
        let line = &data[start..end];
        let line_no = self.line_no();
        write!(
            out,
            "{ln:<lnw$} | {line}\
             \n{0:lnw$} | {0:>lpos$}{mark}",
            "",
            line = String::from_utf8_lossy(line).trim_end_matches('\r'),
            ln = line_no,
            lnw = lnw,
            lpos = self.start - start, // TODO: Count visible char width
            mark = marker.to_string().repeat((self.end - self.start).max(1)), // DITO
        )?;
        if what.is_empty() {
            writeln!(out)
        } else {
            writeln!(out, " {what}")
        }
    }

    /// Show the file name of this pos and where it was imported from.
    fn show_files(&self, out: &mut impl Write) -> fmt::Result {
        let mut nextpos = Some(self.clone());
        let mut lines = Vec::new();
        while let Some(pos) = nextpos {
            lines.push((
                format!(
                    "{file} {row}:{col}",
                    file = pos.file_url(),
                    row = pos.line_no(),
                    col = pos.line_pos(),
                ),
                pos.source.source().imported.to_string(),
            ));
            nextpos = pos.source.source().imported.next().cloned();
        }
        if let Some(whatw) = lines.iter().map(|(what, _why)| what.len()).max()
        {
            for (what, why) in lines {
                write!(
                    out,
                    "\n{0:lnw$} {what:whatw$}  {why}",
                    "",
                    lnw = self.line_no().to_string().len(),
                    what = what,
                    whatw = whatw,
                    why = why,
                )?;
            }
        }
        Ok(())
    }

    /// If self is preceded (on same line) by `s`, include `s` in self.
    pub(crate) fn opt_back(mut self, s: &str) -> Self {
        let len = s.len();
        if self.source.data().get(self.start - len..self.start)
            == Some(s.as_bytes())
        {
            self.start -= len;
        }
        self
    }

    /// If the position is `calc(some-arg)`, change to only `some-arg`.
    ///
    /// This is only used to make errors from rsass more similar to
    /// dart-sass errors.
    pub(crate) fn opt_in_calc(mut self) -> Self {
        let s = b"calc(";
        let part = &self.fragment();
        if part.starts_with(s) && part.ends_with(b")") {
            self.start += s.len();
            self.end -= 1;
        }
        self
    }

    /// Only to make error messages match dart-sass in peculiar cases.
    pub(crate) fn opt_trail_ws(mut self) -> Self {
        while self.source.data().get(self.end) == Some(&b' ') {
            self.end += 1;
        }
        self
    }

    pub(crate) fn in_call(&self, name: &str) -> Self {
        Self {
            start: self.start,
            end: self.end,
            source: SourceFile::scss_bytes(
                self.source.data(),
                SourceName::called(name, self.clone()),
            ),
        }
    }

    /// True if the source of this position is built-in.
    pub fn is_builtin(&self) -> bool {
        self.source.source().is_builtin()
    }

    pub(crate) fn mock_function(
        name: &Name,
        args: &FormalArgs,
        module: &str,
    ) -> Self {
        Self::mock_impl("@function", name, args, module)
    }
    pub(crate) fn mock_mixin(
        name: &Name,
        args: &FormalArgs,
        module: &str,
    ) -> Self {
        Self::mock_impl("@mixin", name, args, module)
    }
    fn mock_impl(
        kind: &str,
        name: &Name,
        args: &FormalArgs,
        module: &str,
    ) -> Self {
        let line = format!("{kind} {name}{args} {{");
        Self {
            start: kind.len() + 1,
            end: line.len() - 2,
            source: SourceFile::scss_bytes(line, SourceName::root(module)),
        }
    }

    /// Get the resolved name / url of the file this source is loaded from.
    pub fn file_url(&self) -> &str {
        self.source.source().name()
    }
}

impl From<SourceFile> for SourcePos {
    fn from(source: SourceFile) -> Self {
        Self {
            start: 0,
            end: source.data().len(),
            source,
        }
    }
}

impl PartialOrd for SourcePos {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for SourcePos {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.source
            .cmp(&other.source)
            .then(self.start.cmp(&other.start))
            .then(self.end.cmp(&other.end))
    }
}
impl PartialEq for SourcePos {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}
impl Eq for SourcePos {}

impl std::fmt::Debug for SourcePos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OwnedSpan")
            .field("range", &self.range())
            .field("data", &DebugBytes(self.fragment()))
            .finish()
    }
}
