//! Tests from spec/scss
extern crate rsass;
use rsass::{OutputStyle, compile_scss};


#[test]
fn important() {
    check(b"div {\n  color: red ! important;\n  width: 5px ! important;\n}",
          "div {\n  color: red !important;\n  width: 5px !important;\n}\n")
}

/// My own addition
#[test]
fn important_compact_input() {
    check(b"div {z-index: 40!important}",
          "div {\n  z-index: 40 !important;\n}\n")
}

fn check(input: &[u8], expected: &str) {
    assert_eq!(compile_scss(input, OutputStyle::Normal).and_then(|s| {
                   String::from_utf8(s)
                       .map_err(|e| format!("Non-utf8 output: {}", e))
               }),
               Ok(expected.into()));
}
