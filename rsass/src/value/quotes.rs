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
        *self == Self::None
    }
}

impl fmt::Display for Quotes {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Double => out.write_char('"'),
            Self::Single => out.write_char('\''),
            Self::None => Ok(()),
        }
    }
}
