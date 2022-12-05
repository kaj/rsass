//! Types describing how to format output.
mod cssbuf;
mod cssdata;
mod cssdest;
mod format;
mod style;
mod transform;

pub use format::{Format, Formatted};
pub use style::Style;

pub(crate) use cssbuf::CssBuf;
pub(crate) use cssdata::CssData;
pub(crate) use transform::handle_parsed;
