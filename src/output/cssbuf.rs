use super::Format;
use crate::css::{BodyItem, Rule};
use crate::sass::SassString;
use crate::Error;
use std::io::{self, Write};

pub struct CssBuf {
    pub buf: Vec<u8>,
    format: Format,
    indent: usize,
    separate: bool,
}

impl CssBuf {
    pub fn new(format: Format) -> CssBuf {
        CssBuf::_new(format, 0)
    }
    pub fn new_as(orig: &Self) -> CssBuf {
        CssBuf::_new(orig.format, orig.indent)
    }
    pub fn new_below(orig: &Self) -> CssBuf {
        CssBuf::_new(orig.format, orig.indent + 2)
    }
    fn _new(format: Format, indent: usize) -> CssBuf {
        CssBuf {
            buf: Vec::new(),
            format,
            indent,
            separate: false,
        }
    }

    pub fn write_rule(
        &mut self,
        rule: &Rule,
        skip_nl: bool,
    ) -> io::Result<()> {
        if !rule.body.is_empty() {
            if skip_nl {
                self.do_indent_no_nl();
            } else {
                self.do_indent();
            }
            if self.format.is_compressed() {
                write!(self.buf, "{:#}{{", rule.selectors)?;
            } else {
                write!(self.buf, "{} {{", rule.selectors)?;
            }
            self.indent += 2;
            self.write_body_items(&rule.body)?;
            self.indent -= 2;
            self.do_indent();
            self.buf.write_all(if !self.format.is_compressed() {
                b"}\n"
            } else {
                b"}"
            })?;
        }
        Ok(())
    }

    pub fn write_body_items(&mut self, items: &[BodyItem]) -> io::Result<()> {
        for item in items {
            self.do_indent();
            match item {
                BodyItem::Property(ref name, ref val) => write!(
                    self.buf,
                    "{}:{}{};",
                    name,
                    if self.format.is_compressed() { "" } else { " " },
                    val.format(self.format).to_string().replace('\n', " "),
                )?,
                BodyItem::Comment(ref c) => {
                    let indent = self.indent;
                    let existing = c
                        .lines()
                        .skip(1)
                        .map(|s| s.bytes().take_while(|b| *b == b' ').count())
                        .min()
                        .unwrap_or(0);
                    let c = if existing < indent {
                        c.replace(
                            "\n",
                            self.format.get_indent(indent - existing),
                        )
                    } else {
                        c.clone()
                    };
                    self.buf.extend(b"/*");
                    self.buf.extend(c.as_bytes());
                    self.buf.extend(b"*/");
                }
            }
        }
        if self.format.is_compressed() && self.buf.last() == Some(&b';') {
            self.buf.pop();
        }
        Ok(())
    }

    pub fn add_import(
        &mut self,
        name: SassString,
        args: crate::css::Value,
    ) -> Result<(), Error> {
        self.do_indent_no_nl();
        write!(&mut self.buf, "@import {}", name)?;
        if !args.is_null() {
            write!(&mut self.buf, " {}", args.format(self.format))?;
        }
        self.buf.extend(if self.format.is_compressed() {
            &b";"[..]
        } else {
            &b";\n"[..]
        });
        Ok(())
    }

    pub fn do_separate(&mut self) {
        if self.separate {
            if !self.format.is_compressed() && !self.buf.is_empty() {
                self.buf.push(b'\n');
            }
        } else {
            self.separate = true;
        }
    }
    pub fn do_indent(&mut self) {
        self.buf
            .extend(self.format.get_indent(self.indent).as_bytes())
    }
    fn do_indent_no_nl(&mut self) {
        let stuff = self.format.get_indent(self.indent);
        if stuff.len() > 1 {
            self.buf.extend(stuff[1..].as_bytes())
        }
    }

    pub fn is_root_level(&self) -> bool {
        self.indent == 0
    }
    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }
    pub fn is_ascii(&self) -> bool {
        self.buf.is_ascii()
    }

    pub fn join(&mut self, sub: Self) {
        self.buf.extend(sub.buf);
    }
}
