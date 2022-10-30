use super::{AtRule, Comment, CssString, Rule, Value};
use crate::output::CssBuf;
use std::io::{self, Write};

/// A top-level item in a css file.
#[derive(Clone, Debug)]
pub enum Item {
    /// A comment
    Comment(Comment),
    /// A css import statement
    Import(Import),
    /// A css rule.
    Rule(Rule),
    /// An `@` rule, e.g. `@media ... { ... }`
    AtRule(AtRule),
}

impl Item {
    pub(crate) fn write(&self, buf: &mut CssBuf) -> io::Result<()> {
        match self {
            Item::Comment(comment) => comment.write(buf),
            Item::Import(import) => import.write(buf)?,
            Item::Rule(rule) => rule.write(buf)?,
            Item::AtRule(atrule) => atrule.write(buf)?,
        }
        Ok(())
    }
}

impl From<Comment> for Item {
    fn from(comment: Comment) -> Item {
        Item::Comment(comment)
    }
}
impl From<Import> for Item {
    fn from(import: Import) -> Item {
        Item::Import(import)
    }
}
impl From<Rule> for Item {
    fn from(rule: Rule) -> Item {
        Item::Rule(rule)
    }
}
impl From<AtRule> for Item {
    fn from(value: AtRule) -> Self {
        Item::AtRule(value)
    }
}

/// An `@import` rule in css.
#[derive(Clone, Debug)]
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
    pub(crate) fn write(&self, buf: &mut CssBuf) -> io::Result<()> {
        buf.do_indent_no_nl();
        write!(buf, "@import {}", self.name)?;
        if !self.args.is_null() {
            write!(buf, " {}", self.args.format(buf.format()))?;
        }
        buf.add_one(";\n", ";");
        Ok(())
    }
}
