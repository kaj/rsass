//! Tests auto-converted from "sass-spec/spec/css"
//! version ac22fe99, 2019-01-09 15:50:06 -0500.
//! See <https://github.com/sass/sass-spec> for source material.\n
//! The following tests are excluded from conversion:
//! ["bizarrely_formatted_comments", "custom_properties", "moz_document", "ms_long_filter_syntax", "multi_star_comments", "plain", "selector/slotted", "unknown_directive"]
extern crate rsass;
use rsass::{compile_scss, OutputStyle};

// Ignoring "bizarrely_formatted_comments", not expected to work yet.

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

// Ignoring "custom_properties", not expected to work yet.

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

// Ignoring "moz_document", not expected to work yet.

// Ignoring "ms_long_filter_syntax", not expected to work yet.

// Ignoring "multi_star_comments", not expected to work yet.

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

// Ignoring "unknown_directive", not expected to work yet.

fn rsass(input: &str) -> Result<String, String> {
    compile_scss(input.as_bytes(), OutputStyle::Expanded)
        .map_err(|e| format!("rsass failed: {}", e))
        .and_then(|s| String::from_utf8(s).map_err(|e| format!("{:?}", e)))
}
