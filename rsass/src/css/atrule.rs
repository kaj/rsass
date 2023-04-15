use super::{
    BodyItem, Comment, CustomProperty, Import, Property, Rule, Value,
};
use crate::output::CssBuf;
use std::io::{self, Write};

/// An `@media rule in css.
#[derive(Clone, Debug)]
pub struct MediaRule {
    args: MediaArgs,
    body: Vec<AtRuleBodyItem>,
}

impl MediaRule {
    pub(crate) fn new(args: MediaArgs, body: Vec<AtRuleBodyItem>) -> Self {
        MediaRule { args, body }
    }
    pub(crate) fn write(&self, buf: &mut CssBuf) -> io::Result<()> {
        buf.do_indent_no_nl();
        buf.add_str("@media ");
        self.args.write(buf)?;
        //if !self.args.is_null() {
        //write!(buf, " {}", self.args.format(buf.format()))?;
        //}
        buf.start_block();
        for item in &self.body {
            item.write(buf)?;
        }
        buf.end_block();
        Ok(())
    }
}

/// The media selection argument of an `@media` rule.
#[derive(Clone, Debug)]
pub enum MediaArgs {
    /// `all` media.
    Name(String),
    /// `(cond: valud)` media.
    Condition(String, Value),
    /// unary logic
    Only(Box<MediaArgs>),
    /// Any media subquery in parenthesis.
    Paren(Box<MediaArgs>),
    /// unary logic
    Not(Box<MediaArgs>),
    /// a and b and c media.
    And(Vec<MediaArgs>),
    /// a or b or c media.
    Or(Vec<MediaArgs>),
}

impl MediaArgs {
    pub(crate) fn write(&self, buf: &mut CssBuf) -> io::Result<()> {
        match self {
            MediaArgs::Name(name) => write!(buf, "{name}")?,
            MediaArgs::Only(a) => {
                buf.add_str("only ");
                a.write(buf)?;
            }
            MediaArgs::Not(a) => {
                buf.add_str("not ");
                a.write(buf)?;
            }
            MediaArgs::Condition(c, v) => {
                write!(buf, "({c}: {})", v.format(buf.format()))?
            }
            MediaArgs::And(args) => {
                if let Some((first, rest)) = args.split_first() {
                    first.write(buf)?;
                    for arg in rest {
                        buf.add_str(" and ");
                        arg.write(buf)?;
                    }
                }
            }
            MediaArgs::Or(args) => {
                if let Some((first, rest)) = args.split_first() {
                    first.write(buf)?;
                    for arg in rest {
                        buf.add_str(" or ");
                        arg.write(buf)?;
                    }
                }
            }
            MediaArgs::Paren(a) => {
                buf.add_str("(");
                a.write(buf)?;
                buf.add_str(")");
            }
        }
        Ok(())
    }
}

/// An `@something` rule in css.
///
/// Note that some well-known at rules (`@media`, `@keyframes`, ...)
/// probably should have their own types.
#[derive(Clone, Debug)]
pub struct AtRule {
    name: String,
    args: Value,
    // Some<[]> outputs "{}", None outputs ";".
    body: Option<Vec<AtRuleBodyItem>>,
}

impl AtRule {
    pub(crate) fn new(
        name: String,
        args: Value,
        body: Option<Vec<AtRuleBodyItem>>,
    ) -> Self {
        AtRule { name, args, body }
    }
    pub(crate) fn no_body(&self) -> bool {
        self.body.is_none()
    }
    pub(crate) fn write(&self, buf: &mut CssBuf) -> io::Result<()> {
        buf.do_indent_no_nl();
        write!(buf, "@{}", self.name)?;
        if !self.args.is_null() {
            write!(buf, " {}", self.args.format(buf.format()))?;
        }
        if let Some(body) = &self.body {
            buf.start_block();
            for item in body {
                item.write(buf)?;
            }
            buf.end_block();
        } else {
            buf.add_one(";\n", ";");
        }
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
    /// A custom property declaration with a name and a value.
    CustomProperty(CustomProperty),
    /// An `@` rule.
    AtRule(AtRule),
}

impl AtRuleBodyItem {
    pub(crate) fn write(&self, buf: &mut CssBuf) -> io::Result<()> {
        match self {
            AtRuleBodyItem::Import(import) => import.write(buf)?,
            AtRuleBodyItem::Comment(comment) => comment.write(buf),
            AtRuleBodyItem::Rule(rule) => rule.write(buf)?,
            AtRuleBodyItem::Property(property) => property.write(buf),
            AtRuleBodyItem::CustomProperty(cp) => cp.write(buf),
            AtRuleBodyItem::AtRule(rule) => rule.write(buf)?,
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
impl From<AtRule> for AtRuleBodyItem {
    fn from(rule: AtRule) -> Self {
        AtRuleBodyItem::AtRule(rule)
    }
}
impl From<MediaRule> for AtRuleBodyItem {
    fn from(_: MediaRule) -> Self {
        todo!("This should not be needed")
    }
}
impl From<BodyItem> for AtRuleBodyItem {
    fn from(value: BodyItem) -> Self {
        match value {
            BodyItem::Import(i) => AtRuleBodyItem::Import(i),
            BodyItem::Property(p) => AtRuleBodyItem::Property(p),
            BodyItem::CustomProperty(p) => AtRuleBodyItem::CustomProperty(p),
            BodyItem::Comment(c) => AtRuleBodyItem::Comment(c),
            BodyItem::ARule(r) => AtRuleBodyItem::AtRule(r),
        }
    }
}
