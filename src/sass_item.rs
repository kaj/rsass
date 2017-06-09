use formalargs::{CallArgs, FormalArgs};
use functions::SassFunction;
use selectors::Selectors;
use value::Value;

/// Every sass file is a sequence of sass items.
/// Scoping items contains further sequences of items.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SassItem {
    Import(Value),
    VariableDeclaration {
        name: String,
        val: Value,
        default: bool,
        global: bool,
    },
    AtRule {
        name: String,
        args: Value,
        body: Option<Vec<SassItem>>,
    },

    MixinDeclaration {
        name: String,
        args: FormalArgs,
        body: Vec<SassItem>,
    },
    MixinCall { name: String, args: CallArgs, body: Vec<SassItem> },
    Content,

    FunctionDeclaration { name: String, func: SassFunction },
    Return(Value),

    IfStatement(Value, Vec<SassItem>, Vec<SassItem>),
    /// The value may be or evaluate to a list.
    Each(String, Value, Vec<SassItem>),
    For {
        name: String,
        from: Box<Value>,
        to: Box<Value>,
        inclusive: bool,
        body: Vec<SassItem>,
    },
    While(Value, Vec<SassItem>),

    Rule(Selectors, Vec<SassItem>),
    NamespaceRule(String, Value, Vec<SassItem>),
    Property(String, Value, bool),
    Comment(String),
    None,
}
