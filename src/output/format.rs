use super::Style;

/// Specifies the format for outputing css.
///
/// The format is the style (expanded or compressed) and the precision
/// for numeric values.
#[derive(Clone, Copy, Debug)]
pub struct Format {
    pub style: Style,
    pub precision: usize,
}

impl Format {
    pub fn introspect() -> Self {
        let mut t = Self::default();
        t.style = Style::Introspection;
        t
    }
    pub fn is_compressed(&self) -> bool {
        self.style == Style::Compressed
    }
    pub fn is_introspection(&self) -> bool {
        self.style == Style::Introspection
    }
}

impl Default for Format {
    fn default() -> Format {
        Format {
            style: Style::Expanded,
            precision: 6,
        }
    }
}

/// A small container binding a value with an output format.
///
/// See e.g. [`css::Value::format`].
///
/// [`css::Value::format`]: ../css/enum.Value.html#method.format
pub struct Formatted<'a, T> {
    pub value: &'a T,
    pub format: Format,
}
