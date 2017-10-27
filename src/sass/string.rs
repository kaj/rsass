use sass::Value;
use variablescope::Scope;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SassString {
    parts: Vec<StringPart>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StringPart {
    Raw(String),
    Interpolation(Value),
}

impl SassString {
    pub fn new(parts: Vec<StringPart>) -> Self {
        SassString { parts }
    }
    pub fn evaluate(&self, scope: &Scope) -> SassString {
        SassString {
            parts: self.parts.iter().map(|p| match *p {
                StringPart::Interpolation(ref v) => {
                    StringPart::Raw(format!("{}", v.evaluate(scope).unquote()))
                }
                ref s => s.clone(),
            }).collect(),
        }
    }
}

impl fmt::Display for SassString {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        for part in &self.parts {
            match *part {
                StringPart::Raw(ref s) => s.fmt(out)?,
                StringPart::Interpolation(ref v) => {
                    panic!("Interpolation should be evaluated: {:?}", v)
                }
            }
        }
        Ok(())
    }
}

impl<'a> From<&'a str> for SassString {
    fn from(s: &'a str) -> Self {
        SassString {
            parts: vec![StringPart::Raw(s.to_string())],
        }
    }
}
