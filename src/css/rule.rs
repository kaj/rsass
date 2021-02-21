use super::Value;
use crate::selectors::Selectors;

/// A css rule
pub struct Rule {
    pub(crate) selectors: Selectors,
    pub(crate) body: Vec<BodyItem>,
}

impl Rule {
    /// Create a new Rule
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
    /// ...
    pub fn mut_body(&mut self) -> &mut Vec<BodyItem> {
        &mut self.body
    }
}

/// Something that may exist inside a rule.
pub enum BodyItem {
    /// A property declaration
    Property(String, Value),
    /// A comment
    Comment(String),
}
