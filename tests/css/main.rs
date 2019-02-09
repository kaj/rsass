//! Tests auto-converted from "sass-spec/spec/css"
//! version dd3a5edf, 2019-02-04 13:14:26 -0800.
//! See <https://github.com/sass/sass-spec> for source material.\n
//! The following tests are excluded from conversion:
//! ["plain"]
use rsass::{compile_scss, OutputStyle};

/// From "sass-spec/spec/css/bizarrely_formatted_comments"
#[test]
#[ignore] // failing
fn bizarrely_formatted_comments() {
    assert_eq!(
        rsass(".foo {\n    /* Foo\n Bar\nBaz */\n  a: b; }\n").unwrap(),
        ".foo {\n  /* Foo\n   Bar\n  Baz */\n  a: b;\n}\n"
    );
}

/// From "sass-spec/spec/css/blockless_directive_without_semicolon"
#[test]
fn blockless_directive_without_semicolon() {
    assert_eq!(rsass("@foo \"bar\";\n").unwrap(), "@foo \"bar\";\n");
}

/// From "sass-spec/spec/css/closing_line_comment_end_with_compact_output"
#[test]
fn closing_line_comment_end_with_compact_output() {
    assert_eq!(
        rsass("/* foo */\nbar { baz: bang; }\n").unwrap(),
        "/* foo */\nbar {\n  baz: bang;\n}\n"
    );
}

mod custom_properties;

/// From "sass-spec/spec/css/directive_with_lots_of_whitespace"
#[test]
fn directive_with_lots_of_whitespace() {
    assert_eq!(rsass("@foo \"bar\";\n").unwrap(), "@foo \"bar\";\n");
}

/// From "sass-spec/spec/css/empty_block_directive"
#[test]
fn empty_block_directive() {
    assert_eq!(rsass("@foo {}\n").unwrap(), "@foo {}\n");
}

/// From "sass-spec/spec/css/function_name_identifiers"
#[test]
fn function_name_identifiers() {
    assert_eq!(
        rsass(
            "a {\n  b: url;\n  c: calc;\n  d: element;\n  e: expression;\n  f: progid;\n}\n"
        )
        .unwrap(),
        "a {\n  b: url;\n  c: calc;\n  d: element;\n  e: expression;\n  f: progid;\n}\n"
    );
}

mod media;

// Ignoring "min_max", start_version is 3.7.

mod moz_document;

/// From "sass-spec/spec/css/ms_long_filter_syntax"
#[test]
#[ignore] // failing
fn ms_long_filter_syntax() {
    assert_eq!(
        rsass(
            "foo {\n  filter: progid:DXImageTransform.Microsoft.gradient(GradientType=1, startColorstr=#c0ff3300, endColorstr=#ff000000);\n  filter: progid:DXImageTransform.Microsoft.gradient(GradientType=1, startColorstr=#c0ff3300, endColorstr=#ff000000); }\n"
        )
        .unwrap(),
        "foo {\n  filter: progid:DXImageTransform.Microsoft.gradient(GradientType=1, startColorstr=#c0ff3300, endColorstr=#ff000000);\n  filter: progid:DXImageTransform.Microsoft.gradient(GradientType=1, startColorstr=#c0ff3300, endColorstr=#ff000000);\n}\n"
    );
}

/// From "sass-spec/spec/css/multi_star_comments"
#[test]
fn multi_star_comments() {
    assert_eq!(
        rsass(
            "a /***/ b {x: y}\na /****/ b {x: y}\na /* **/ b {x: y}\na /** */ b {x: y}\n"
        )
        .unwrap(),
        "a b {\n  x: y;\n}\na b {\n  x: y;\n}\na b {\n  x: y;\n}\na b {\n  x: y;\n}\n"
    );
}

/// From "sass-spec/spec/css/multiple_comments"
#[test]
fn multiple_comments() {
    assert_eq!(
        rsass(".foo {\n  /* Foo Bar */\n  /* Baz Bang */ }\n").unwrap(),
        ".foo {\n  /* Foo Bar */\n  /* Baz Bang */\n}\n"
    );
}

// Ignoring "plain", not expected to work yet.

mod selector;

mod unicode_range;

mod unknown_directive;

fn rsass(input: &str) -> Result<String, String> {
    compile_scss(input.as_bytes(), OutputStyle::Expanded)
        .map_err(|e| format!("rsass failed: {}", e))
        .and_then(|s| {
            String::from_utf8(s)
                .map(|s| s.replace("\n\n", "\n"))
                .map_err(|e| format!("{:?}", e))
        })
}
