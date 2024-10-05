use crate::input::{SourceFile, SourcePos};
use nom::{
    Compare, CompareResult, InputIter, InputLength, InputTake,
    InputTakeAtPosition, Needed, Offset, Slice,
};
use std::fmt::Write;
use std::ops::{Deref, Range, RangeFrom, RangeTo};

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

impl<'a> InputLength for Span<'a> {
    fn input_len(&self) -> usize {
        self.range().len()
    }
}

impl<'a> Deref for Span<'a> {
    type Target = [u8];
    fn deref(&self) -> &'a [u8] {
        self.fragment()
    }
}

impl<'a> AsRef<[u8]> for Span<'a> {
    fn as_ref(&self) -> &[u8] {
        self.fragment()
    }
}

impl<'a> InputTake for Span<'a> {
    fn take(&self, count: usize) -> Self {
        let end = self.start + count;
        assert!(end <= self.end, "Tried to take {count} from {self:?}");
        Span {
            start: self.start,
            end,
            source: self.source,
        }
    }
    fn take_split(&self, count: usize) -> (Self, Self) {
        let mid = self.start + count;
        assert!(mid <= self.end, "Tried to take_split {count} from {self:?}");
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
}

impl<'a> Slice<Range<usize>> for Span<'a> {
    fn slice(&self, range: Range<usize>) -> Self {
        let start = self.start + range.start;
        let end = self.start + range.end;
        assert!(start <= self.end);
        assert!(end <= self.end);
        Span {
            start,
            end,
            source: self.source,
        }
    }
}
impl<'a> Slice<RangeFrom<usize>> for Span<'a> {
    fn slice(&self, range: RangeFrom<usize>) -> Self {
        let start = self.start + range.start;
        assert!(start <= self.end);
        Span {
            start,
            end: self.end,
            source: self.source,
        }
    }
}
impl<'a> Slice<RangeTo<usize>> for Span<'a> {
    fn slice(&self, range: RangeTo<usize>) -> Self {
        let end = self.start + range.end;
        assert!(end <= self.end);
        Span {
            start: self.start,
            end,
            source: self.source,
        }
    }
}

impl<'a> InputIter for Span<'a> {
    type Item = <&'a [u8] as InputIter>::Item;
    type Iter = <&'a [u8] as InputIter>::Iter;
    type IterElem = <&'a [u8] as InputIter>::IterElem;
    fn iter_indices(&self) -> Self::Iter {
        self.fragment().iter_indices()
    }
    fn iter_elements(&self) -> Self::IterElem {
        self.fragment().iter_elements()
    }
    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.fragment().position(predicate)
    }
    fn slice_index(&self, count: usize) -> Result<usize, Needed> {
        self.fragment().slice_index(count)
    }
}

impl<'a> Offset for Span<'a> {
    fn offset(&self, second: &Self) -> usize {
        assert!(std::ptr::eq(self.source, second.source));
        second.start - self.start
    }
}

/// Capture the position of the current fragment
pub fn position(s: Span) -> super::PResult<Span> {
    Ok((s, s))
}

impl<'a> InputTakeAtPosition for Span<'a> {
    type Item = u8;

    fn split_at_position<P, E: nom::error::ParseError<Self>>(
        &self,
        predicate: P,
    ) -> nom::IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.position(predicate) {
            Some(n) => Ok(self.take_split(n)),
            None => Err(nom::Err::Incomplete(Needed::new(1))),
        }
    }

    fn split_at_position1<P, E: nom::error::ParseError<Self>>(
        &self,
        predicate: P,
        e: nom::error::ErrorKind,
    ) -> nom::IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.position(predicate) {
            Some(0) => Err(nom::Err::Error(E::from_error_kind(*self, e))),
            Some(n) => Ok(self.take_split(n)),
            None => Err(nom::Err::Incomplete(Needed::new(1))),
        }
    }

    fn split_at_position_complete<P, E: nom::error::ParseError<Self>>(
        &self,
        predicate: P,
    ) -> nom::IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.split_at_position(predicate) {
            Err(nom::Err::Incomplete(_)) => {
                Ok(self.take_split(self.input_len()))
            }
            res => res,
        }
    }

    fn split_at_position1_complete<P, E: nom::error::ParseError<Self>>(
        &self,
        predicate: P,
        e: nom::error::ErrorKind,
    ) -> nom::IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.split_at_position1(predicate, e) {
            Err(nom::Err::Incomplete(_)) => {
                if self.input_len() == 0 {
                    Err(nom::Err::Error(E::from_error_kind(*self, e)))
                } else {
                    Ok(self.take_split(self.input_len()))
                }
            }
            res => res,
        }
    }
}

impl<'a> std::fmt::Debug for Span<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Span")
            .field("range", &self.range())
            .field("data", &DebugBytes(self.fragment()))
            .finish()
    }
}

pub(crate) struct DebugBytes<'a>(pub(crate) &'a [u8]);

impl<'a> std::fmt::Debug for DebugBytes<'a> {
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
