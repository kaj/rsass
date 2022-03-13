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

/// Return true iff s is a valid _css_ function name.
pub fn is_function_name(s: &str) -> bool {
    s == "calc" || s == "clamp" || s == "max" || s == "min" || s == "var"
}
