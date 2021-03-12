use super::{SourcePos, Span};
use nom::error::ErrorKind;
use nom::{Finish, IResult};
use std::fmt;

/// An error encountered when parsing sass.
///
/// This contains an error message (currently just a String, and often
/// not very descriptive) and informaion on where in the parsed data
/// the error occured.
#[derive(Debug, PartialEq, Eq)]
pub struct ParseError {
    msg: String,
    pos: SourcePos,
}

impl std::error::Error for ParseError {}

impl ParseError {
    /// Check a nom result for errors.
    ///
    /// This differs from a `From<nom::Err>` implementation in that an
    /// `Ok` result with remaining unparsed data is also considered an
    /// error.
    pub fn check<T>(res: IResult<Span, T>) -> Result<T, Self> {
        let (rest, value) = res.finish()?;
        if rest.fragment().is_empty() {
            Ok(value)
        } else {
            Err(ParseError::remaining(rest))
        }
    }

    fn remaining(span: Span) -> ParseError {
        ParseError {
            msg: "Expected end of file.".into(),
            pos: span.into(),
        }
    }
    fn err(kind: ErrorKind, span: Span) -> ParseError {
        ParseError {
            msg: format!("Parse error: {:?}", kind),
            pos: span.into(),
        }
    }
}

impl From<nom::error::Error<Span<'_>>> for ParseError {
    fn from(err: nom::error::Error<Span>) -> Self {
        ParseError::err(err.code, err.input)
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        writeln!(out, "{}", self.msg)?;
        self.pos.show(out)
    }
}
