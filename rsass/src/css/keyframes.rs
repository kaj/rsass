use super::{Comment, Property};
use crate::output::CssBuf;
use std::io::{self, Write};

/// An `@keyframes` rule in css.
#[derive(Clone, Debug)]
pub struct Keyframes {
    name: String,
    items: Vec<KfItem>,
}

impl Keyframes {
    pub(crate) fn new(name: String, items: Vec<KfItem>) -> Self {
        Keyframes { name, items }
    }
    pub(crate) fn write(&self, buf: &mut CssBuf) -> io::Result<()> {
        write!(buf, "@keyframes {}", self.name)?;
        if let &[KfItem::Comment(ref single)] = &self.items[..] {
            buf.add_one("{ ", "{");
            single.write(buf);
            buf.pop_nl();
            buf.add_one(" }", "}");
        } else {
            buf.start_block();
            for item in &self.items {
                match item {
                    KfItem::Stop(name, rules) => {
                        buf.do_indent_no_nl();
                        if let Some((first, rest)) = name.split_first() {
                            buf.add_str(first);
                            for name in rest {
                                buf.add_one(", ", ",");
                                buf.add_str(name);
                            }
                        }
                        buf.start_block();
                        for rule in rules {
                            rule.write(buf);
                        }
                        buf.end_block();
                    }
                    KfItem::Comment(comment) => comment.write(buf),
                }
            }
            buf.end_block();
        }
        Ok(())
    }
}

/// An item in keyframes, either a stop or a comment.
#[derive(Clone, Debug)]
pub enum KfItem {
    /// A stop
    Stop(Vec<String>, Vec<Property>),
    /// A comment
    Comment(Comment),
}
