use crate::css::{is_not, Value};
use crate::input::SourcePos;
use crate::ParseError;
use std::fmt;

pub(super) enum BadSelector0 {
    Value,
    Parse(ParseError),
}
impl BadSelector0 {
    pub(super) fn ctx(self, v: Value) -> BadSelector {
        match self {
            Self::Value => BadSelector::Value(v),
            Self::Parse(err) => BadSelector::Parse(err),
        }
    }
}
impl From<ParseError> for BadSelector0 {
    fn from(e: ParseError) -> Self {
        Self::Parse(e)
    }
}

/// The error when a [Value] cannot be converted to a [Selectors] or [Selector].
#[derive(Debug)]
pub enum BadSelector {
    /// The value was not the expected type of list or string.
    Value(Value),
    /// There was an error parsing a string value.
    Parse(ParseError),
    /// A backref (`&`) were present but not allowed there.
    Backref(SourcePos),
}

impl fmt::Display for BadSelector {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Value(v) => out.write_str(&is_not(
                v,
                "a valid selector: it must be a string,\
                 \na list of strings, or a list of lists of strings",
            )),
            Self::Parse(e) => e.fmt(out),
            Self::Backref(pos) => {
                writeln!(out, "Parent selectors aren\'t allowed here.")?;
                pos.show(out)
            }
        }
    }
}
impl From<ParseError> for BadSelector {
    fn from(e: ParseError) -> Self {
        Self::Parse(e)
    }
}
