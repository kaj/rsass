use super::Value;
use crate::output::Format;
use crate::selectors::Selectors;
use std::io::Write;

/// A css rule
pub struct Rule {
    selectors: Selectors,
    body: Vec<BodyItem>,
}

impl Rule {
    /// Create a new Rule
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
    /// ...
    pub fn mut_body(&mut self) -> &mut Vec<BodyItem> {
        &mut self.body
    }

    /// ...
    pub fn write(
        &self,
        out: &mut dyn Write,
        format: Format,
        indent: usize,
    ) -> std::io::Result<()> {
        if !self.body.is_empty() {
            if indent > 0 && !format.is_compressed() {
                out.write_all(format.get_indent(indent)[1..].as_bytes())?;
            }
            if format.is_compressed() {
                write!(out, "{:#}{{", self.selectors)?;
            } else {
                write!(out, "{} {{", self.selectors)?;
            }

            let mut buf = Vec::new();
            for item in &self.body {
                buf.write_all(format.get_indent(indent + 2).as_bytes())?;
                item.write(&mut buf, format)?;
            }
            if format.is_compressed() && buf.last() == Some(&b';') {
                buf.pop();
            }
            out.write_all(&buf)?;
            out.write_all(format.get_indent(indent).as_bytes())?;
            out.write_all(if !format.is_compressed() {
                b"}\n"
            } else {
                b"}"
            })?;
        }
        Ok(())
    }
}

/// Something that may exist inside a rule.
pub enum BodyItem {
    /// A property declaration
    Property(String, Value),
    /// A comment
    Comment(String),
}

impl BodyItem {
    pub(crate) fn write(
        &self,
        out: &mut dyn Write,
        format: Format,
    ) -> std::io::Result<()> {
        match *self {
            BodyItem::Property(ref name, ref val) => write!(
                out,
                "{}:{}{};",
                name,
                if format.is_compressed() { "" } else { " " },
                val.format(format).to_string().replace('\n', " "),
            ),
            BodyItem::Comment(ref c) => write!(out, "/*{}*/", c),
        }
    }
}
