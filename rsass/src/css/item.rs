use super::{AtRule, Comment, MediaRule, Rule, Value};
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
    /// An `@media` rule.
    MediaRule(MediaRule),
    /// An (unknown) `@` rule.
    AtRule(AtRule),
    /// An extra newline for grouping (unless compressed format).
    Separator,
}

impl Item {
    pub(crate) fn write(&self, buf: &mut CssBuf) -> io::Result<()> {
        match self {
            Self::Comment(comment) => comment.write(buf),
            Self::Import(import) => import.write(buf)?,
            Self::Rule(rule) => rule.write(buf)?,
            Self::MediaRule(rule) => rule.write(buf)?,
            Self::AtRule(atrule) => atrule.write(buf)?,
            Self::Separator => buf.opt_nl(),
        }
        Ok(())
    }
}

impl From<Comment> for Item {
    fn from(comment: Comment) -> Self {
        Self::Comment(comment)
    }
}
impl From<Import> for Item {
    fn from(import: Import) -> Self {
        Self::Import(import)
    }
}
impl From<Rule> for Item {
    fn from(rule: Rule) -> Self {
        Self::Rule(rule)
    }
}
impl From<AtRule> for Item {
    fn from(value: AtRule) -> Self {
        Self::AtRule(value)
    }
}
impl From<MediaRule> for Item {
    fn from(value: MediaRule) -> Self {
        Self::MediaRule(value)
    }
}

/// An `@import` rule in css.
#[derive(Clone, Debug)]
pub struct Import {
    name: Value,
    args: Value,
}

impl Import {
    /// Create a new `@import`.
    pub fn new(name: Value, args: Value) -> Self {
        Self { name, args }
    }

    /// Write this comment to a css output buffer.
    pub(crate) fn write(&self, buf: &mut CssBuf) -> io::Result<()> {
        buf.do_indent_no_nl();
        write!(buf, "@import {}", &self.name.format(buf.format()))?;
        if !self.args.is_null() {
            write!(buf, " {}", self.args.format(buf.format()))?;
        }
        buf.add_one(";\n", ";");
        Ok(())
    }
}
