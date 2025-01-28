use crate::input::{SourceFile, SourcePos};
use nom::{Compare, CompareResult, Input, Needed, Offset};
use std::fmt::Write;
use std::ops::{Deref, Range};

/// A specific piece of input data.
#[derive(Clone, Copy)]
pub struct Span<'a> {
    start: usize,
    end: usize,
    pub(crate) source: &'a SourceFile,
}
impl<'a> Span<'a> {
    pub(crate) fn new(source: &'a SourceFile) -> Self {
        Span {
            start: 0,
            end: source.data().len(),
            source,
        }
    }
    pub(crate) fn new_range(
        source: &'a SourceFile,
        range: Range<usize>,
    ) -> Self {
        // precondition: 0 <= range.start <= range.end <= source.data().len()
        // only the last part is not guaranteed by the range type, so check it:
        assert!(range.end <= source.data().len());
        Span {
            start: range.start,
            end: range.end,
            source,
        }
    }
    pub(crate) fn is_at_end(&self) -> bool {
        self.start == self.source.data().len()
    }
    fn range(&self) -> Range<usize> {
        self.start..self.end
    }
    /// This span of input as actual bytes.
    pub fn fragment(&self) -> &'a [u8] {
        &self.source.data()[self.range()]
    }
    /// How far into the source does this span start?
    pub fn location_offset(&self) -> usize {
        self.start
    }
    /// Should find a line number
    pub fn location_line(&self) -> u32 {
        self.source.data()[0..self.start]
            .iter()
            .filter(|c| c == &&b'\n')
            .count() as u32
            + 1
    }
    /// Should find a position in line
    pub fn get_utf8_column(&self) -> usize {
        (0..self.start)
            .rev()
            .find(|i| self.source.data().get(*i) == Some(&b'\n'))
            .map_or(self.start + 1, |s| self.start - s)
    }
    pub fn to_owned(self) -> SourcePos {
        SourcePos::new_range(self.source.clone(), self.range())
    }
    pub(crate) fn up_to(self, other: &Self) -> Self {
        self.take(self.offset(other))
    }
    /// If `self` goes on the end of input, return just the starting point.
    /// Otherwise preserve `self` as is.
    pub(crate) fn sanitize_end(self) -> Self {
        if self.end < self.source.data().len() {
            self
        } else {
            self.up_to(&self)
        }
    }
}

impl<'a, T> Compare<T> for Span<'a>
where
    &'a [u8]: Compare<T>,
{
    fn compare(&self, t: T) -> CompareResult {
        self.fragment().compare(t)
    }
    fn compare_no_case(&self, t: T) -> CompareResult {
        self.fragment().compare_no_case(t)
    }
}

impl<'a> Input for Span<'a> {
    type Item = u8;

    type Iter = <&'a [u8] as Input>::Iter;

    type IterIndices = <&'a [u8] as Input>::IterIndices;

    fn input_len(&self) -> usize {
        self.end - self.start
    }

    fn take(&self, index: usize) -> Self {
        let end = self.start + index;
        assert!(end <= self.end, "Tried to take {index} from {self:?}");
        Span {
            start: self.start,
            end,
            source: self.source,
        }
    }

    fn take_from(&self, index: usize) -> Self {
        let mid = self.start + index;
        assert!(mid <= self.end, "Tried to take_from {index} from {self:?}");
        Span {
            start: mid,
            end: self.end,
            source: self.source,
        }
    }

    fn take_split(&self, index: usize) -> (Self, Self) {
        let mid = self.start + index;
        assert!(mid <= self.end, "Tried to take_split {index} from {self:?}");
        (
            Span {
                start: mid,
                end: self.end,
                source: self.source,
            },
            Span {
                start: self.start,
                end: mid,
                source: self.source,
            },
        )
    }

    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.fragment().position(predicate)
    }

    fn iter_elements(&self) -> Self::Iter {
        self.fragment().iter_elements()
    }

    fn iter_indices(&self) -> Self::IterIndices {
        self.fragment().iter_indices()
    }

    fn slice_index(&self, count: usize) -> Result<usize, Needed> {
        self.fragment().slice_index(count)
    }
}

impl<'a> Deref for Span<'a> {
    type Target = [u8];
    fn deref(&self) -> &'a [u8] {
        self.fragment()
    }
}

impl Offset for Span<'_> {
    fn offset(&self, second: &Self) -> usize {
        assert!(std::ptr::eq(self.source, second.source));
        second.start - self.start
    }
}

/// Capture the position of the current fragment
pub fn position(s: Span) -> super::PResult<Span> {
    Ok((s, s))
}

impl std::fmt::Debug for Span<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Span")
            .field("range", &self.range())
            .field("data", &DebugBytes(self.fragment()))
            .finish()
    }
}

pub(crate) struct DebugBytes<'a>(pub(crate) &'a [u8]);

impl std::fmt::Debug for DebugBytes<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('"')?;
        for b in self.0 {
            match b {
                b'\n' => f.write_str("\\n"),
                b'\t' => f.write_str("\\t"),
                b'\'' | b'\"' | b'\\' => {
                    write!(f, "\\{b}")
                }
                b if b.is_ascii_graphic() || *b == b' ' => {
                    f.write_char(*b as char)
                }
                b => write!(f, "\\{b:02x}"),
            }?;
        }
        f.write_char('"')
    }
}
