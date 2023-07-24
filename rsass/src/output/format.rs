use super::Style;

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

    /// Get a newline followed by len spaces, unles self is compressed.
    pub fn get_indent(&self, len: usize) -> &'static str {
        static INDENT: &str = "\n                                                                                ";
        if self.is_compressed() {
            ""
        } else {
            &INDENT[..=len]
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
