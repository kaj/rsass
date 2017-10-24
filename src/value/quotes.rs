/// A literal value can be double-quoted, single-quoted or not quoted.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Quotes {
    Double,
    Single,
    None,
}
