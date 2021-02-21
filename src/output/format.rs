use super::cssbuf::CssBuf;
use super::transform::handle_body;
use super::Style;
use crate::file_context::FileContext;
use crate::sass::Item;
use crate::variablescope::Scope;
use crate::Error;
use std::io::Write;

/// Specifies the format for outputing css.
///
/// The format is the style (expanded or compressed) and the precision
/// for numeric values.
#[derive(Clone, Copy, Debug)]
pub struct Format {
    /// The style of this format (expanded, compressed or introspection)
    pub style: Style,
    /// Number of decimals to use for numeric output.
    pub precision: usize,
}

impl Format {
    /// Create a format for introspection.
    pub fn introspect() -> Self {
        Self {
            style: Style::Introspection,
            ..Default::default()
        }
    }
    /// Return true if this is a compressed format.
    pub fn is_compressed(&self) -> bool {
        self.style == Style::Compressed
    }
    /// Return true if this is an introspection format.
    pub fn is_introspection(&self) -> bool {
        self.style == Style::Introspection
    }
    /// Write a slice of sass items in this format.
    /// The `file_context` is needed if there are `@import` statements
    /// in the sass file.
    pub fn write_root(
        &self,
        items: &[Item],
        globals: &mut dyn Scope,
        file_context: &impl FileContext,
    ) -> Result<Vec<u8>, Error> {
        let mut head = CssBuf::new(*self, 0);
        let mut body = CssBuf::new(*self, 0);
        handle_body(
            items,
            &mut head,
            None,
            &mut body,
            globals,
            file_context,
        )?;
        let mut result = vec![];
        let compressed = self.is_compressed();
        if !head.is_ascii() || !body.is_ascii() {
            if compressed {
                // U+FEFF is byte order mark, used to show encoding.
                result.extend_from_slice("\u{feff}".as_bytes());
            } else {
                result.extend_from_slice(b"@charset \"UTF-8\";\n");
            }
        }
        result.extend(head.buf);
        result.extend(body.buf);
        if compressed && result.last() == Some(&b';') {
            result.pop();
        }
        if result.last().unwrap_or(&b'\n') != &b'\n' {
            writeln!(&mut result)?;
        }
        Ok(result)
    }

    /// Get a newline followed by len spaces, unles self is compressed.
    pub fn get_indent(&self, len: usize) -> &'static str {
        static INDENT: &str = "\n                                                                                ";
        if self.is_compressed() {
            ""
        } else {
            &INDENT[..(len + 1)]
        }
    }
}

impl Default for Format {
    fn default() -> Format {
        Format {
            style: Style::Expanded,
            precision: 10,
        }
    }
}

/// A small container binding a value with an output format.
///
/// See e.g. [`css::Value::format`].
///
/// [`css::Value::format`]: ../css/enum.Value.html#method.format
pub struct Formatted<'a, T> {
    pub(crate) value: &'a T,
    pub(crate) format: Format,
}
