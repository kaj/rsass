use super::Format;
use crate::css::{BodyItem, Rule};
use crate::sass::SassString;
use crate::Error;
use std::io::{self, Write};

pub struct CssBuf {
    buf: Vec<u8>,
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

    pub fn combine_final(head: Self, body: Self) -> Vec<u8> {
        let mut result = vec![];
        let compressed = head.format.is_compressed();
        if !head.is_ascii() || !body.is_ascii() {
            if compressed {
                // U+FEFF is byte order mark, used to show encoding.
                result.extend_from_slice("\u{feff}".as_bytes());
            } else {
                result.extend_from_slice(b"@charset \"UTF-8\";\n");
            }
        }
        result.extend_from_slice(&head.buf);
        result.extend_from_slice(&body.buf);
        if compressed && result.last() == Some(&b';') {
            result.pop();
        }
        if result.last().unwrap_or(&b'\n') != &b'\n' {
            result.push(b'\n');
        }
        result
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
            self.add_one("}\n", "}");
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
                        .unwrap_or(indent);

                    self.add_str("/*");
                    if indent > existing {
                        let start = self.format.get_indent(indent - existing);
                        self.add_str(&c.replace("\n", start));
                    } else {
                        self.add_str(c);
                    }
                    self.add_str("*/");
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
        self.add_one(";\n", ";");
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
        self.add_str(self.format.get_indent(self.indent))
    }
    fn do_indent_no_nl(&mut self) {
        let stuff = self.format.get_indent(self.indent);
        if stuff.len() > 1 {
            self.add_str(&stuff[1..])
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
        self.buf.extend_from_slice(&sub.buf);
    }
    pub fn add_str(&mut self, sub: &str) {
        self.buf.extend_from_slice(sub.as_bytes())
    }
    pub fn add_one(&mut self, normal: &str, compressed: &str) {
        self.add_str(if self.format.is_compressed() {
            compressed
        } else {
            normal
        })
    }
}

impl Write for CssBuf {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        self.buf.extend_from_slice(data);
        Ok(data.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
