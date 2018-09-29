//! Tests auto-converted from "sass-spec/spec/parser"
//! version df44d3aa, 2018-09-26 21:05:32 -0400.
//! See <https://github.com/sass/sass-spec> for source material.\n
//! The following tests are excluded from conversion:
//! ["interpolate/11_escaped_literal", "interpolate/14_escapes_literal_numbers", "interpolate/17_escapes_literal_lowercase", "interpolate/20_escapes_literal_uppercase", "interpolate/23_escapes_literal_specials", "interpolate/24_escapes_double_quoted_specials/todo_05_variable_quoted_double-4.0", "interpolate/25_escapes_single_quoted_specials/todo_05_variable_quoted_double-4.0", "operations/addition/dimensions/pairs-4.0", "operations/addition/numbers/pairs-4.0", "operations/addition/strings/pairs-4.0", "operations/binary-and-unary", "operations/division/dimensions/pairs-4.0", "operations/division/mixed/pairs-4.0", "operations/division/numbers/pairs-4.0", "operations/division/strings/pairs-4.0", "operations/subtract/dimensions/pairs-4.0", "operations/subtract/numbers/pairs-4.0", "operations/subtract/strings/pairs-4.0"]
extern crate rsass;
use rsass::{compile_scss, OutputStyle};

// Ignoring "and_and", tests with expected error not implemented yet.

mod arglists;

mod interpolate;

mod malformed_expressions;

mod operations;

fn rsass(input: &str) -> Result<String, String> {
    compile_scss(input.as_bytes(), OutputStyle::Expanded)
        .map_err(|e| format!("rsass failed: {}", e))
        .and_then(|s| String::from_utf8(s).map_err(|e| format!("{:?}", e)))
}
