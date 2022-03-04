use super::Format;
use crate::css::{BodyItem, CssString, Rule, Value};
use crate::{Error, ScopeRef};
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::io::{self, Write};

/// A [CssBuf] for imports, that also keeps track of loaded modules.
pub struct CssHead {
    buf: CssBuf,
    modules: BTreeMap<String, ScopeRef>,
}

impl CssHead {
    pub fn new(format: Format) -> Self {
        CssHead {
            buf: CssBuf::new(format),
            modules: Default::default(),
        }
    }
    pub fn add_import(
        &mut self,
        name: CssString,
        args: Value,
    ) -> Result<(), Error> {
        self.buf.add_import(name, args)
    }

    pub fn load_module<Init>(
        &mut self,
        path: &str,
        init: Init,
    ) -> Result<ScopeRef, Error>
    where
        Init: FnOnce(&mut Self) -> Result<ScopeRef, Error>,
    {
        if let Some(loaded) = self.modules.get(path) {
            return Ok(loaded.clone());
        }
        let module = init(self)?;
        self.modules.insert(path.into(), module.clone());
        Ok(module)
    }

    pub fn merge_imports(&mut self, other: Self) {
        self.buf.buf.extend_from_slice(&other.buf.buf);
    }

    pub fn combine_final(&self, body: CssBuf) -> Vec<u8> {
        let mut result = vec![];
        let compressed = self.buf.format.is_compressed();
        if !self.buf.is_ascii() || !body.is_ascii() {
            if compressed {
                // U+FEFF is byte order mark, used to show encoding.
                result.extend_from_slice("\u{feff}".as_bytes());
            } else {
                result.extend_from_slice(b"@charset \"UTF-8\";\n");
            }
        }
        result.extend_from_slice(&self.buf.buf);
        result.extend_from_slice(&body.buf);
        if compressed && result.last() == Some(&b';') {
            result.pop();
        }
        if result.last().unwrap_or(&b'\n') != &b'\n' {
            result.push(b'\n');
        }
        result
    }
}

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
                BodyItem::Import(ref name, ref args) => {
                    write!(&mut self.buf, "@import {}", name)?;
                    if !args.is_null() {
                        write!(
                            &mut self.buf,
                            " {}",
                            args.format(self.format)
                        )?;
                    }
                    self.add_one(";\n", ";");
                }
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
                    match indent.cmp(&existing) {
                        Ordering::Greater => {
                            let start =
                                self.format.get_indent(indent - existing);
                            self.add_str(&c.replace('\n', start));
                        }
                        Ordering::Less => {
                            let start =
                                self.format.get_indent(existing - indent - 1);
                            self.add_str(&c.replace(start, "\n"));
                        }
                        Ordering::Equal => {
                            self.add_str(c);
                        }
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
        name: CssString,
        args: Value,
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
