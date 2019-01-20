//! Tests auto-converted from "sass-spec/spec/parser"
//! version ac22fe99, 2019-01-09 15:50:06 -0500.
//! See <https://github.com/sass/sass-spec> for source material.\n
//! The following tests are excluded from conversion:
//! ["interpolate/00_concatenation/unspaced", "interpolate/11_escaped_literal", "interpolate/12_escaped_double_quoted/06_escape_interpolation", "interpolate/13_escaped_single_quoted/06_escape_interpolation", "interpolate/14_escapes_literal_numbers", "interpolate/15_escapes_double_quoted_numbers/06_escape_interpolation", "interpolate/16_escapes_single_quoted_numbers/06_escape_interpolation", "interpolate/17_escapes_literal_lowercase", "interpolate/18_escapes_double_quoted_lowercase/06_escape_interpolation", "interpolate/19_escapes_single_quoted_lowercase/06_escape_interpolation", "interpolate/20_escapes_literal_uppercase", "interpolate/21_escapes_double_quoted_uppercase/06_escape_interpolation", "interpolate/22_escapes_single_quoted_uppercase/06_escape_interpolation", "interpolate/23_escapes_literal_specials", "interpolate/24_escapes_double_quoted_specials/todo_05_variable_quoted_double-4.0", "interpolate/24_escapes_double_quoted_specials/06_escape_interpolation", "interpolate/25_escapes_single_quoted_specials/todo_05_variable_quoted_double-4.0", "interpolate/25_escapes_single_quoted_specials/06_escape_interpolation", "operations/addition", "operations/binary-and-unary", "operations/division", "operations/logic_eq/dimensions/pairs", "operations/logic_eq/mixed/pairs", "operations/logic_eq/numbers/pairs", "operations/logic_eq/strings/pairs", "operations/logic_ge/numbers/pairs", "operations/logic_ge/strings/pairs", "operations/logic_gt/numbers/pairs", "operations/logic_gt/strings/pairs", "operations/logic_le/numbers/pairs", "operations/logic_le/strings/pairs", "operations/logic_lt/numbers/pairs", "operations/logic_lt/strings/pairs", "operations/logic_ne/dimensions/pairs", "operations/logic_ne/mixed/pairs", "operations/logic_ne/numbers/pairs", "operations/logic_ne/strings/pairs", "operations/modulo/numbers/pairs", "operations/multiply/numbers/pairs", "operations/multiply/strings/pairs", "operations/subtract"]
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
