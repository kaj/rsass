use super::{CallArgs, FormalArgs, Name, SassString, Value};
use crate::functions::SassFunction;
use crate::parser::SourcePos;
use crate::selectors::Selectors;

/// Every sass file is a sequence of sass items.
/// Scoping items contains further sequences of items.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub enum Item {
    /// An `@import` directive.
    Import(Vec<SassString>, Value, SourcePos),
    /// A variable declaration.
    VariableDeclaration {
        name: String,
        val: Value,
        default: bool,
        global: bool,
    },
    /// An `@at-root` directive.
    AtRoot {
        selectors: Selectors,
        body: Vec<Item>,
    },
    /// A generic `@` directive.
    AtRule {
        name: String,
        args: Value,
        body: Option<Vec<Item>>,
    },
    /// An `@debug` directive.
    Debug(Value),
    /// An `@warn` directive.
    Warn(Value),
    /// An `@error` directive.
    Error(Value),

    /// A `@mixin` directive, declaring a mixin.
    MixinDeclaration {
        name: String,
        args: FormalArgs,
        body: Vec<Item>,
    },
    /// An `@include` directive, calling a mixin.
    MixinCall {
        name: String,
        args: CallArgs,
        body: Vec<Item>,
    },
    /// An `@content` directive (in a mixin declaration).
    Content,

    /// An `@function` declaration.
    FunctionDeclaration {
        name: String,
        func: SassFunction,
    },
    /// An `@return` statement in a function declaration.
    Return(Value),

    /// An `@if` conditional directive.
    IfStatement(Value, Vec<Item>, Vec<Item>),
    /// An `@each` loop directive.
    ///
    /// The value may be or evaluate to a list.
    Each(Vec<Name>, Value, Vec<Item>),
    /// An `@for` loop directive.
    For {
        name: Name,
        from: Box<Value>,
        to: Box<Value>,
        inclusive: bool,
        body: Vec<Item>,
    },
    /// An `@while` loop directive.
    While(Value, Vec<Item>),

    /// An `@use` directive.
    Use(SassString, UseAs),
    /// A sass rule; selectors followed by a block of items.
    Rule(Selectors, Vec<Item>),
    /// A sass namespace rule; a name followed by a block of properties.
    NamespaceRule(SassString, Value, Vec<Item>),
    /// A sass property; a name and a value.
    Property(SassString, Value),
    /// A comment (that might be preserved for the output).
    Comment(SassString),
    /// Nothing
    None,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub enum UseAs {
    KeepName,
    Star,
    Name(SassString),
}
