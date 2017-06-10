use formalargs::{CallArgs, FormalArgs};
use functions::SassFunction;
use selectors::Selectors;
use value::Value;

/// Every sass file is a sequence of sass items.
/// Scoping items contains further sequences of items.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Item {
    Import(Value),
    VariableDeclaration {
        name: String,
        val: Value,
        default: bool,
        global: bool,
    },
    AtRule { name: String, args: Value, body: Option<Vec<Item>> },

    MixinDeclaration { name: String, args: FormalArgs, body: Vec<Item> },
    MixinCall { name: String, args: CallArgs, body: Vec<Item> },
    Content,

    FunctionDeclaration { name: String, func: SassFunction },
    Return(Value),

    IfStatement(Value, Vec<Item>, Vec<Item>),
    /// The value may be or evaluate to a list.
    Each(String, Value, Vec<Item>),
    For {
        name: String,
        from: Box<Value>,
        to: Box<Value>,
        inclusive: bool,
        body: Vec<Item>,
    },
    While(Value, Vec<Item>),

    Rule(Selectors, Vec<Item>),
    NamespaceRule(String, Value, Vec<Item>),
    Property(String, Value, bool),
    Comment(String),
    None,
}
