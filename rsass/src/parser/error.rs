use super::{PResult, Span};
use crate::input::SourcePos;
use nom::{character::complete::one_of, error::VerboseErrorKind, Finish};
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
    pub fn check<T>(res: PResult<T>) -> Result<T, Self> {
        let (rest, value) = res.finish()?;
        if rest.fragment().is_empty() {
            Ok(value)
        } else {
            Err(Self::new("Expected end of file.", rest.to_owned()))
        }
    }

    pub(crate) fn new<Msg>(msg: Msg, pos: SourcePos) -> Self
    where
        Msg: Into<String>,
    {
        let msg = if pos.is_at_end() {
            String::from("expected more input.")
        } else {
            msg.into()
        };
        Self { msg, pos }
    }
}

impl From<nom::error::VerboseError<Span<'_>>> for ParseError {
    fn from(value: nom::error::VerboseError<Span<'_>>) -> Self {
        let (msg, pos) = value
            .errors
            .iter()
            .filter_map(|(pos, kind)| {
                match kind {
                    VerboseErrorKind::Context(ctx) => {
                        Some((ctx.to_string(), pos))
                    }
                    VerboseErrorKind::Char(ch) => {
                        Some((format!("expected {:?}.", ch.to_string()), pos))
                    }
                    VerboseErrorKind::Nom(_) => None, // Try the next one!
                }
            })
            .next()
            .or_else(|| {
                value.errors.first().map(|(pos, kind)| {
                    if pos.is_at_end() {
                        ("expected more input.".to_string(), pos)
                    } else if let PResult::Ok((_, b)) = one_of(")}]")(*pos) {
                        (format!("unmatched \"{b}\"."), pos)
                    } else {
                        (format!("Parse error: {kind:?}"), pos)
                    }
                })
            })
            .unwrap();
        Self::new(msg, pos.up_to(pos).to_owned())
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        writeln!(out, "{}", self.msg)?;
        self.pos.show(out)
    }
}
