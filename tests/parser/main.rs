//! Tests auto-converted from "sass-spec/spec/parser"
//! version 5717844f, 2019-01-28 20:42:33 -0500.
//! See <https://github.com/sass/sass-spec> for source material.\n
//! The following tests are excluded from conversion:
//! ["interpolate/11_escaped_literal", "interpolate/12_escaped_double_quoted/06_escape_interpolation", "interpolate/13_escaped_single_quoted/06_escape_interpolation", "interpolate/14_escapes_literal_numbers", "interpolate/15_escapes_double_quoted_numbers/06_escape_interpolation", "interpolate/16_escapes_single_quoted_numbers/06_escape_interpolation", "interpolate/17_escapes_literal_lowercase", "interpolate/18_escapes_double_quoted_lowercase/06_escape_interpolation", "interpolate/19_escapes_single_quoted_lowercase/06_escape_interpolation", "interpolate/20_escapes_literal_uppercase", "interpolate/21_escapes_double_quoted_uppercase/06_escape_interpolation", "interpolate/22_escapes_single_quoted_uppercase/06_escape_interpolation", "interpolate/23_escapes_literal_specials", "interpolate/24_escapes_double_quoted_specials/todo_05_variable_quoted_double-4.0", "interpolate/24_escapes_double_quoted_specials/06_escape_interpolation", "interpolate/25_escapes_single_quoted_specials/todo_05_variable_quoted_double-4.0", "interpolate/25_escapes_single_quoted_specials/06_escape_interpolation", "operations/binary-and-unary"]
extern crate rsass;
use rsass::{compile_scss, OutputStyle};

/// From "sass-spec/spec/parser/and_and"
#[test]
fn and_and() {
    assert_eq!(
        rsass(".and-and {\n  value: true && false;\n}\n").unwrap(),
        ".and-and {\n  value: true .and-and .and-and false;\n}\n"
    );
}

mod arglists;

mod interpolate;

mod malformed_expressions;

mod operations;

fn rsass(input: &str) -> Result<String, String> {
    compile_scss(input.as_bytes(), OutputStyle::Expanded)
        .map_err(|e| format!("rsass failed: {}", e))
        .and_then(|s| String::from_utf8(s).map_err(|e| format!("{:?}", e)))
}
