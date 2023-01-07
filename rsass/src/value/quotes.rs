use std::fmt::{self, Write};

/// A literal value can be double-quoted, single-quoted or not quoted.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Quotes {
    /// Double quotes
    Double,
    /// Single quotes
    Single,
    /// No quotes
    None,
}

impl Quotes {
    /// Return true is this is no quotes.
    pub fn is_none(&self) -> bool {
        *self == Quotes::None
    }
}

impl fmt::Display for Quotes {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Quotes::Double => out.write_char('"'),
            Quotes::Single => out.write_char('\''),
            Quotes::None => Ok(()),
        }
    }
}
