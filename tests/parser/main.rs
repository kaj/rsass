//! Tests auto-converted from "sass-spec/spec/parser"
//! version 6c9bd98c, 2019-02-13 14:56:19 -0800.
//! See <https://github.com/sass/sass-spec> for source material.\n
use rsass::{compile_scss, OutputStyle};

/// From "sass-spec/spec/parser/and_and.hrx"
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
        .and_then(|s| {
            String::from_utf8(s)
                .map(|s| s.replace("\n\n", "\n"))
                .map_err(|e| format!("{:?}", e))
        })
}
