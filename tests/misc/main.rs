//! Tests auto-converted from "sass-spec/spec/misc"
//! version 3653ded68, 2019-11-15 15:45:05 -0800.
//! See <https://github.com/sass/sass-spec> for source material.\n
use rsass::{compile_scss, OutputStyle};

// From "sass-spec/spec/misc/JMA-pseudo-test.hrx"
#[test]
#[ignore] // wrong result
fn jma_pseudo_test() {
    assert_eq!(
        rsass(
            ".foo {\
             \n        h1 {\
             \n                color:red;\
             \n        }\
             \n}\
             \n\
             \n.bar {\
             \n        &:hover h3,\
             \n        h3 {\
             \n                @extend h1;\
             \n        }\
             \n}\
             \n"
        )
        .unwrap(),
        ".foo h1,\
         \n.foo .bar h3,\
         \n.bar .foo h3 {\
         \n  color: red;\
         \n}\
         \n"
    );
}

// From "sass-spec/spec/misc/directive_interpolation.hrx"
#[test]
fn directive_interpolation() {
    assert_eq!(
        rsass(
            "$baz: 12;\
             \n@foo bar#{$baz} qux {a: b}\
             \n"
        )
        .unwrap(),
        "@foo bar12 qux {\
         \n  a: b;\
         \n}\
         \n"
    );
}

// From "sass-spec/spec/misc/empty_content.hrx"
#[test]
fn empty_content() {
    assert_eq!(
        rsass(
            "@mixin foo { @content }\
             \na { b: c; @include foo {} }\
             \n"
        )
        .unwrap(),
        "a {\
         \n  b: c;\
         \n}\
         \n"
    );
}

// From "sass-spec/spec/misc/error-directive.hrx"

// Ignoring "error_directive", error tests are not supported yet.

// From "sass-spec/spec/misc/import_in_mixin.hrx"
#[test]
fn import_in_mixin() {
    assert_eq!(
        rsass(
            "@mixin import-google-fonts() {\
            \n  @import url(\"http://fonts.googleapis.com/css?family=#{$family}\");\
            \n}\
            \n$family: unquote(\"Droid+Sans\");\
            \n@include import-google-fonts();\
            \n"
        )
        .unwrap(),
        "@import url(\"http://fonts.googleapis.com/css?family=Droid+Sans\");\
        \n"
    );
}

// From "sass-spec/spec/misc/import_with_interpolation.hrx"
#[test]
fn import_with_interpolation() {
    assert_eq!(
        rsass(
            "$family: unquote(\"Droid+Sans\");\
            \n@import url(\"http://fonts.googleapis.com/css?family=#{$family}\");\
            \n"
        )
        .unwrap(),
        "@import url(\"http://fonts.googleapis.com/css?family=Droid+Sans\");\
        \n"
    );
}

// From "sass-spec/spec/misc/lang-bug.hrx"
#[test]
fn lang_bug() {
    assert_eq!(
        rsass(
            "div:lang(nb) {\
             \n  color: red;\
             \n}"
        )
        .unwrap(),
        "div:lang(nb) {\
         \n  color: red;\
         \n}\
         \n"
    );
}

// From "sass-spec/spec/misc/media_interpolation.hrx"
#[test]
fn media_interpolation() {
    assert_eq!(
        rsass(
            "$baz: 12;\
             \n@media bar#{$baz} {a {b: c}}\
             \n"
        )
        .unwrap(),
        "@media bar12 {\
         \n  a {\
         \n    b: c;\
         \n  }\
         \n}\
         \n"
    );
}

// From "sass-spec/spec/misc/mixin_content.hrx"
#[test]
#[ignore] // wrong result
fn mixin_content() {
    assert_eq!(
        rsass(
            "$color: blue;\
             \n@mixin context($class, $color: red) {\
             \n  .#{$class} {\
             \n    background-color: $color;\
             \n    @content;\
             \n    border-color: $color;\
             \n  }\
             \n}\
             \n@include context(parent) {\
             \n  @include context(child, $color: yellow) {\
             \n    color: $color;\
             \n  }\
             \n}\
             \n"
        )
        .unwrap(),
        ".parent {\
         \n  background-color: red;\
         \n  border-color: red;\
         \n}\
         \n.parent .child {\
         \n  background-color: yellow;\
         \n  color: blue;\
         \n  border-color: yellow;\
         \n}\
         \n"
    );
}

