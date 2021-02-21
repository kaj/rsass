use std::fmt;
use std::str::FromStr;

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

impl fmt::Display for Style {
    fn fmt(&self, out: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        out.write_str(match self {
            Style::Compressed => "compressed",
            Style::Expanded => "expanded",
            Style::Introspection => "introspection",
        })
    }
}

/// Get an output style from its name.
impl FromStr for Style {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_ref() {
            "compressed" => Ok(Style::Compressed),
            "expanded" => Ok(Style::Expanded),
            s => Err(format!("Output style {:?} not supported", s)),
        }
    }
}

static FORMAT_NAMES: [&str; 2] = ["Compressed", "Expanded"];

impl Style {
    /// Get the names of the supported output styles.
    pub fn variants() -> &'static [&'static str] {
        &FORMAT_NAMES
    }
}
