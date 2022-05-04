use super::{Comment, CssString, Selectors, Value};
use crate::output::CssBuf;
use std::io::{self, Write};

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
}

impl Rule {
    /// Write this rule to a css output buffer.
    pub fn write(&self, buf: &mut CssBuf, skip_nl: bool) -> io::Result<()> {
        if !self.body.is_empty() {
            if skip_nl {
                buf.do_indent_no_nl();
            } else {
                buf.do_indent();
            }
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
pub enum BodyItem {
    /// An `@import` statement with a name and args.
    Import(Import),
    /// A property declaration with a name and a value.
    Property(String, Value),
    /// A property declaration with a name and a value.
    CustomProperty(String, CssString),
    /// A comment
    Comment(Comment),
}

impl BodyItem {
    /// Write this item to a css output buffer.
    pub fn write(&self, buf: &mut CssBuf) -> io::Result<()> {
        buf.do_indent();
        match self {
            BodyItem::Import(ref import) => import.write(buf),
            BodyItem::Property(ref name, ref val) => write!(
                buf,
                "{}:{}{};",
                name,
                if buf.format().is_compressed() {
                    ""
                } else {
                    " "
                },
                val.format(buf.format()).to_string().replace('\n', " "),
            ),
            BodyItem::CustomProperty(ref name, ref val) => write!(
                buf,
                "{}:{}{};",
                name,
                if val.quotes().is_none() || buf.format().is_compressed() {
                    ""
                } else {
                    " "
                },
                val,
            ),
            BodyItem::Comment(ref c) => {
                c.write(buf);
                Ok(())
            }
        }
    }
}

/// An `@import` rule in css.
pub struct Import {
    name: CssString,
    args: Value,
}

impl Import {
    /// Create a new `@import`.
    pub fn new(name: CssString, args: Value) -> Self {
        Import { name, args }
    }

    /// Write this comment to a css output buffer.
    pub fn write(&self, buf: &mut CssBuf) -> io::Result<()> {
        write!(buf, "@import {}", self.name)?;
        if !self.args.is_null() {
            write!(buf, " {}", self.args.format(buf.format()))?;
        }
        buf.add_one(";\n", ";");
        Ok(())
    }
}

impl From<Import> for BodyItem {
    fn from(import: Import) -> BodyItem {
        BodyItem::Import(import)
    }
}