// From "sass-spec/spec/misc/namespace_properties_with_script_value.hrx"
#[test]
fn namespace_properties_with_script_value() {
    assert_eq!(
        rsass(
            "foo {\
             \n  bar: baz + bang {\
             \n    bip: bop;\
             \n    bing: bop; }}\
             \n"
        )
        .unwrap(),
        "foo {\
         \n  bar: bazbang;\
         \n  bar-bip: bop;\
         \n  bar-bing: bop;\
         \n}\
         \n"
    );
}

// From "sass-spec/spec/misc/negative_numbers.hrx"
#[test]
#[ignore] // wrong result
fn negative_numbers() {
    assert_eq!(
        rsass(
            "$zero: 0;\
             \na {\
             \n  zero: -$zero;\
             \n  zero: $zero * -1;\
             \n}\
             \n$near: 0.000000000001;\
             \na {\
             \n  near: -$near;\
             \n  near: $near * -1;\
             \n}\
             \n"
        )
        .unwrap(),
        "a {\
         \n  zero: 0;\
         \n  zero: 0;\
         \n}\
         \na {\
         \n  near: 0;\
         \n  near: 0;\
         \n}\
         \n"
    );
}

// From "sass-spec/spec/misc/selector_interpolation_before_element_name.hrx"
#[test]
fn selector_interpolation_before_element_name() {
    assert_eq!(
        rsass(
            "#{\"foo\" + \" bar\"}baz {a: b}\
             \n"
        )
        .unwrap(),
        "foo barbaz {\
         \n  a: b;\
         \n}\
         \n"
    );
}

// From "sass-spec/spec/misc/selector_only_interpolation.hrx"
#[test]
fn selector_only_interpolation() {
    assert_eq!(
        rsass(
            "#{\"foo\" + \" bar\"} {a: b}\
             \n"
        )
        .unwrap(),
        "foo bar {\
         \n  a: b;\
         \n}\
         \n"
    );
}

// From "sass-spec/spec/misc/trailing_comma_in_selector.hrx"
#[test]
#[ignore] // wrong result
fn trailing_comma_in_selector() {
    assert_eq!(
        rsass(
            "#foo #bar,,\
             \n,#baz #boom, {a: b}\
             \n\
             \n#bip #bop, ,, {c: d}\
             \n"
        )
        .unwrap(),
        "#foo #bar,\
         \n#baz #boom {\
         \n  a: b;\
         \n}\
         \n#bip #bop {\
         \n  c: d;\
         \n}\
         \n"
    );
}

// From "sass-spec/spec/misc/unicode_variables"
#[test]
fn unicode_variables() {
    assert_eq!(
        rsass(
            "$vär: foo;\
             \n\
             \nblat {a: $vär}\
             \n"
        )
        .unwrap(),
        "blat {\
         \n  a: foo;\
         \n}\
         \n"
    );
}

// From "sass-spec/spec/misc/warn-directive.hrx"
#[test]
#[ignore] // wrong result
fn warn_directive() {
    assert_eq!(
        rsass(
            "h1 { color: blue; } \
             \n@warn \"Don\'t crash the ambulance, whatever you do\"\
             \n"
        )
        .unwrap(),
        "h1 {\
         \n  color: blue;\
         \n}\
         \n"
    );
}

fn rsass(input: &str) -> Result<String, String> {
    compile_scss(input.as_bytes(), OutputStyle::Expanded)
        .map_err(|e| format!("rsass failed: {}", e))
        .and_then(|s| {
            String::from_utf8(s)
                .map(|s| s.replace("\n\n", "\n"))
                .map_err(|e| format!("{:?}", e))
        })
}
