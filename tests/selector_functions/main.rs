//! Tests auto-converted from "sass-spec/spec/selector-functions"
//! version 0f59164a, 2019-02-01 17:21:13 -0800.
//! See <https://github.com/sass/sass-spec> for source material.\n
//! The following tests are excluded from conversion:
//! ["extend/nested", "extend/simple", "is_superselector", "parse", "replace", "unify/base", "unify/universal_simple"]
use rsass::{compile_scss, OutputStyle};

/// From "sass-spec/spec/selector-functions/append"
#[test]
fn append() {
    assert_eq!(
        rsass(
            "body {\n  a: selector-append(\".foo\", \".bar\", \".baz\");\n  b: selector-append(\".a .foo\", \".b .bar\");\n  c: selector-append(\".foo\", \"-suffix\");\n  d: selector-append(\'.foo\', \'.bar, -suffix\');\n  e: selector-append(\'.foo\', \'--suffix\');\n  f: selector-append(\'.foo\', \'.bar, --suffix\');\n}"
        )
        .unwrap(),
        "body {\n  a: .foo.bar.baz;\n  b: .a .foo.b .bar;\n  c: .foo-suffix;\n  d: .foo.bar, .foo-suffix;\n  e: .foo--suffix;\n  f: .foo.bar, .foo--suffix;\n}\n"
    );
}

mod extend;

// Ignoring "is_superselector", not expected to work yet.

/// From "sass-spec/spec/selector-functions/nest"
#[test]
fn nest() {
    assert_eq!(
        rsass(
            ".simple {\n  a: selector-nest(\'.foo\');\n  b: selector-nest(\'.foo\', \'.bar\');\n  c: selector-nest(\'.foo\', \'.bar\', \'.baz\');\n  d: selector-nest(\'.a .foo\', \'.b .bar\');\n  e: selector-nest(\'.foo\', \'&.bar\');\n  e: selector-nest(\'.foo\', \'&.bar\', \'.baz &\');\n}"
        )
        .unwrap(),
        ".simple {\n  a: .foo;\n  b: .foo .bar;\n  c: .foo .bar .baz;\n  d: .a .foo .b .bar;\n  e: .foo.bar;\n  e: .baz .foo.bar;\n}\n"
    );
}

// Ignoring "parse", not expected to work yet.

// Ignoring "replace", not expected to work yet.

mod unify;

fn rsass(input: &str) -> Result<String, String> {
    compile_scss(input.as_bytes(), OutputStyle::Expanded)
        .map_err(|e| format!("rsass failed: {}", e))
        .and_then(|s| String::from_utf8(s).map_err(|e| format!("{:?}", e)))
}
