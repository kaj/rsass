use super::{
    CallArgs, Callable, Name, SassString, Selectors, Value,
    VariableDeclaration,
};
use crate::input::SourcePos;
use std::collections::BTreeSet;

/// Every sass file is a sequence of sass items.
/// Scoping items contains further sequences of items.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub enum Item {
    /// An `@import` directive.
    Import(Vec<SassString>, Value, SourcePos),
    /// A variable declaration.
    VariableDeclaration(VariableDeclaration),

    /// An `@at-root` directive.
    AtRoot(Selectors, Vec<Item>),
    /// An `@media` directive.
    AtMedia {
        /// Any arguments
        args: Value,
        /// The directive may have a body.
        body: Option<Vec<Item>>,
        /// The source location of this at rule.
        pos: SourcePos,
    },
    /// A generic `@` directive.
    AtRule {
        /// The name of this directive
        name: SassString,
        /// Any arguments
        args: Value,
        /// The directive may have a body.
        body: Option<Vec<Item>>,
        /// The source location of this at rule.
        pos: SourcePos,
    },
    /// An `@debug` directive.
    Debug(Value),
    /// An `@warn` directive.
    Warn(Value),
    /// An `@error` directive.
    Error(Value, SourcePos),

    /// A `@mixin` directive, declaring a mixin.
    MixinDeclaration(String, Callable),
    /// An `@include` directive, calling a mixin.
    MixinCall(String, CallArgs, Option<Callable>, SourcePos),
    /// An `@content` directive (in a mixin declaration).
    Content(CallArgs, SourcePos),

    /// An `@function` declaration.
    FunctionDeclaration(String, Callable),
    /// An `@return` statement in a function declaration.
    Return(Value, SourcePos),

    /// An `@if` conditional directive.
    IfStatement(Value, Vec<Item>, Vec<Item>),
    /// An `@each` loop directive.
    ///
    /// The value may be or evaluate to a list.
    Each(Vec<Name>, Value, Vec<Item>),
    /// An `@for` loop directive.
    For {
        /// The name of the iteration variable.
        name: Name,
        /// The start value for the iteration.
        from: Box<Value>,
        /// The end value for the iteration.
        to: Box<Value>,
        /// True if the end should be included in the range.
        inclusive: bool,
        /// The body of the loop.
        body: Vec<Item>,
    },
    /// An `@while` loop directive.
    While(Value, Vec<Item>),

    /// An `@use` directive.
    Use(SassString, UseAs, Vec<(Name, Value, bool)>, SourcePos),
    /// An `@forward` directive.
    Forward(
        SassString,
        UseAs,
        Expose,
        Vec<(Name, Value, bool)>,
        SourcePos,
    ),

    /// Extend rule (not really supported yet).
    Extend(Selectors),

    /// A sass rule; selectors followed by a block of items.
    Rule(Selectors, Vec<Item>),
    /// A sass namespace rule; a name followed by a block of properties.
    NamespaceRule(SassString, Value, Vec<Item>),
    /// A sass property; a name and a value.
    /// The position is the full value.
    Property(SassString, Value, SourcePos),
    /// A custom property.
    CustomProperty(SassString, SassString),
    /// A comment (that might be preserved for the output).
    Comment(SassString),
    /// Nothing
    None,
}

impl From<VariableDeclaration> for Item {
    fn from(value: VariableDeclaration) -> Self {
        Self::VariableDeclaration(value)
    }
}

/// How an `@forward`-ed module should be exposed.
///
/// As directed by the `show` or `hide` keywords or their absense.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub enum Expose {
    /// No `show` or `hide` specified; expose everything.
    All,
    /// Only show the listed items.
    ///
    /// The first list is functions and mixins, the second is variables.
    Show(BTreeSet<Name>, BTreeSet<Name>),
    /// Hide the listed items, show everything else.
    ///
    /// The first list is functions and mixins, the second is variables.
    Hide(BTreeSet<Name>, BTreeSet<Name>),
}

impl Expose {
    /// Check if `name` should be exposed as a function/mixin.
    pub fn allow_fun(&self, name: &Name) -> bool {
        match self {
            Self::All => true,
            Self::Show(show, _) => show.contains(name),
            Self::Hide(hide, _) => !hide.contains(name),
        }
    }
    /// Check if `name` should be exposed as a variable.
    pub fn allow_var(&self, name: &Name) -> bool {
        match self {
            Self::All => true,
            Self::Show(_, show) => show.contains(name),
            Self::Hide(_, hide) => !hide.contains(name),
        }
    }
}

/// The `as` part of an `@use` or `@forward` directive.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub enum UseAs {
    /// A plain `@use foo;`.
    KeepName,
    /// Include the module contents directly in the scope, `@use foo as *;`.
    Star,
    /// An explicit name, `@use foo as bar`.
    Name(String),
    /// A prefix, `@forward foo as foo-*`.
    Prefix(String),
}
