use nom::is_alphanumeric;
use parser::util::{opt_spacelike, spacelike2};
use std::fmt;
use std::io::Write;
use std::str::from_utf8;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Selectors(pub Vec<Selector>);

impl Selectors {
    pub fn root() -> Self {
        Selectors(vec![Selector::root()])
    }
    pub fn inside(&self, parent: Option<&Self>) -> Self {
        if let Some(parent) = parent {
            let mut result = Vec::new();
            for ref p in &parent.0 {
                for ref s in &self.0 {
                    result.push(p.join(s));
                }
            }
            Selectors(result)
        } else {
            self.clone()
        }
    }
}

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
    Attribute { name: String, op: String, val: String },
    Pseudo { name: String, arg: Option<Selectors> },
    BackRef,
}

named!(pub selectors<Selectors>,
       map!(separated_nonempty_list!(
           do_parse!(tag!(",") >> opt!(is_a!(", \t\n")) >> ()),
           selector),
            |s| Selectors(s)));

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
           map!(selector_string, |s| SelectorPart::Simple(s)) |
           value!(SelectorPart::Simple("*".to_string()), tag!("*")) |
           do_parse!(tag!(":") >>
                     name: selector_string >>
                     arg: opt!(delimited!(tag!("("), selectors,
                                          tag!(")"))) >>
                     (SelectorPart::Pseudo {
                         name: name,
                         arg: arg,
                     })) |
           do_parse!(tag!("[") >> opt_spacelike >>
                     name: selector_string >> opt_spacelike >>
                     op: alt_complete!(tag!("*=") | tag!("|=") | tag!("=")) >>
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
                         name: name,
                         op: from_utf8(op).unwrap().into(),
                         val: val,
                     })) |
           do_parse!(tag!("[") >> opt_spacelike >>
                     name: selector_string >> opt_spacelike >>
                     tag!("]") >>
                     (SelectorPart::Attribute {
                         name: name,
                         op: "".to_string(),
                         val: "".to_string(),
                     })) |
           value!(SelectorPart::BackRef, tag!("&")) |
           delimited!(opt_spacelike,
                      alt!(value!(SelectorPart::RelOp(b'>'), tag!(">")) |
                           value!(SelectorPart::RelOp(b'+'), tag!("+")) |
                           value!(SelectorPart::RelOp(b'~'), tag!("~")) |
                           value!(SelectorPart::RelOp(b'\\'), tag!("\\"))),
                      opt_spacelike) |
           value!(SelectorPart::Descendant, spacelike2)
           ));


named!(selector_string<String>,
       fold_many1!(alt_complete!(selector_plain_part | selector_escaped_part),
                   String::new(),
                   |mut acc: String, item: &[u8]| {
                       acc.push_str(from_utf8(item).unwrap());
                       acc
                   }));
named!(selector_plain_part<&[u8]>,
       take_while1!(is_selector_char));
named!(selector_escaped_part<&[u8]>,
       recognize!(preceded!(tag!("\\"), many_m_n!(1, 3, hexpair))));
named!(hexpair,
       recognize!(do_parse!(one_of!("0123456789ABCDEFabcdef") >>
                            one_of!("0123456789ABCDEFabcdef") >> ())));

fn is_selector_char(chr: u8) -> bool {
    is_alphanumeric(chr) || chr == b'_' || chr == b'-' || chr == b'.' ||
    chr == b'#'
}

impl SelectorPart {
    fn is_operator(&self) -> bool {
        match *self {
            SelectorPart::Descendant |
            SelectorPart::RelOp(_) => true,
            SelectorPart::Simple(_) |
            SelectorPart::Attribute { .. } |
            SelectorPart::Pseudo { .. } |
            SelectorPart::BackRef => false,
        }
    }
}

impl fmt::Display for Selectors {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        if let Some((first, rest)) = self.0.split_first() {
            first.fmt(out)?;
            let separator = if out.alternate() { "," } else { ", " };
            for item in rest {
                out.write_str(separator)?;
                item.fmt(out)?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for Selector {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        // Note: There should be smarter whitespace-handling here, avoiding
        // the need to clean up afterwards.
        let mut buf = vec![];
        for p in &self.0 {
            if out.alternate() {
                write!(&mut buf, "{:#}", p).map_err(|_| fmt::Error)?;
            } else {
                write!(&mut buf, "{}", p).map_err(|_| fmt::Error)?;
            }
        }
        while buf.last() == Some(&b' ') {
            buf.pop();
        }
        while buf.first() == Some(&b' ') {
            buf.remove(0);
        }
        let buf = String::from_utf8(buf).map_err(|_| fmt::Error)?;
        out.write_str(&buf.replace("  ", " "))
    }
}

impl fmt::Display for SelectorPart {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SelectorPart::Simple(ref s) => write!(out, "{}", s),
            SelectorPart::Descendant => write!(out, " "),
            SelectorPart::RelOp(ref c) => {
                if out.alternate() && *c != b'~' {
                    write!(out, "{}", *c as char)
                } else {
                    write!(out, " {} ", *c as char)
                }
            }
            SelectorPart::Attribute { ref name, ref op, ref val } => {
                write!(out, "[{}{}{}]", name, op, val)
            }
            SelectorPart::Pseudo { ref name, ref arg } => {
                if let Some(ref arg) = *arg {
                    if out.alternate() {
                        write!(out, ":{}({:#})", name, arg)
                    } else {
                        write!(out, ":{}({})", name, arg)
                    }
                } else {
                    write!(out, ":{}", name)
                }
            }
            SelectorPart::BackRef => write!(out, "&"),
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
                   Done(&b""[..],
                        Selector(vec![SelectorPart::Simple("foo".into())])))
    }
    #[test]
    fn escaped_simple_selector() {
        assert_eq!(selector(b"\\E9m "),
                   Done(&b""[..],
                        Selector(vec![SelectorPart::Simple("\\E9m".into())])))
    }

    #[test]
    fn selector2() {
        assert_eq!(selector(b"foo bar "),
                   Done(&b""[..],
                        Selector(vec![SelectorPart::Simple("foo".into()),
                                      SelectorPart::Descendant,
                                      SelectorPart::Simple("bar".into())])))
    }

    #[test]
    fn child_selector() {
        assert_eq!(selector(b"foo > bar "),
                   Done(&b""[..],
                        Selector(vec![SelectorPart::Simple("foo".into()),
                                      SelectorPart::RelOp(b'>'),
                                      SelectorPart::Simple("bar".into())])))
    }

    #[test]
    fn foo1_selector() {
        assert_eq!(selector(b"[data-icon='test-1'] "),
                   Done(&b""[..],
                        Selector(vec![SelectorPart::Attribute {
                            name: "data-icon".into(),
                            op: "=".into(),
                            val: "'test-1'".into(),
                        }])))
    }

    #[test]
    fn pseudo_selector() {
        assert_eq!(selector(b":before "),
                   Done(&b""[..],
                        Selector(vec![SelectorPart::Pseudo {
                                          name: "before".into(),
                                          arg: None,
                                      }])))
    }
    #[test]
    fn pseudo_on_simple_selector() {
        assert_eq!(selector(b"figure:before "),
                   Done(&b""[..],
                        Selector(vec![SelectorPart::Simple("figure".into()),
                                      SelectorPart::Pseudo {
                                          name: "before".into(),
                                          arg: None,
                                      }])))
    }
}
