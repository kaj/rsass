/// Selected target format.
/// Only formats that are variants of this type are supported by rsass.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Style {
    /// The expanded format, nice readable css.
    Expanded,
    /// The compressed format, saves download size.
    Compressed,
    /// Special format used by the inspect(value) sass function
    Introspection,
}
