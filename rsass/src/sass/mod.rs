//! Value and Item types (and some supporting) for sass.

macro_rules! name {
    ($name:ident) => {
        crate::sass::Name::from_static(stringify!($name))
    };
    () => {
        // an empty name
        crate::sass::Name::from_static("")
    };
}

mod call_args;
mod callable;
mod formal_args;
mod functions;
mod item;
mod mixin;
mod name;
mod selectors;
mod srcrange;
mod srcvalue;
mod string;
mod value;
mod variabledeclaration;

pub use self::call_args::CallArgs;
pub use self::callable::{Call, Callable, Closure};
pub use self::formal_args::{ArgsError, FormalArgs};
pub use self::functions::{
    get_global_module, CallError, Function, ResolvedArgs,
};
pub use self::item::{Expose, Item, ItemBody, UseAs};
pub use self::mixin::{Mixin, MixinDecl};
pub use self::name::Name;
pub use self::selectors::{Selector, SelectorPart, Selectors};
pub use self::string::{SassString, StringPart};
pub use self::value::{BinOp, Value};
pub use self::variabledeclaration::VariableDeclaration;

pub(crate) use srcrange::SrcRange;
pub(crate) use srcvalue::SrcValue;

pub(crate) mod parser {
    pub(crate) use super::srcrange::parser::*;
    pub(crate) use super::variabledeclaration::parser::*;
}
