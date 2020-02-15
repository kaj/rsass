use super::{SourceName, SourcePos, Span};
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
    pub fn check<T>(res: IResult<Span, T>, data: &[u8]) -> Result<T, Self> {
        match res {
            Ok((rest, items)) if rest.fragment().is_empty() => Ok(items),
            Ok((rest, _styles)) => {
                Err(ParseError::remaining(rest.fragment(), data))
            }
            Err(Err::Error((rest, err))) => {
                Err(ParseError::err(err, rest.fragment(), data))
            }
            Err(Err::Incomplete(_needed)) => {
                Err(ParseError::incomplete(data))
            }
            Err(Err::Failure((rest, err))) => {
                Err(ParseError::err(err, rest.fragment(), data))
            }
        }
    }

    fn remaining(span: &[u8], data: &[u8]) -> ParseError {
        ParseError {
            msg: "Expected end of file.".into(),
            pos: SourcePos::pos_of(span, data),
        }
    }
    fn incomplete(data: &[u8]) -> ParseError {
        ParseError {
            msg: "Unexpected end of file.".into(),
            pos: SourcePos::pos_of(&data[data.len()..], data),
        }
    }
    fn err(kind: ErrorKind, span: &[u8], data: &[u8]) -> ParseError {
        ParseError {
            msg: format!("Parse error: {:?}", kind),
            pos: SourcePos::pos_of(span, data),
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
             \n{0:lnw$} '\
             \n{0:lnw$} {file} {ln}:{lpos}  {cause}",
            "",
            line = self.pos.line,
            msg = self.msg,
            ln = line_no,
            lnw = line_no.len(),
            lpos = self.pos.line_pos,
            file = self.pos.file.name(),
            cause = "root stylesheet", // TODO: Handle imports
        )?;
        Ok(())
    }
}
