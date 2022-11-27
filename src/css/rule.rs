use super::{AtRule, Comment, CssString, Import, Selectors, Value};
use crate::output::CssBuf;
use std::io::{self, Write};

/// A css rule.
///
/// A rule binds [`Selectors`] to a body of [`BodyItem`]s (mainly
/// properties with [`Value`]s).
#[derive(Clone, Debug)]
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

    /// Write this rule to a css output buffer.
    pub(crate) fn write(&self, buf: &mut CssBuf) -> io::Result<()> {
        if !self.body.is_empty() {
            buf.do_indent_no_nl();
            if buf.format().is_compressed() {
                write!(buf, "{:#}", self.selectors)?;
            } else {
                write!(buf, "{}", self.selectors)?;
            }
            buf.start_block();
            for item in &self.body {
                item.write(buf)?;
            }
            buf.end_block();
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
    /// A property declaration with a name and a value.
    CustomProperty(String, CssString),
    /// A comment
    Comment(Comment),
    /// Empty at-rules are allowed in a rule body.
    ARule(AtRule),
}

impl BodyItem {
    /// Write this item to a css output buffer.
    pub(crate) fn write(&self, buf: &mut CssBuf) -> io::Result<()> {
        match self {
            BodyItem::Comment(c) => c.write(buf),
            BodyItem::Import(import) => import.write(buf)?,
            BodyItem::Property(property) => property.write(buf),
            BodyItem::CustomProperty(ref name, ref val) => {
                buf.do_indent_no_nl();
                write!(
                    buf,
                    "{}:{}{}",
                    name,
                    if val.quotes().is_none() || buf.format().is_compressed()
                    {
                        ""
                    } else {
                        " "
                    },
                    val,
                )?;
                buf.add_one(";\n", ";");
            }
            BodyItem::ARule(rule) => rule.write(buf)?,
        }
        Ok(())
    }
}

impl From<Comment> for BodyItem {
    fn from(comment: Comment) -> BodyItem {
        BodyItem::Comment(comment)
    }
}
impl From<Import> for BodyItem {
    fn from(import: Import) -> BodyItem {
        BodyItem::Import(import)
    }
}
impl From<Property> for BodyItem {
    fn from(property: Property) -> BodyItem {
        BodyItem::Property(property)
    }
}

impl TryFrom<AtRule> for BodyItem {
    type Error = AtRule;

    fn try_from(value: AtRule) -> Result<Self, Self::Error> {
        if value.no_body() {
            Ok(BodyItem::ARule(value))
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
        Property { name, value }
    }
    pub(crate) fn write(&self, buf: &mut CssBuf) {
        buf.do_indent_no_nl();
        buf.add_str(&self.name);
        buf.add_one(": ", ":");
        buf.add_str(
            &self
                .value
                .format(buf.format())
                .to_string()
                .replace('\n', " "),
        );
        buf.add_one(";\n", ";");
    }
}
