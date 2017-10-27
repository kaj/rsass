use sass::Value;
use std::fmt;
use value::Quotes;
use variablescope::Scope;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SassString {
    parts: Vec<StringPart>,
    quotes: Quotes,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StringPart {
    Raw(String),
    Interpolation(Value),
}

impl SassString {
    pub fn new(parts: Vec<StringPart>, quotes: Quotes) -> Self {
        SassString { parts, quotes }
    }
    pub fn evaluate(&self, scope: &Scope) -> SassString {
        SassString {
            parts: self.parts
                .iter()
                .map(|p| match *p {
                         StringPart::Interpolation(ref v) => {
                             StringPart::Raw(format!("{}",
                                                     v.evaluate(scope)
                                                         .unquote()))
                         }
                         ref s => s.clone(),
                     })
                .collect(),
            quotes: self.quotes,
        }
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
