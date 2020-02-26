//! Tests auto-converted from "sass-spec/spec/parser"
//! version e9e219bdf, 2019-12-19 17:12:28 -0800.
//! See <https://github.com/sass/sass-spec> for source material.\n
use rsass::{compile_scss, OutputFormat};

// From "sass-spec/spec/parser/and_and.hrx"
#[test]
fn and_and() {
    assert_eq!(
        rsass(
            ".and-and {\
            \n  value: true && false;\
            \n}\
            \n"
        )
        .unwrap(),
        ".and-and {\
        \n  value: true .and-and .and-and false;\
        \n}\
        \n"
    );
}

mod arglists;

mod interpolate;

mod malformed_expressions;

mod operations;

fn rsass(input: &str) -> Result<String, String> {
    compile_scss(input.as_bytes(), OutputFormat::default())
        .map_err(|e| format!("rsass failed: {}", e))
        .and_then(|s| {
            String::from_utf8(s)
                .map(|s| s.replace("\n\n", "\n"))
                .map_err(|e| format!("{:?}", e))
        })
}
#[allow(unused)]
fn rsass_fmt(format: OutputFormat, input: &str) -> Result<String, String> {
    compile_scss(input.as_bytes(), format)
        .map_err(|e| format!("rsass failed: {}", e))
        .and_then(|s| {
            String::from_utf8(s)
                .map(|s| s.replace("\n\n", "\n"))
                .map_err(|e| format!("{:?}", e))
        })
}
