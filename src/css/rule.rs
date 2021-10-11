use super::{CssString, Selectors, Value};

/// A css rule.
///
/// A rule binds [`Selectors`] to a body of [`BodyItem`]s (mainly
/// properties with [`Value`]s).
pub struct Rule {
    pub(crate) selectors: Selectors,
    pub(crate) body: Vec<BodyItem>,
}

impl Rule {
    /// Create a new Rule.
    pub fn new(selectors: Selectors) -> Rule {
        Rule {
            selectors,
            body: Vec::new(),
        }
    }
    /// Add an item to the body of this rule.
    pub fn push(&mut self, item: BodyItem) {
        self.body.push(item)
    }
    /// Add an import statement to the body of this rule.
    pub fn add_import(&mut self, name: CssString, args: Value) {
        self.body.push(BodyItem::Import(name, args))
    }
}

/// Something that may exist inside a rule.
pub enum BodyItem {
    /// An `@import` statement with a name and args.
    Import(CssString, Value),
    /// A property declaration with a name and a value.
    Property(String, Value),
    /// A comment
    Comment(String),
}
