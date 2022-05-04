use super::Format;
use crate::css::Import;
use crate::{Error, ScopeRef};
use std::collections::BTreeMap;
use std::io::{self, Write};

/// A [CssBuf] for imports, that also keeps track of loaded modules.
pub struct CssHead {
    imports: Vec<Import>,
    modules: BTreeMap<String, ScopeRef>,
}

impl CssHead {
    pub fn new() -> Self {
        CssHead {
            imports: Default::default(),
            modules: Default::default(),
        }
    }
    pub fn add_import(&mut self, import: Import) {
        self.imports.push(import)
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
        self.imports.extend(other.imports);
    }

    pub fn combine_final(&self, body: CssBuf) -> Vec<u8> {
        let mut buf = CssBuf::new_as(&body);
        for i in &self.imports {
            i.write(&mut buf).unwrap();
        }
        let mut result = vec![];
        let compressed = body.format.is_compressed();
        if !buf.is_ascii() || !body.is_ascii() {
            if compressed {
                // U+FEFF is byte order mark, used to show encoding.
                result.extend_from_slice("\u{feff}".as_bytes());
            } else {
                result.extend_from_slice(b"@charset \"UTF-8\";\n");
            }
        }
        result.extend(buf.buf);
        result.extend(body.buf);
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
    pub(crate) indent: usize,
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
    pub(crate) fn format(&self) -> Format {
        self.format
    }
    pub(crate) fn indent_level(&self) -> usize {
        self.indent
    }

    pub fn start_block(&mut self) {
        self.add_one(" {", "{");
        self.indent += 2;
    }
    pub fn end_block(&mut self) {
        if self.format.is_compressed() && self.buf.last() == Some(&b';') {
            self.buf.pop();
        }
        self.indent -= 2;
        self.do_indent();
        self.add_one("}\n", "}");
    }

    pub fn add_import(&mut self, import: Import) -> Result<(), Error> {
        self.do_indent_no_nl();
        Ok(import.write(self)?)
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
    pub(crate) fn do_indent_no_nl(&mut self) {
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
