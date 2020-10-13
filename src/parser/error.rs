use super::{code_span, SourceName, SourcePos, Span};
use nom::error::ErrorKind;
use nom::{Err, IResult};
use std::fmt;
use std::path::Path;

/// An error encountered when parsing sass.
///
/// This contains an error message (currently just a String, and often
/// not very descriptive) and informaion on where in the parsed data
/// the error occured.
#[derive(Debug)]
pub struct ParseError {
    pub msg: String,
    pub pos: SourcePos,
}

impl std::error::Error for ParseError {}

impl ParseError {
    /// Check a nom result for errors.
    ///
    /// This is not a `From` implementation for two reasons:
    /// 1. It needs a reference to the original data to find the position.
    /// 2. An `Ok` result with remaining unparsed data is also considered an error.
    pub fn check<T>(res: IResult<Span, T>) -> Result<T, Self> {
        match res {
            Ok((rest, items)) if rest.fragment().is_empty() => Ok(items),
            Ok((rest, _styles)) => Err(ParseError::remaining(rest)),
            Err(Err::Error((rest, err))) => Err(ParseError::err(err, rest)),
            Err(Err::Incomplete(_needed)) => Err(ParseError::incomplete()),
            Err(Err::Failure((rest, err))) => Err(ParseError::err(err, rest)),
        }
    }

    fn remaining(span: Span) -> ParseError {
        ParseError {
            msg: "Expected end of file.".into(),
            pos: span.into(),
        }
    }
    fn incomplete() -> ParseError {
        ParseError {
            msg: "Unexpected end of file.".into(),
            pos: code_span(b"").into(), // TODO: What to do?
        }
    }
    fn err(kind: ErrorKind, span: Span) -> ParseError {
        ParseError {
            msg: format!("Parse error: {:?}", kind),
            pos: span.into(),
        }
    }

    /// Add information about in what file an error occurred to self.
    pub fn in_file(mut self, file: &Path) -> Self {
        self.pos.file = SourceName::root(file.to_string_lossy());
        self
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
             \n{0:lnw$} | {0:>lpos$}^\
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
