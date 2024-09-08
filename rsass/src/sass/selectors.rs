//! This module contains types for the selectors of a rule.
//!
//! Basically, in a rule like `p.foo, .foo p { some: thing; }` there
//! is a `Selectors` object which contains two `Selector` objects, one
//! for `p.foo` and one for `.foo p`.
//!
//! This _may_ change to a something like a tree of operators with
//! leafs of simple selectors in some future release.
use crate::css::{self, parser::selector_set};
use crate::parser::input_span;
use crate::sass::SassString;
use crate::{Error, ParseError, ScopeRef};
use std::fmt::Write;

/// A full set of selectors
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct Selectors {
    /// The actual selectors.
    s: Vec<Selector>,
}

impl Selectors {
    /// Create a root (empty) selector.
    pub fn root() -> Self {
        Self::new(vec![Selector::root()])
    }
    /// Return true if this is a root (empty) selector.
    pub fn is_root(&self) -> bool {
        self.s == [Selector::root()]
    }
    /// Create a new Selectors from a vec of selectors.
    pub fn new(s: Vec<Selector>) -> Self {
        Self { s }
    }

    /// Evaluate any interpolation in these Selectors.
    pub fn eval(&self, scope: ScopeRef) -> Result<css::SelectorSet, Error> {
        let mut s = Vec::new();
        for sel in &self.s {
            s.extend(sel.eval(scope.clone())?);
        }
        Ok(css::SelectorSet { s })
    }
    fn write_eval(
        &self,
        f: &mut String,
        scope: ScopeRef,
    ) -> Result<(), Error> {
        if let Some((first, rest)) = self.s.split_first() {
            first.write_eval(f, scope.clone())?;
            for s in rest {
                f.push_str(", ");
                s.write_eval(f, scope.clone())?;
            }
        }
        Ok(())
    }
}

/// A css (or sass) selector.
///
/// A selector does not contain `,`.  If it does, it is a `Selectors`,
/// where each of the parts separated by the comma is a `Selector`.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct Selector(Vec<SelectorPart>);

impl Selector {
    /// Get the root (empty) selector.
    pub fn root() -> Self {
        Self(vec![])
    }
    /// Create a new selector from parts.
    pub fn new(s: Vec<SelectorPart>) -> Self {
        Self(s)
    }
    fn eval(&self, scope: ScopeRef) -> Result<Vec<css::Selector>, Error> {
        if self.0.is_empty() {
            Ok(vec![css::Selector::default()])
        } else {
            let mut text = String::new();
            self.write_eval(&mut text, scope)?;
            Ok(ParseError::check(selector_set(input_span(text).borrow()))?.s)
        }
    }

    fn write_eval(
        &self,
        f: &mut String,
        scope: ScopeRef,
    ) -> Result<(), Error> {
        for p in &self.0 {
            p.write_eval(f, scope.clone())?;
        }
        Ok(())
    }
}

/// A selector consist of a sequence of these parts.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
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
        /// The attribute name
        name: SassString,
        /// An operator
        op: String,
        /// A value to match.
        val: SassString,
        /// Optional modifier.
        modifier: Option<char>,
    },
    /// A css3 pseudo-element (::foo)
    PseudoElement {
        /// The name of the pseudo-element
        name: SassString,
        /// Arguments to the pseudo-element
        arg: Option<Selectors>,
    },
    /// A pseudo-class or a css2 pseudo-element (:foo)
    Pseudo {
        /// The name of the pseudo-class
        name: SassString,
        /// Arguments to the pseudo-class
        arg: Option<Selectors>,
    },
    /// A sass backref (`&`), to be replaced with outer selector.
    BackRef,
}

impl SelectorPart {
    fn write_eval(
        &self,
        f: &mut String,
        scope: ScopeRef,
    ) -> Result<(), Error> {
        match self {
            SelectorPart::Simple(s) => we(s, f, scope)?,
            SelectorPart::Descendant => f.push(' '),
            SelectorPart::RelOp(op) => {
                if let Some(ch) = char::from_u32(u32::from(*op)) {
                    f.push(' ');
                    f.push(ch);
                    f.push(' ');
                }
            }
            SelectorPart::Attribute {
                name,
                op,
                val,
                modifier,
            } => {
                f.push('[');
                we(name, f, scope.clone())?;
                f.push_str(op);
                let val = val.evaluate(scope)?.opt_unquote();
                write!(f, "{val}")?;
                if let Some(modifier) = modifier {
                    if val.quotes().is_none() {
                        f.push(' ');
                    }
                    f.push(*modifier);
                }
                f.push(']');
            }
            SelectorPart::PseudoElement { name, arg } => {
                f.push_str("::");
                we(name, f, scope.clone())?;
                if let Some(arg) = arg {
                    f.push('(');
                    arg.write_eval(f, scope)?;
                    f.push(')');
                }
            }
            SelectorPart::Pseudo { name, arg } => {
                f.push(':');
                we(name, f, scope.clone())?;
                if let Some(arg) = arg {
                    f.push('(');
                    arg.write_eval(f, scope)?;
                    f.push(')');
                }
            }
            SelectorPart::BackRef => f.push('&'),
        }
        Ok(())
    }
}

fn we(s: &SassString, f: &mut String, scope: ScopeRef) -> Result<(), Error> {
    write!(f, "{}", s.evaluate(scope)?)?;
    Ok(())
}
