use std::str::from_utf8;

/// A position in sass input.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct SourcePos {
    pub line: String,
    pub line_no: u32,
    pub line_pos: usize,
    pub file: SourceName,
}

impl SourcePos {
    pub fn pos_of(part: &[u8], whole: &[u8]) -> SourcePos {
        let index = whole.len() - part.len();
        let before = &whole[0..index];
        let line_start = before.rsplit(|c| *c == b'\n').next().unwrap();
        let line_end = part.split(|c| *c == b'\n').next().unwrap();
        let line_bytes =
            &whole[index - line_start.len()..index + line_end.len()];
        SourcePos {
            line: from_utf8(line_bytes)
                .unwrap_or("<<failed to display line>>")
                .to_string(),
            line_no: 1 + bytecount::count(before, b'\n') as u32,
            line_pos: bytecount::num_chars(line_start),
            file: SourceName::root("-"),
        }
    }
}

/// The name of a scss source file.
///
/// Currently this is just a String.
/// In a future version it should also contain the information if this
/// was the root stylesheet or where it was imported from.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct SourceName {
    name: String,
}

impl SourceName {
    pub fn root<T: ToString>(name: T) -> Self {
        SourceName {
            name: name.to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
