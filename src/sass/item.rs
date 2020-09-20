use crate::functions::SassFunction;
use crate::sass::{CallArgs, FormalArgs, SassString, Value};
use crate::selectors::Selectors;

/// Every sass file is a sequence of sass items.
/// Scoping items contains further sequences of items.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Item {
    Import(Vec<SassString>, Value),
    VariableDeclaration {
        name: String,
        val: Value,
        default: bool,
        global: bool,
    },
    AtRoot {
        selectors: Selectors,
        body: Vec<Item>,
    },
    AtRule {
        name: String,
        args: Value,
        body: Option<Vec<Item>>,
    },
    Error(Value),

    MixinDeclaration {
        name: String,
        args: FormalArgs,
        body: Vec<Item>,
    },
    MixinCall {
        name: String,
        args: CallArgs,
        body: Vec<Item>,
    },
    Content,

    FunctionDeclaration {
        name: String,
        func: SassFunction,
    },
    Return(Value),

    IfStatement(Value, Vec<Item>, Vec<Item>),
    /// The value may be or evaluate to a list.
    Each(Vec<String>, Value, Vec<Item>),
    For {
        name: String,
        from: Box<Value>,
        to: Box<Value>,
        inclusive: bool,
        body: Vec<Item>,
    },
    While(Value, Vec<Item>),

    Rule(Selectors, Vec<Item>),
    NamespaceRule(SassString, Value, Vec<Item>),
    Property(SassString, Value),
    Comment(String),
    Warn(Value),
    None,
}
