//! Tests auto-converted from "sass-spec/spec/misc"
//! version 3a838875, 2018-09-19 16:03:37 -0400.
//! See <https://github.com/sass/sass-spec> for source material.\n
//! The following tests are excluded from conversion:
//! ["mixin_content", "negative_numbers", "unicode_variables", "JMA-pseudo-test", "trailing_comma_in_selector"]
extern crate rsass;
use rsass::{compile_scss, OutputStyle};

// Ignoring "JMA-pseudo-test", not expected to work yet

/// From "sass-spec/spec/misc/directive_interpolation"
#[test]
fn directive_interpolation() -> Result<(), String> {
    assert_eq!(
        rsass("$baz: 12;\n@foo bar#{$baz} qux {a: b}\n")?,
        "@foo bar12 qux {\n  a: b;\n}\n"
    );
    Ok(())
}

/// From "sass-spec/spec/misc/empty_content"
#[test]
fn empty_content() -> Result<(), String> {
    assert_eq!(
        rsass("@mixin foo { @content }\na { b: c; @include foo {} }\n")?,
        "a {\n  b: c;\n}\n"
    );
    Ok(())
}

// Ignoring "error-directive", tests with expected error not implemented yet.

/// From "sass-spec/spec/misc/import_in_mixin"
#[test]
fn import_in_mixin() -> Result<(), String> {
    assert_eq!(
        rsass(
            "@mixin import-google-fonts() {\n  @import url(\"http://fonts.googleapis.com/css?family=#{$family}\");\n}\n$family: unquote(\"Droid+Sans\");\n@include import-google-fonts();\n"
        )?,
        "@import url(\"http://fonts.googleapis.com/css?family=Droid+Sans\");\n"
    );
    Ok(())
}

/// From "sass-spec/spec/misc/import_with_interpolation"
#[test]
fn import_with_interpolation() -> Result<(), String> {
    assert_eq!(
        rsass(
            "$family: unquote(\"Droid+Sans\");\n@import url(\"http://fonts.googleapis.com/css?family=#{$family}\");\n"
        )?,
        "@import url(\"http://fonts.googleapis.com/css?family=Droid+Sans\");\n"
    );
    Ok(())
}

/// From "sass-spec/spec/misc/lang-bug"
#[test]
fn lang_bug() -> Result<(), String> {
    assert_eq!(
        rsass("div:lang(nb) {\n  color: red;\n}")?,
        "div:lang(nb) {\n  color: red;\n}\n"
    );
    Ok(())
}

/// From "sass-spec/spec/misc/media_interpolation"
#[test]
fn media_interpolation() -> Result<(), String> {
    assert_eq!(
        rsass("$baz: 12;\n@media bar#{$baz} {a {b: c}}\n")?,
        "@media bar12 {\n  a {\n    b: c;\n  }\n}\n"
    );
    Ok(())
}

// Ignoring "mixin_content", not expected to work yet

/// From "sass-spec/spec/misc/namespace_properties_with_script_value"
#[test]
fn namespace_properties_with_script_value() -> Result<(), String> {
    assert_eq!(
        rsass(
            "foo {\n  bar: baz + bang {\n    bip: bop;\n    bing: bop; }}\n"
        )?,
        "foo {\n  bar: bazbang;\n  bar-bip: bop;\n  bar-bing: bop;\n}\n"
    );
    Ok(())
}

// Ignoring "negative_numbers", not expected to work yet

/// From "sass-spec/spec/misc/selector_interpolation_before_element_name"
#[test]
fn selector_interpolation_before_element_name() -> Result<(), String> {
    assert_eq!(
        rsass("#{\"foo\" + \" bar\"}baz {a: b}\n")?,
        "foo barbaz {\n  a: b;\n}\n"
    );
    Ok(())
}

/// From "sass-spec/spec/misc/selector_only_interpolation"
#[test]
fn selector_only_interpolation() -> Result<(), String> {
    assert_eq!(
        rsass("#{\"foo\" + \" bar\"} {a: b}\n")?,
        "foo bar {\n  a: b;\n}\n"
    );
    Ok(())
}

// Ignoring "trailing_comma_in_selector", not expected to work yet

// Ignoring "unicode_variables", not expected to work yet

// Ignoring "warn-directive", tests with expected error not implemented yet.

fn rsass(input: &str) -> Result<String, String> {
    compile_scss(input.as_bytes(), OutputStyle::Expanded)
        .map_err(|e| format!("rsass failed: {}", e))
        .and_then(|s| String::from_utf8(s).map_err(|e| format!("{:?}", e)))
}
