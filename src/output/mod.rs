//! Types describing how to format output.
mod cssbuf;
mod format;
mod style;
mod transform;

pub(crate) use cssbuf::CssBuf;
pub use format::{Format, Formatted};
pub use style::Style;
