use crate::error::Error;
use crate::sass::Value;
use crate::value::Quotes;
use crate::variablescope::Scope;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SassString {
    parts: Vec<StringPart>,
    quotes: Quotes,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum StringPart {
    Raw(String),
    Interpolation(Value),
}

impl SassString {
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
                        p2.push(StringPart::Raw(special_unescape(&buf)));
                        buf.clear();
                    }
                    p2.push(interpolation);
                }
            }
        }
        if !buf.is_empty() {
            p2.push(StringPart::Raw(special_unescape(&buf)));
        }
        fn special_unescape(s: &str) -> String {
            if s.contains("\\#{") && !s.contains('}') {
                s.replace("\\#{", "#{")
            } else {
                s.to_string()
            }
        }
        SassString { parts: p2, quotes }
    }
    fn evaluate_inner(
        &self,
        scope: &dyn Scope,
    ) -> Result<(String, Quotes), Error> {
        // Note This is an extremely peculiar special case;
        // A single-quoted string consisting only of an interpolation
        // becomes double-quoted.
        if self.quotes != Quotes::None && self.parts.len() == 1 {
            if let StringPart::Interpolation(ref v) = self.parts[0] {
                let s = v
                    .evaluate(scope)?
                    .unquote()
                    .format(scope.get_format())
                    .to_string()
                    .replace('\n', "\\a");
                if s.contains('"') && !s.contains('\'') {
                    return Ok((s, Quotes::Single));
                } else {
                    return Ok((s, Quotes::Double));
                }
            }
        }
        let mut result = String::new();
        let mut interpolated = false;
        for part in &self.parts {
            match *part {
                StringPart::Interpolation(ref v) => {
                    interpolated = true;
                    let mut v = v
                        .evaluate(scope)?
                        .unquote()
                        .format(scope.get_format())
                        .to_string();
                    if self.quotes == Quotes::None {
                        v = v.replace('\n', " ");
                    } else {
                        v = v.replace('\n', "\\a");
                    }
                    result.push_str(&v)
                }
                StringPart::Raw(ref s) => {
                    for c in s.chars() {
                        if c.is_control() {
                            result.push_str(&format!("\\{:x} ", c as usize))
                        } else {
                            result.push(c)
                        }
                    }
                }
            }
        }
        if interpolated
            && self.quotes == Quotes::Double
            && result.contains('"')
            && !result.contains('\'')
        {
            Ok((result, Quotes::Single))
        } else {
            Ok((result, self.quotes))
        }
    }

    pub fn evaluate(
        &self,
        scope: &dyn Scope,
    ) -> Result<(String, Quotes), Error> {
        let (result, quotes) = self.evaluate_inner(scope)?;
        if self.quotes == Quotes::Single && !result.contains('"') {
            Ok((result, Quotes::Double))
        } else {
            Ok((result, quotes))
        }
    }

    pub fn evaluate2(&self, scope: &dyn Scope) -> Result<SassString, Error> {
        let (result, quotes) = self.evaluate_inner(scope)?;
        Ok(SassString {
            parts: vec![StringPart::Raw(result)],
            quotes,
        })
    }

    pub fn evaluate_opt_unquote(
        &self,
        scope: &dyn Scope,
    ) -> Result<SassString, Error> {
        let (result, quotes) = self.evaluate(scope)?;
        let t = !result.is_empty()
            && result.bytes().enumerate().all(|(i, c)| {
                (c >= b'a' && c <= b'z')
                    || (c >= b'A' && c <= b'Z')
                    || (i > 0 && c == b'-')
            });
        Ok(SassString {
            parts: vec![StringPart::Raw(result)],
            quotes: if t { Quotes::None } else { quotes },
        })
    }

    pub fn opt_unquote(&self) -> SassString {
        let t = self
            .single_raw()
            .map(|s| {
                let mut bytes = s.bytes();
                bytes
                    .next()
                    .map(|c| c.is_ascii_alphabetic())
                    .unwrap_or(false)
                    && bytes.all(|c| c.is_ascii_alphanumeric())
            })
            .unwrap_or(false);
        SassString {
            parts: self.parts.clone(),
            quotes: if t { Quotes::None } else { self.quotes },
        }
    }

    pub fn is_unquoted(&self) -> bool {
        self.quotes == Quotes::None
    }
    pub fn single_raw(&self) -> Option<&str> {
        if self.parts.len() == 1 {
            if let StringPart::Raw(ref s) = self.parts[0] {
                return Some(s);
            }
        }
        None
    }
    pub fn prepend(&mut self, data: &str) {
        if let Some(StringPart::Raw(ref mut first)) = self.parts.get_mut(0) {
            first.insert_str(0, data);
        } else {
            self.parts.insert(0, data.into());
        }
    }
    pub fn append_str(&mut self, data: &str) {
        if let Some(StringPart::Raw(ref mut last)) = self.parts.last_mut() {
            last.push_str(data);
        } else {
            self.parts.push(data.into());
        }
    }
    pub fn append(&mut self, other: &Self) {
        self.parts.extend_from_slice(&other.parts);
    }
    pub fn into_parts(self) -> Vec<StringPart> {
        self.parts
    }
}

impl fmt::Display for SassString {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        self.quotes.fmt(out)?;
        for part in &self.parts {
            match *part {
                StringPart::Raw(ref s) => s.fmt(out)?,
                StringPart::Interpolation(ref v) => {
                    panic!("Interpolation should be evaluated: {:?}", v)
                }
            }
        }
        self.quotes.fmt(out)
    }
}

impl From<&str> for StringPart {
    fn from(s: &str) -> Self {
        StringPart::Raw(s.to_string())
    }
}

impl<'a> From<&'a str> for SassString {
    fn from(s: &'a str) -> Self {
        SassString {
            parts: vec![StringPart::from(s)],
            quotes: Quotes::None,
        }
    }
}
