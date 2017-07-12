/// The difference between a comma-separated and a
/// whitespace-separated list.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ListSeparator {
    Comma,
    Space,
}
