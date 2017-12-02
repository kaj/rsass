//! Tests from `spec/parser/interpolate`

extern crate rsass;

mod interpolate {
    use rsass::{OutputStyle, compile_scss};

    mod t00_concatenation;
    mod t01_literal;
    mod t02_double_quoted;
    mod t03_single_quoted;
    mod t04_space_list_quoted;
    mod t06_space_list_complex;
    mod t10_escaped_backslash;
    mod t11_escaped_literal;
    mod t12_escaped_double_quoted;

    fn check(input: &str, expected: &str) {
        assert_eq!(compile_scss(input.as_bytes(), OutputStyle::Normal)
                       .and_then(|s| Ok(String::from_utf8(s)?))
                       .unwrap(),
                   expected);
    }
}
