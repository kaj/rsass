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
            ListSeparator::Comma if compressed => ",",
            ListSeparator::Comma => ", ",
            ListSeparator::Slash if compressed => "/",
            ListSeparator::SlashNoSpace => "/",
            ListSeparator::Slash => " / ",
            ListSeparator::Space => " ",
        }
    }
}

impl Default for ListSeparator {
    fn default() -> Self {
        ListSeparator::Space
    }
}

#[test]
fn check_sep_order() {
    assert!(ListSeparator::Comma > ListSeparator::Space);
    assert!(ListSeparator::Slash > ListSeparator::Space);
}
