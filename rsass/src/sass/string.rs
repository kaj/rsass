use crate::css::CssString;
use crate::error::{Error, ResultPos};
use crate::sass::Value;
use crate::value::Quotes;
use crate::ScopeRef;
use std::fmt::{self, Write};
use std::mem::swap;

/// A string that may contain interpolations.
///
/// Used in all places in sass items and values where interpolations
/// may occur.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct SassString {
    parts: Vec<StringPart>,
    quotes: Quotes,
}

/// A part of a string value, either a string or a value to interpolate from.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub enum StringPart {
    /// A raw (literal) string.
    Raw(String),
    /// A value to be evaluated.
    Interpolation(Value),
}

impl SassString {
    /// Create a new sassstring from parts.
    pub fn new(parts: Vec<StringPart>, quotes: Quotes) -> Self {
        // Any sequence of Raw parts in `parts` shall be merged to a
        // single Raw part.  Interpolation parts are left as is.
        let mut p2 = vec![];
        let mut buf = String::new();
        for part in parts {
            match part {
                StringPart::Raw(s) => buf.push_str(&s),
                interpolation => {
                    if !buf.is_empty() {
                        let mut buf2 = String::new();
                        swap(&mut buf, &mut buf2);
                        p2.push(StringPart::Raw(buf2));
                    }
                    p2.push(interpolation);
                }
            }
        }
        if !buf.is_empty() {
            p2.push(StringPart::Raw(buf));
        }
        SassString { parts: p2, quotes }
    }

    /// Evaluate this `SassString` to a `CssString`.
    ///
    /// All interpolated values are interpolated in the given `scope`.
    pub fn evaluate(&self, scope: ScopeRef) -> Result<CssString, Error> {
        let mut result = String::new();
        for part in &self.parts {
            match *part {
                StringPart::Interpolation(ref v) => {
                    let v = v.evaluate(scope.clone())?.unquote();
                    if self.quotes == Quotes::None {
                        let v = v
                            .valid_css()
                            .no_pos()? // TODO: Get the position.
                            .format(scope.get_format())
                            .to_string();
                        result.push_str(&v);
                    } else {
                        let v = v.format(scope.get_format()).to_string();
                        let mut carry_space = false;
                        for c in v.chars() {
                            if carry_space {
                                if c.is_ascii_hexdigit() || c == '\t' {
                                    result.push(' ');
                                }
                                carry_space = false;
                            }
                            if c == '\\' {
                                write!(&mut result, "\\{c}")?;
                            } else if c.is_alphanumeric()
                                || c.is_ascii_graphic()
                                || c == ' '
                                || c == '\t'
                                || c == char::REPLACEMENT_CHARACTER
                            {
                                result.push(c);
                            } else if !c.is_control()
                                && c != '\n'
                                && c != '\t'
                            {
                                result.push('\\');
                                result.push(c);
                            } else {
                                write!(&mut result, "\\{:x}", u32::from(c))?;
                                carry_space = true;
                            }
                        }
                    }
                }
                StringPart::Raw(ref s) => {
                    result.push_str(s);
                }
            }
        }
        Ok(CssString::new(result, self.quotes))
    }

    /// Return true if self represents an unquoted string.
    pub fn is_unquoted(&self) -> bool {
        self.quotes == Quotes::None
    }
    /// Check if this `SassString` is a single raw value.
    ///
    /// If so, return a reference to it.
    pub fn single_raw(&self) -> Option<&str> {
        if self.parts.len() == 1 {
            if let StringPart::Raw(ref s) = self.parts[0] {
                return Some(s);
            }
        }
        None
    }
    pub(crate) fn prepend(&mut self, data: &str) {
        if let Some(StringPart::Raw(ref mut first)) = self.parts.get_mut(0) {
            first.insert_str(0, data);
        } else {
            self.parts.insert(0, data.into());
        }
    }
    pub(crate) fn append_str(&mut self, data: &str) {
        if let Some(StringPart::Raw(ref mut last)) = self.parts.last_mut() {
            last.push_str(data);
        } else {
            self.parts.push(data.into());
        }
    }
    pub(crate) fn append(&mut self, other: &Self) {
        self.parts.extend_from_slice(&other.parts);
    }
}

impl fmt::Display for SassString {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        self.quotes.fmt(out)?;
        for part in &self.parts {
            match *part {
                StringPart::Raw(ref s) => s.fmt(out)?,
                StringPart::Interpolation(ref v) => {
                    panic!("Interpolation should be evaluated: {v:?}")
                }
            }
        }
        self.quotes.fmt(out)
    }
}

impl From<CssString> for SassString {
    fn from(s: CssString) -> Self {
        SassString {
            parts: vec![StringPart::Raw(s.value().into())],
            quotes: s.quotes(),
        }
    }
}

impl From<&str> for StringPart {
    fn from(s: &str) -> Self {
        StringPart::Raw(s.to_string())
    }
}

impl From<&str> for SassString {
    fn from(s: &str) -> Self {
        SassString {
            parts: vec![StringPart::from(s)],
            quotes: Quotes::None,
        }
    }
}
impl From<String> for SassString {
    fn from(value: String) -> Self {
        SassString {
            parts: vec![StringPart::Raw(value)],
            quotes: Quotes::None,
        }
    }
}
