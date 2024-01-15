use crate::css::CssString;

/// A logical attribute selector.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Attribute {
    /// The attribute name
    name: String,
    /// An operator
    op: String,
    /// A value to match.
    val: CssString,
    /// Optional modifier.
    modifier: Option<char>,
}

impl Attribute {
    pub fn new(
        name: &CssString,
        op: &str,
        val: &CssString,
        modifier: Option<char>,
    ) -> Self {
        Attribute {
            name: name.value().into(),
            op: op.into(),
            val: val.clone(),
            modifier,
        }
    }
    pub(super) fn is_superselector(&self, b: &Self) -> bool {
        self.name == b.name
            && self.op == b.op
            && self.val == b.val
            && self.modifier == b.modifier
    }

    pub(super) fn write_to_buf(&self, buf: &mut String) {
        use std::fmt::Write;
        buf.push('[');
        write!(buf, "{}{}{}", self.name, self.op, self.val).unwrap();
        if let Some(m) = self.modifier {
            buf.push(' ');
            buf.push(m);
        }
        buf.push(']');
    }
}
