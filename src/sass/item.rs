use functions::SassFunction;
use sass::{CallArgs, FormalArgs, SassString, Value};
use selectors::Selectors;

/// Every sass file is a sequence of sass items.
/// Scoping items contains further sequences of items.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Item {
    Import(Value),
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
    MediaRule {
        queries: Vec<MediaQuery>, // Never empty.
        body: Vec<Item>,
    },
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
    NamespaceRule(String, Value, Vec<Item>),
    Property(SassString, Value),
    Comment(String),
    None,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MediaQuery {
    // The modifier, such as "not" or "only".
    pub modifier: Option<String>,

    // The media type, such as "screen" or "print".
    pub media_type: Option<String>,

    // Feature queries, including parenthesis.
    pub features: Vec<String>,
}
