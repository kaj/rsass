//! Tests from `spec/parser/interpolate`

extern crate rsass;

mod interpolate {
    use rsass::{OutputStyle, compile_scss};

    mod t00_concatenation;
    mod t01_literal;
    mod t04_space_list_quoted;
    mod t06_space_list_complex;

    fn check(input: &str, expected: &str) {
        assert_eq!(compile_scss(input.as_bytes(), OutputStyle::Normal)
                       .and_then(|s| Ok(String::from_utf8(s)?))
                       .unwrap(),
                   expected);
    }
}
