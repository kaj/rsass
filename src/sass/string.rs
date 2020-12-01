use crate::error::Error;
use crate::sass::Value;
use crate::value::Quotes;
use crate::variablescope::Scope;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct SassString {
    parts: Vec<StringPart>,
    quotes: Quotes,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
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
    pub fn evaluate(
        &self,
        scope: &dyn Scope,
    ) -> Result<(String, Quotes), Error> {
        let mut result = String::new();
        let mut interpolated = false;
        for part in &self.parts {
            match *part {
                StringPart::Interpolation(ref v) => {
                    interpolated = true;
                    let v = v
                        .evaluate(scope)?
                        .unquote()
                        .format(scope.get_format())
                        .to_string();
                    if self.quotes == Quotes::None {
                        result.push_str(&v);
                    } else {
                        let mut carry_space = false;
                        for c in v.chars() {
                            if carry_space {
                                if c.is_ascii_hexdigit() || c == '\t' {
                                    result.push(' ');
                                }
                                carry_space = false;
                            }
                            if c == '\\' {
                                result.push_str(&format!("\\{}", c));
                            } else if c.is_alphanumeric()
                                || c.is_ascii_graphic()
                                || c == ' '
                                || c == '\t'
                                || c == '\u{fffd}'
                            {
                                result.push(c);
                            } else if !c.is_control()
                                && c != '\n'
                                && c != '\t'
                            {
                                result.push_str(&format!("\\{}", c));
                            } else {
                                result.push_str(&format!(
                                    "\\{:x}",
                                    u32::from(c)
                                ));
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
    pub fn evaluate2(&self, scope: &dyn Scope) -> Result<SassString, Error> {
        let (result, quotes) = self.evaluate(scope)?;
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
        let mut chars = result.chars();
        let t = chars.next()
            .map(|c| c.is_alphabetic()) // first letter
            .unwrap_or(false) // not empty
            && chars.all(|c| c.is_alphanumeric() || c == '-');
        Ok(SassString {
            parts: vec![StringPart::Raw(result)],
            quotes: if t { Quotes::None } else { quotes },
        })
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

impl From<&str> for SassString {
    fn from(s: &str) -> Self {
        SassString {
            parts: vec![StringPart::from(s)],
            quotes: Quotes::None,
        }
    }
}
