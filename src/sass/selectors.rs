//! This module contains types for the selectors of a rule.
//!
//! Basically, in a rule like `p.foo, .foo p { some: thing; }` there
//! is a `Selectors` object which contains two `Selector` objects, one
//! for `p.foo` and one for `.foo p`.
//!
//! This _may_ change to a something like a tree of operators with
//! leafs of simple selectors in some future release.
use crate::css;
use crate::sass::SassString;
use crate::{Error, ParseError, ScopeRef};

/// A full set of selectors
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct Selectors {
    /// The actual selectors.
    s: Vec<Selector>,
}

impl Selectors {
    /// Create a root (empty) selector.
    pub fn root() -> Self {
        Selectors::new(vec![Selector::root()])
    }
    /// Return true if this is a root (empty) selector.
    pub fn is_root(&self) -> bool {
        self.s == [Selector::root()]
    }
    /// Create a new Selectors from a vec of selectors.
    pub fn new(s: Vec<Selector>) -> Self {
        Selectors { s }
    }

    /// Evaluate any interpolation in these Selectors.
    pub fn eval(&self, scope: ScopeRef) -> Result<css::Selectors, Error> {
        let s = css::Selectors::new(
            self.s
                .iter()
                .map(|s| s.eval(scope.clone()))
                .collect::<Result<Vec<_>, Error>>()?,
        );
        // The "simple" parts we get from evaluating interpolations may
        // contain high-level selector separators (i.e. ","), so we need to
        // parse the selectors again, from a string representation.
        use crate::parser::css::selectors;
        use crate::parser::input_span;
        // TODO: Get the span from the source of self!
        Ok(ParseError::check(selectors(input_span(
            format!("{} ", s).as_bytes(),
        )))?)
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
        Selector(vec![])
    }
    /// Create a new selector from parts.
    pub fn new(s: Vec<SelectorPart>) -> Self {
        Selector(s)
    }
    fn eval(&self, scope: ScopeRef) -> Result<css::Selector, Error> {
        self.0
            .iter()
            .map(|sp| sp.eval(scope.clone()))
            .collect::<Result<_, _>>()
            .map(css::Selector)
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
    fn eval(&self, scope: ScopeRef) -> Result<css::SelectorPart, Error> {
        match *self {
            SelectorPart::Attribute {
                ref name,
                ref op,
                ref val,
                ref modifier,
            } => Ok(css::SelectorPart::Attribute {
                name: name.evaluate(scope.clone())?,
                op: op.clone(),
                val: val.evaluate(scope)?.opt_unquote(),
                modifier: *modifier,
            }),
            SelectorPart::Simple(ref v) => {
                Ok(css::SelectorPart::Simple(v.evaluate(scope)?.to_string()))
            }
            SelectorPart::Pseudo { ref name, ref arg } => {
                let arg = match &arg {
                    Some(ref a) => Some(a.eval(scope.clone())?),
                    None => None,
                };
                Ok(css::SelectorPart::Pseudo {
                    name: name.evaluate(scope)?,
                    arg,
                })
            }
            SelectorPart::PseudoElement { ref name, ref arg } => {
                let arg = match &arg {
                    Some(ref a) => Some(a.eval(scope.clone())?),
                    None => None,
                };
                Ok(css::SelectorPart::PseudoElement {
                    name: name.evaluate(scope)?,
                    arg,
                })
            }
            SelectorPart::Descendant => Ok(css::SelectorPart::Descendant),
            SelectorPart::RelOp(op) => Ok(css::SelectorPart::RelOp(op)),
            SelectorPart::BackRef => Ok(css::SelectorPart::BackRef),
        }
    }
}
