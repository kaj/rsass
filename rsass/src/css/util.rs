use super::Value;
use crate::output::{Format, Formatted};

pub fn is_not<'a, T>(value: &'a T, expected: &str) -> String
where
    Formatted<'a, T>: std::fmt::Display,
{
    format!(
        "{} is not {}.",
        Formatted {
            value,
            format: Format::introspect()
        },
        expected,
    )
}

#[derive(Debug)]
pub struct IsNot {
    got: Value,
    expected: &'static str,
}
impl IsNot {
    pub fn new(got: Value, expected: &'static str) -> Self {
        Self { got, expected }
    }
    pub fn value(&self) -> &Value {
        &self.got
    }
}

impl std::fmt::Display for IsNot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} is not {}.",
            Formatted {
                value: &self.got,
                format: Format::introspect()
            },
            self.expected,
        )
    }
}

// Needed for error propagation
impl From<IsNot> for String {
    fn from(value: IsNot) -> Self {
        value.to_string()
    }
}

/// Return true iff s is a valid _css_ function name.
pub fn is_function_name(s: &str) -> bool {
    is_calc_name(s) || s == "var"
}

/// Return true iff s is a valid _css_ calculation name.
///
/// That is, a css function name that is not `"var"`.
pub fn is_calc_name(s: &str) -> bool {
    s == "calc" || s == "clamp" || s == "max" || s == "min"
}
