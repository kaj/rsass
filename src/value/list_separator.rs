/// The difference between a comma-separated and a
/// whitespace-separated list.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ListSeparator {
    /// The list is comma-separated.
    Comma,
    /// The list is space-separated.
    Space,
}
