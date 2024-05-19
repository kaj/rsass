/// The difference between a comma-separated and a
/// whitespace-separated list.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ListSeparator {
    /// The list is space-separated.
    Space,
    /// The list is slash-separated.
    Slash,
    /// The list is slash-separated without whitespace.
    SlashNoSpace,
    /// The list is comma-separated.
    Comma,
}

impl ListSeparator {
    /// Get the actutual separator string.
    pub fn sep(&self, compressed: bool) -> &'static str {
        match self {
            Self::Comma if compressed => ",",
            Self::Comma => ", ",
            Self::Slash if compressed => "/",
            Self::SlashNoSpace => "/",
            Self::Slash => " / ",
            Self::Space => " ",
        }
    }
}

impl Default for ListSeparator {
    fn default() -> Self {
        Self::Space
    }
}

#[test]
fn check_sep_order() {
    assert!(ListSeparator::Comma > ListSeparator::Space);
    assert!(ListSeparator::Slash > ListSeparator::Space);
}
