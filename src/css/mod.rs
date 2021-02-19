//! Value type for css values.
mod call_args;
mod value;
mod valueformat;

pub use self::call_args::CallArgs;
pub use self::value::Value;

/// Something that may exist inside a rule.
pub enum BodyItem {
    /// A property declaration
    Property(String, Value),
    /// A commentx
    Comment(String),
}

use crate::output::Format;
use std::io::Write;

impl BodyItem {
    /// x
    pub fn write(
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
