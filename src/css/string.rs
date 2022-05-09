use crate::value::Quotes;
use std::convert::TryFrom;
use std::fmt::{self, Write};

/// A string in css.  May be quoted.
#[derive(Clone, Debug, Eq, PartialOrd)]
pub struct CssString {
    value: String,
    quotes: Quotes,
}

impl CssString {
    /// Create a new CssString.
    pub fn new(value: String, quotes: Quotes) -> Self {
        CssString { value, quotes }
    }
    /// Unquote this string.
    pub fn unquote(self) -> String {
        if self.quotes.is_none() {
            self.value
        } else {
            let mut result = String::new();
            let mut iter = self.value.chars().peekable();
            while let Some(c) = iter.next() {
                if c == '\\' {
                    let mut val: u32 = 0;
                    let mut got_num = false;
                    let nextchar = loop {
                        match iter.peek() {
                            Some(' ') if got_num => {
                                iter.next();
                                break (None);
                            }
                            Some(&c) => {
                                if let Some(digit) = c.to_digit(16) {
                                    val = val * 10 + digit;
                                    got_num = true;
                                    iter.next();
                                } else if !got_num {
                                    break (iter.next());
                                } else {
                                    break (None);
                                }
                            }
                            _ => break (None),
                        }
                    };
                    if got_num {
                        // TODO: char::REPLACEMENT_CHARACTER from rust 1.52.0
                        result
                            .push(char::try_from(val).unwrap_or('\u{fffd}'));
                    }
                    match nextchar {
                        Some('\n') => {
                            result.push('\\');
                            result.push('a');
                        }
                        Some(c) => {
                            result.push(c);
                        }
                        None => (),
                    }
                } else {
                    result.push(c)
                }
            }
            result
        }
    }
    /// If the value is name-like, make it unquoted.
    pub fn opt_unquote(self) -> Self {
        let mut chars = self.value.chars();
        let t = chars.next()
            .map(|c| c.is_alphabetic()) // first letter
            .unwrap_or(false) // not empty
            && chars.all(|c| c.is_alphanumeric() || c == '-');
        CssString {
            value: self.value,
            quotes: if t { Quotes::None } else { self.quotes },
        }
    }
    /// Quote this string
    pub fn quote(self) -> Self {
        let value = if self.quotes.is_none() {
            self.value.replace('\\', "\\\\")
        } else {
            self.value
        };
        if value.contains('"') && !value.contains('\'') {
            CssString {
                value,
                quotes: Quotes::Single,
            }
        } else {
            CssString {
                value,
                quotes: Quotes::Double,
            }
        }
    }
    /// Adapt the kind of quotes as prefered for a css value.
    pub fn pref_dquotes(self) -> Self {
        let value = self.value;
        let quotes = match self.quotes {
            Quotes::Double
                if value.contains('"') && !value.contains('\'') =>
            {
                Quotes::Single
            }
            Quotes::Single
                if !value.contains('"') || value.contains('\'') =>
            {
                Quotes::Double
            }
            q => q,
        };
        CssString { value, quotes }
    }
    /// Return true if this is an empty unquoted string.
    pub fn is_null(&self) -> bool {
        self.value.is_empty() && self.quotes.is_none()
    }
    /// Return true if this is a css special function call.
    pub(crate) fn is_css_fn(&self) -> bool {
        let value = self.value();
        self.quotes() == Quotes::None
            && value.ends_with(')')
            && (value.starts_with("calc(") || value.starts_with("var("))
    }
    /// Return true if this is a css special function call.
    pub(crate) fn is_css_calc(&self) -> bool {
        let value = self.value();
        self.quotes() == Quotes::None
            && value.ends_with(')')
            && (value.starts_with("calc(") || value.starts_with("clamp("))
    }
    /// Return true if this is a css url function call.
    pub(crate) fn is_css_url(&self) -> bool {
        let value = self.value();
        self.quotes() == Quotes::None
            && value.ends_with(')')
            && value.starts_with("url(")
    }
    /// Access the string value
    pub fn value(&self) -> &str {
        &self.value
    }
    /// Access the quotes
    pub fn quotes(&self) -> Quotes {
        self.quotes
    }
}

impl<S: Into<String>> From<S> for CssString {
    fn from(value: S) -> CssString {
        CssString {
            value: value.into(),
            quotes: Quotes::None,
        }
    }
}

impl fmt::Display for CssString {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        let q = match self.quotes {
            Quotes::None => None,
            Quotes::Double => Some('"'),
            Quotes::Single => Some('\''),
        };
        if let Some(q) = q {
            out.write_char(q)?;
        }
        for c in self.value.chars() {
            if Some(c) == q {
                out.write_char('\\')?;
                out.write_char(c)?;
            } else if is_private_use(c) {
                write!(out, "\\{:x}", c as u32)?;
            } else {
                out.write_char(c)?;
            }
        }
        if let Some(q) = q {
            out.write_char(q)?;
        };
        Ok(())
    }
}

impl PartialEq for CssString {
    fn eq(&self, other: &Self) -> bool {
        if self.quotes == other.quotes {
            self.value == other.value
        } else {
            self.clone().unquote() == other.clone().unquote()
        }
    }
}

impl From<CssString> for crate::sass::Name {
    fn from(s: CssString) -> Self {
        s.value.into()
    }
}

fn is_private_use(c: char) -> bool {
    // https://en.wikipedia.org/wiki/Private_Use_Areas
    matches!(c as u32, 0xE000..=0xF8FF | 0xF0000..=0xFFFFD | 0x100000..=0x10FFFD)
}
