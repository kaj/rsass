use nom::is_alphanumeric;
use parseutil::{opt_spacelike, spacelike};
use std::fmt;
use std::str::from_utf8;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Selector(Vec<SelectorPart>);

impl Selector {
    pub fn root() -> Self {
        Selector(vec![])
    }

    pub fn join(&self, other: &Selector) -> Selector {
        let mut split = other.0.splitn(2, |p| p == &SelectorPart::BackRef);
        let o1 = split.next().unwrap();
        if let Some(o2) = split.next() {
            let mut result = o1.to_vec();
            result.extend(self.0.iter().cloned());
            result.extend(o2.iter().cloned());
            Selector(result)
        } else {
            let mut result = self.0.clone();
            if !result.is_empty() &&
               !other.0.first().map(|p| p.is_operator()).unwrap_or(false) {
                result.push(SelectorPart::Descendant);
            }
            result.extend(other.0.iter().cloned());
            Selector(result)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SelectorPart {
    Simple(String),
    Descendant,
    RelOp(u8), // >, +, ~
    Attribute {
        name: String,
        op: String,
        val: String,
    },
    BackRef,
}

named!(pub selector<Selector>,
       map!(many1!(selector_part),
            |s: Vec<SelectorPart>| {
                let mut s = s;
                if s.last() == Some(&SelectorPart::Descendant) {
                    s.pop();
                }
                Selector(s)
            }));

named!(selector_part<&[u8], SelectorPart>,
       alt_complete!(
           map!(take_while1!(is_selector_char),
                |s: &[u8]| SelectorPart::Simple(from_utf8(s).unwrap().into())) |
           do_parse!(tag!("[") >> opt_spacelike >>
                     name: take_while1!(is_selector_char) >> opt_spacelike >>
                     op: alt_complete!(tag!("*=") | tag!("=")) >>
                     opt_spacelike >>
                     val: alt_complete!(
                         map!(delimited!(tag!("\""),
                                         escaped!(is_not!("\\\""), '\\',
                                                  one_of!("\"\\")),
                                         tag!("\"")),
                              |s| format!("\"{}\"", from_utf8(s).unwrap())) |
                         map!(delimited!(tag!("'"),
                                         escaped!(is_not!("\\'"), '\\',
                                                  one_of!("'\\")),
                                         tag!("'")),
                              |s| format!("'{}'", from_utf8(s).unwrap()))) >>
                     opt_spacelike >>
                     tag!("]") >>
                     (SelectorPart::Attribute {
                         name: from_utf8(name).unwrap().into(),
                         op: from_utf8(op).unwrap().into(),
                         val: val,
                     })) |
           value!(SelectorPart::BackRef, tag!("&")) |
           delimited!(opt_spacelike,
                      alt!(value!(SelectorPart::RelOp(b'>'), tag!(">")) |
                           value!(SelectorPart::RelOp(b'+'), tag!("+")) |
                           value!(SelectorPart::RelOp(b'~'), tag!("~"))),
                      opt_spacelike) |
           value!(SelectorPart::Descendant, spacelike)
           ));

fn is_selector_char(chr: u8) -> bool {
    is_alphanumeric(chr) || chr == b'_' || chr == b'.' || chr == b'#'
}

impl SelectorPart {
    fn is_operator(&self) -> bool {
        match self {
            &SelectorPart::Simple(_) => false,
            &SelectorPart::Descendant => true,
            &SelectorPart::RelOp(_) => true,
            &SelectorPart::Attribute { .. } => false,
            &SelectorPart::BackRef => false,
        }
    }
}

impl fmt::Display for Selector {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        for ref p in &self.0 {
            try!(write!(out, "{}", p));
        }
        Ok(())
    }
}

impl fmt::Display for SelectorPart {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &SelectorPart::Simple(ref s) => {
                write!(out, "{}", s)
            }
            &SelectorPart::Descendant => write!(out, " "),
            &SelectorPart::RelOp(ref c) => {
                write!(out, " {} ", c.clone() as char)
            }
            &SelectorPart::Attribute { ref name, ref op, ref val } => {
                write!(out, "[{}{}{}]", name, op, val)
            }
            &SelectorPart::BackRef => write!(out, "&"),
        }
    }
}

#[cfg(test)]
mod test {
    use nom::IResult::*;
    use selectors::*;

    #[test]
    fn root_join() {
        let s = Selector(vec![SelectorPart::Simple("foo".into())]);
        assert_eq!(Selector::root().join(&s), s)
    }

    #[test]
    fn simple_selector() {
        assert_eq!(selector(b"foo "),
                   Done(&b""[..], Selector(
                       vec![SelectorPart::Simple("foo".into())])))
    }

    #[test]
    fn selector2() {
        assert_eq!(selector(b"foo bar "),
                   Done(&b""[..], Selector(
                       vec![SelectorPart::Simple("foo".into()),
                            SelectorPart::Descendant,
                            SelectorPart::Simple("bar".into())])))
    }

    #[test]
    fn child_selector() {
        assert_eq!(selector(b"foo > bar "),
                   Done(&b""[..], Selector(
                       vec![SelectorPart::Simple("foo".into()),
                            SelectorPart::RelOp(b'>'),
                            SelectorPart::Simple("bar".into())])))
    }
}
