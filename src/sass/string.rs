use sass::Value;
use std::fmt;
use value::Quotes;
use variablescope::Scope;

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
        SassString {
            parts: p2,
            quotes,
        }
    }
    pub fn evaluate(&self, scope: &Scope) -> (String, Quotes) {
        // Note This is an extremely peculiar special case;
        // A single-quoted string consisting only of an interpolation
        // becomes double-quoted.
        if self.quotes != Quotes::None && self.parts.len() == 1 {
            if let StringPart::Interpolation(ref v) = self.parts[0] {
                let s = format!("{}", v.evaluate(scope).unquote())
                    .replace('\\', "\\\\")
                    .replace('\n', "\\a");
                if s.contains('"') && !s.contains('\'') {
                    return (s, Quotes::Single);
                } else {
                    return (s, Quotes::Double);
                }
            }
        }
        let mut result = String::new();
        let mut interpolated = false;
        for part in &self.parts {
            match *part {
                StringPart::Interpolation(ref v) => {
                    interpolated = true;
                    let mut v = format!("{}", v.evaluate(scope).unquote());
                    if self.quotes == Quotes::None {
                        v = v.replace('\n', " ");
                    } else {
                        v = v.replace('\\', "\\\\").replace('\n', "\\a");
                    }
                    result.push_str(&v)
                }
                StringPart::Raw(ref s) => result.push_str(s),
            }
        }
        if interpolated && self.quotes == Quotes::Double
            && result.contains('"') && !result.contains('\'')
        {
            (result, Quotes::Single)
        } else {
            (result, self.quotes)
        }
    }
    pub fn evaluate2(&self, scope: &Scope) -> SassString {
        let (result, quotes) = self.evaluate(scope);
        SassString {
            parts: vec![StringPart::Raw(result)],
            quotes: quotes,
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
    pub fn append(&mut self, other: &Self) {
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
                    panic!("Interpolation should be evaluated: {:?}", v)
                }
            }
        }
        self.quotes.fmt(out)
    }
}

impl<'a> From<&'a str> for SassString {
    fn from(s: &'a str) -> Self {
        SassString {
            parts: vec![StringPart::Raw(s.to_string())],
            quotes: Quotes::None,
        }
    }
}
