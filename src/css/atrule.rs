use super::{Comment, Import, Property, Rule};
use crate::output::CssBuf;
use std::io::{self, Write};

/// An `@something` rule in css.
///
/// Note that some well-known at rules (`@media`, `@keyframes`, ...)
/// probably should have their own types.
#[derive(Clone, Debug)]
pub struct AtRule {
    name: String,
    args: String, // TODO: More typed?  Value? Or separate types for @media etc?
    body: Vec<AtRuleBodyItem>,
}

impl AtRule {
    pub(crate) fn new(
        name: String,
        args: String,
        body: Vec<AtRuleBodyItem>,
    ) -> Self {
        AtRule { name, args, body }
    }
    pub(crate) fn write(&self, buf: &mut CssBuf) -> io::Result<()> {
        write!(buf, "@{}", self.name)?;
        if !self.args.is_empty() {
            write!(buf, " {}", self.args)?;
        }
        buf.start_block();
        for item in &self.body {
            item.write(buf)?;
        }
        buf.end_block();
        Ok(())
    }
}

/// Something that may exist in the body of an [`AtRule`].
#[derive(Clone, Debug)]
pub enum AtRuleBodyItem {
    /// An `@import` statement.
    Import(Import),
    /// A comment
    Comment(Comment),
    /// A rule
    Rule(Rule),
    /// A raw property.
    Property(Property),
}

impl AtRuleBodyItem {
    pub(crate) fn write(&self, buf: &mut CssBuf) -> io::Result<()> {
        match self {
            AtRuleBodyItem::Import(import) => import.write(buf)?,
            AtRuleBodyItem::Comment(comment) => comment.write(buf),
            AtRuleBodyItem::Rule(rule) => rule.write(buf)?,
            AtRuleBodyItem::Property(property) => property.write(buf),
        }
        Ok(())
    }
}
impl From<Rule> for AtRuleBodyItem {
    fn from(rule: Rule) -> Self {
        AtRuleBodyItem::Rule(rule)
    }
}
impl From<Comment> for AtRuleBodyItem {
    fn from(rule: Comment) -> Self {
        AtRuleBodyItem::Comment(rule)
    }
}
impl From<Import> for AtRuleBodyItem {
    fn from(rule: Import) -> Self {
        AtRuleBodyItem::Import(rule)
    }
}
impl From<Property> for AtRuleBodyItem {
    fn from(rule: Property) -> Self {
        AtRuleBodyItem::Property(rule)
    }
}
