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
    pub msg: String,
    pub pos: SourcePos,
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
        let line_no = self.pos.line_no.to_string();
        write!(
            out,
            "{msg}\
             \n{0:lnw$} ,\
             \n{ln} | {line}\
             \n{0:lnw$} |{0:>lpos$}^\
             \n{0:lnw$} '",
            "",
            line = self.pos.line,
            msg = self.msg,
            ln = line_no,
            lnw = line_no.len(),
            lpos = self.pos.line_pos,
        )?;
        let mut nextpos = Some(&self.pos);
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
