//! Tests auto-converted from "sass-spec/spec/values"
//! version 0f59164a, 2019-02-01 17:21:13 -0800.
//! See <https://github.com/sass/sass-spec> for source material.\n
//! The following tests are excluded from conversion:
//! ["ids", "numbers/units/multiple"]
extern crate rsass;
use rsass::{compile_scss, OutputStyle};

mod colors;

mod identifiers;

// Ignoring "ids", not expected to work yet.

mod lists;

mod maps;

mod numbers;

fn rsass(input: &str) -> Result<String, String> {
    compile_scss(input.as_bytes(), OutputStyle::Expanded)
        .map_err(|e| format!("rsass failed: {}", e))
        .and_then(|s| String::from_utf8(s).map_err(|e| format!("{:?}", e)))
}
