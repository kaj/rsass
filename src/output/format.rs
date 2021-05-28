use super::cssbuf::{CssBuf, CssHead};
use super::transform::handle_body;
use super::Style;
use crate::file_context::FileContext;
use crate::sass::Item;
use crate::{Error, ScopeRef};

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
        globals: ScopeRef,
        file_context: &impl FileContext,
    ) -> Result<Vec<u8>, Error> {
        let mut head = CssHead::new(*self);
        let mut body = CssBuf::new(*self);
        handle_body(
            items,
            &mut head,
            None,
            &mut body,
            globals,
            file_context,
        )?;
        Ok(head.combine_final(body))
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
