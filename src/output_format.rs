use crate::output_style::OutputStyle;

#[derive(Clone, Copy, Debug)]
pub struct OutputFormat {
    pub style: OutputStyle,
    pub precision: usize,
}

impl OutputFormat {
    pub fn is_compressed(&self) -> bool {
        self.style == OutputStyle::Compressed
    }
}

impl Default for OutputFormat {
    fn default() -> OutputFormat {
        OutputFormat {
            style: OutputStyle::Expanded,
            precision: 6,
        }
    }
}

pub struct Formatted<'a, T> {
    pub value: &'a T,
    pub format: OutputFormat,
}
