use super::{AtRule, Comment, CssString, Import, OldSelectors, Value};
use crate::output::CssBuf;
use std::io::{self, Write};

/// A css rule.
///
/// A rule binds [`Selectors`] to a body of [`BodyItem`]s (mainly
/// properties with [`Value`]s).
#[derive(Clone, Debug)]
pub struct Rule {
    pub(crate) selectors: OldSelectors,
    pub(crate) body: Vec<BodyItem>,
}

impl Rule {
    /// Create a new Rule.
    pub fn new(selectors: OldSelectors) -> Self {
        Self {
            selectors,
            body: Vec::new(),
        }
    }
    /// Add an item to the body of this rule.
    pub fn push(&mut self, item: BodyItem) {
        self.body.push(item);
    }

    /// Write this rule to a css output buffer.
    pub(crate) fn write(&self, buf: &mut CssBuf) -> io::Result<()> {
        if !self.body.is_empty() {
            if let Some(selectors) = self.selectors.no_placeholder() {
                buf.do_indent_no_nl();
                if buf.format().is_compressed() {
                    write!(buf, "{selectors:#}")?;
                } else {
                    write!(buf, "{selectors}")?;
                }
                buf.start_block();
                for item in &self.body {
                    item.write(buf)?;
                }
                buf.end_block();
            }
        }
        Ok(())
    }
}

/// Something that may exist inside a rule.
#[derive(Clone, Debug)]
pub enum BodyItem {
    /// An `@import` statement with a name and args.
    Import(Import),
    /// A property declaration with a name and a value.
    Property(Property),
    /// A custom property declaration with a name and a value.
    CustomProperty(CustomProperty),
    /// A comment
    Comment(Comment),
    /// Empty at-rules are allowed in a rule body.
    ARule(AtRule),
}

impl BodyItem {
    /// Write this item to a css output buffer.
    pub(crate) fn write(&self, buf: &mut CssBuf) -> io::Result<()> {
        match self {
            Self::Comment(c) => c.write(buf),
            Self::Import(import) => import.write(buf)?,
            Self::Property(property) => property.write(buf),
            Self::CustomProperty(property) => property.write(buf),
            Self::ARule(rule) => rule.write(buf)?,
        }
        Ok(())
    }
}

impl From<Comment> for BodyItem {
    fn from(comment: Comment) -> Self {
        Self::Comment(comment)
    }
}
impl From<Import> for BodyItem {
    fn from(import: Import) -> Self {
        Self::Import(import)
    }
}
impl From<Property> for BodyItem {
    fn from(property: Property) -> Self {
        Self::Property(property)
    }
}
impl From<CustomProperty> for BodyItem {
    fn from(property: CustomProperty) -> Self {
        Self::CustomProperty(property)
    }
}

impl TryFrom<AtRule> for BodyItem {
    type Error = AtRule;

    fn try_from(value: AtRule) -> Result<Self, Self::Error> {
        if value.no_body() {
            Ok(Self::ARule(value))
        } else {
            Err(value)
        }
    }
}

/// A css property; a name and [Value].
#[derive(Clone, Debug)]
pub struct Property {
    name: String,
    value: Value,
}

impl Property {
    /// Create a new Property.
    pub fn new(name: String, value: Value) -> Self {
        Self { name, value }
    }
    pub(crate) fn write(&self, buf: &mut CssBuf) {
        buf.do_indent_no_nl();
        buf.add_str(&self.name);
        buf.add_one(": ", ":");
        buf.add_str(&self.value.to_string(buf.format()).replace('\n', " "));
        buf.add_one(";\n", ";");
    }
}

/// A css custom property (css variable); a name and a literal value.
#[derive(Clone, Debug)]
pub struct CustomProperty {
    name: String,
    value: CssString,
}

impl CustomProperty {
    /// Construct a new custom property.
    pub fn new(name: String, value: CssString) -> Self {
        Self { name, value }
    }
    pub(crate) fn write(&self, buf: &mut CssBuf) {
        buf.do_indent_no_nl();
        buf.add_str(&self.name);
        buf.add_str(":");
        if !(self.value.quotes().is_none() || buf.format().is_compressed()) {
            buf.add_str(" ");
        }
        buf.add_str(&self.value.to_string());
        buf.add_one(";\n", ";");
    }
}
