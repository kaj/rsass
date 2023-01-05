use crate::output::CssBuf;
use std::cmp::Ordering;

/// A comment in a css file.
#[derive(Clone, Debug)]
pub struct Comment(String);

impl<T: Into<String>> From<T> for Comment {
    fn from(t: T) -> Comment {
        Comment(t.into())
    }
}

impl Comment {
    /// Write this comment to a css output buffer.
    pub(crate) fn write(&self, buf: &mut CssBuf) {
        let indent = buf.indent_level();
        let existing = self
            .0
            .lines()
            .skip(1)
            .map(|s| {
                let i = s.bytes().take_while(|b| *b == b' ').count();
                if s.as_bytes().get(i).unwrap_or(&b'*') == &b'*' {
                    i
                } else {
                    i.saturating_sub(2)
                }
            })
            .min()
            .unwrap_or(indent);

        buf.do_indent_no_nl();
        buf.add_str("/*");
        match indent.cmp(&existing) {
            Ordering::Greater => {
                let start = buf.format().get_indent(indent - existing);
                buf.add_str(&self.0.replace('\n', start));
            }
            Ordering::Less => {
                let start = buf.format().get_indent(existing - indent - 1);
                buf.add_str(&self.0.replace(start, "\n"));
            }
            Ordering::Equal => {
                buf.add_str(&self.0);
            }
        }
        buf.add_one("*/\n", "*/");
    }
}
