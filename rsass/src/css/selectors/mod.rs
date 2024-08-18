//! This module contains types for the selectors of a rule.
//!
//! Basically, in a rule like `p.foo, .foo p { some: thing; }` there
//! is a `SelectorSet` object which contains two `Selector` objects, one
//! for `p.foo` and one for `.foo p`.
mod attribute;
mod context;
mod cssselectorset;
mod error;
mod logical;
mod opt;
mod pseudo;
mod selectorset;

pub(crate) use self::opt::Opt;
pub use context::SelectorCtx;
pub(crate) use cssselectorset::CssSelectorSet;
pub use error::BadSelector;
pub use logical::Selector;
pub use selectorset::SelectorSet;

pub(crate) mod parser {
    pub(crate) use super::selectorset::parser::selector_set;
}
