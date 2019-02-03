//! This module contains types for the selectors of a rule.
//!
//! Basically, in a rule like `p.foo, .foo p { some: thing; }` there
//! is a `Selectors` object which contains two `Selector` objects, one
//! for `p.foo` and one for `.foo p`.
//!
//! This _may_ change to a something like a tree of operators with
//! leafs of simple selectors in some future release.
use css::Value;
use sass::SassString;
use std::fmt;
use std::io::Write;
use value::{ListSeparator, Quotes};

/// A full set of selectors
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Selectors {
    pub s: Vec<Selector>,
    backref: Selector,
}

impl Selectors {
    /// Create a root (empty) selector.
    pub fn root() -> Self {
        Selectors::new(vec![Selector::root()])
    }
    pub fn new(s: Vec<Selector>) -> Self {
        Selectors {
            s,
            backref: Selector::root(),
        }
    }
    pub fn one(&self) -> Selector {
        self.s.first().cloned().unwrap_or_else(Selector::root)
    }
    /// Create the full selector for when self is used inside a parent selector.
    pub fn inside(&self, parent: &Self) -> Self {
        let mut result = Vec::new();
        for p in &parent.s {
            for s in &self.s {
                result.push(p.join(s, &parent.backref));
            }
        }
        Selectors {
            s: result,
            backref: parent.backref.clone(),
        }
    }
    pub fn with_backref(self, context: Selector) -> Self {
        self.inside(&Selectors {
            s: vec![Selector::root()],
            backref: context,
        })
    }
    /// Create a sass `Value` representing this set of selectors.
    pub fn to_value(&self) -> Value {
        let content = self
            .s
            .iter()
            .map(|s: &Selector| {
                Value::List(
                    format!("{}", s)
                        .split_whitespace()
                        .map(|p| Value::Literal(p.to_string(), Quotes::None))
                        .collect(),
                    ListSeparator::Space,
                    false,
                )
            })
            .collect::<Vec<_>>();
        let sep = if content.len() == 1 {
            ListSeparator::Space
        } else {
            ListSeparator::Comma
        };
        Value::List(content, sep, false)
    }
}

/// A css (or sass) selector.
///
/// A selector does not contain `,`.  If it does, it is a `Selectors`,
/// where each of the parts separated by the comma is a `Selector`.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Selector(pub Vec<SelectorPart>);

impl Selector {
    pub fn root() -> Self {
        Selector(vec![])
    }

    fn join(&self, other: &Selector, alt_context: &Selector) -> Selector {
        let mut split = other.0.splitn(2, |p| p == &SelectorPart::BackRef);
        let o1 = split.next().unwrap();
        if let Some(o2) = split.next() {
            let mut result = o1.to_vec();
            if self.0.is_empty() {
                result.extend(alt_context.0.iter().cloned());
            } else {
                result.extend(self.0.iter().cloned());
            }
            result.extend(o2.iter().cloned());
            Selector(result)
        } else {
            let mut result = self.0.clone();
            if !result.is_empty()
                && !other.0.first().map(|p| p.is_operator()).unwrap_or(false)
            {
                result.push(SelectorPart::Descendant);
            }
            result.extend(other.0.iter().cloned());
            Selector(result)
        }
    }
}

/// A selector consist of a sequence of these parts.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SelectorPart {
    /// A simple selector, eg a class, id or element name.
    ///
    /// Note that a Simple selector can hide a more complex selector
    /// through string interpolation.
    Simple(SassString),
    /// The empty relational operator.
    ///
    /// The thing after this is a descendant of the thing before this.
    Descendant,
    /// A relational operator; `>`, `+`, `~`.
    RelOp(u8),
    /// An attribute selector
    Attribute {
        name: SassString,
        op: String,
        val: SassString,
    },
    /// A css3 pseudo-element (::foo)
    PseudoElement {
        name: SassString,
        arg: Option<Selectors>,
    },
    /// A pseudo-class or a css2 pseudo-element (:foo)
    Pseudo {
        name: SassString,
        arg: Option<Selectors>,
    },
    /// A sass backref (`&`), to be replaced with outer selector.
    BackRef,
}

impl SelectorPart {
    fn is_operator(&self) -> bool {
        match *self {
            SelectorPart::Descendant | SelectorPart::RelOp(_) => true,
            SelectorPart::Simple(_)
            | SelectorPart::Attribute { .. }
            | SelectorPart::PseudoElement { .. }
            | SelectorPart::Pseudo { .. }
            | SelectorPart::BackRef => false,
        }
    }
}

impl fmt::Display for Selectors {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        if let Some((first, rest)) = self.s.split_first() {
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
            SelectorPart::Attribute {
                ref name,
                ref op,
                ref val,
            } => write!(out, "[{}{}{}]", name, op, val),
            SelectorPart::PseudoElement { ref name, ref arg } => {
                write!(out, "::{}", name)?;
                if let Some(ref arg) = *arg {
                    if out.alternate() {
                        write!(out, "({:#})", arg)?
                    } else {
                        write!(out, "({})", arg)?
                    }
                }
                Ok(())
            }
            SelectorPart::Pseudo { ref name, ref arg } => {
                let name = format!("{}", name);
                if let Some(ref arg) = *arg {
                    // It seems some pseudo-classes should always have
                    // their arg in compact form.  Maybe we need more
                    // hard-coded names here, or maybe the condition
                    // should be on the argument rather than the name?
                    if out.alternate()
                        || name == "nth-child"
                        || name == "nth-of-type"
                    {
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
    use super::*;

    #[test]
    fn root_join() {
        let s = Selector(vec![SelectorPart::Simple("foo".into())]);
        assert_eq!(Selector::root().join(&s, &Selector::root()), s)
    }

    #[test]
    fn simple_join() {
        let s = Selector(vec![SelectorPart::Simple("foo".into())]).join(
            &Selector(vec![SelectorPart::Simple(".bar".into())]),
            &Selector::root(),
        );
        assert_eq!(format!("{}", s), "foo .bar")
    }

    #[test]
    fn backref_join() {
        let s = Selector(vec![SelectorPart::Simple("foo".into())]).join(
            &Selector(vec![
                SelectorPart::BackRef,
                SelectorPart::Simple(".bar".into()),
            ]),
            &Selector::root(),
        );
        assert_eq!(format!("{}", s), "foo.bar")
    }
}
