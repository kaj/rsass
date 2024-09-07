//! This module contains types for the selectors of a rule.
//!
//! Basically, in a rule like `p.foo, .foo p { some: thing; }` there
//! is a `SelectorSet` object which contains two `Selector` objects, one
//! for `p.foo` and one for `.foo p`.
mod attribute;
mod compound;
mod context;
mod cssselectorset;
mod elemtype;
mod error;
mod logical;
mod opt;
mod pseudo;
mod selectorset;

use self::attribute::Attribute;
use self::elemtype::ElemType;
pub(crate) use self::opt::Opt;
use self::pseudo::Pseudo;
pub use context::SelectorCtx;
pub(crate) use cssselectorset::CssSelectorSet;
pub use error::BadSelector;
pub use logical::Selector;
pub use selectorset::SelectorSet;

pub(crate) mod parser {
    pub(super) use super::attribute::parser::attribute;
    pub(super) use super::compound::parser::compound_selector;
    pub(super) use super::elemtype::parser::{
        elem_name, keyframe_stop, name_opt_ns,
    };
    pub(super) use super::logical::parser::selector;
    pub(super) use super::pseudo::parser::pseudo;
    pub(crate) use super::selectorset::parser::selector_set;
}
