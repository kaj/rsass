use std::fmt::{self, Display, Write};

pub struct WriteStr<'a>(pub &'a str, pub u8);

impl Display for WriteStr<'_> {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        out.write_char('"')?;
        let mut parts = self.0.split('\n');
        if let Some(part) = parts.next() {
            write!(out, "{}", part.escape_debug())?;
        }
        for part in parts {
            if part.is_empty() {
                out.write_str("\\n")?;
            } else {
                write!(
                    out,
                    "\\\n{:1$}\\n{2}",
                    "",
                    self.1 as usize,
                    part.escape_debug()
                )?;
            }
        }
        out.write_char('"')
    }
}
