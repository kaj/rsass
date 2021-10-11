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
