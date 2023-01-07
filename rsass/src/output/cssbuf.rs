use super::Format;
use std::io::{self, Write};

pub struct CssBuf {
    buf: Vec<u8>,
    format: Format,
    indent: usize,
}

impl CssBuf {
    pub fn new(format: Format) -> CssBuf {
        CssBuf {
            buf: Vec::new(),
            format,
            indent: 0,
        }
    }
    pub fn take(self) -> Vec<u8> {
        self.buf
    }
    pub(crate) fn format(&self) -> Format {
        self.format
    }
    pub(crate) fn indent_level(&self) -> usize {
        self.indent
    }

    pub fn start_block(&mut self) {
        self.add_one(" {\n", "{");
        self.indent += 2;
    }
    pub fn end_block(&mut self) {
        if self.buf.last() == Some(&b'\n') {
            self.buf.pop();
        }
        if self.format.is_compressed() && self.buf.last() == Some(&b';') {
            self.buf.pop();
        }
        self.indent -= 2;
        if self.buf.last() != Some(&b'{') {
            self.do_indent();
        }
        self.add_one("}\n", "}");
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
    pub fn opt_nl(&mut self) {
        if !self.format.is_compressed()
            && !self.buf.is_empty()
            && !self.buf.ends_with(b"\n\n")
        {
            self.add_str("\n");
        }
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
