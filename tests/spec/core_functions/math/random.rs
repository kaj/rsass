//! Tests auto-converted from "sass-spec/spec/core_functions/math/random.hrx"

mod error {

    // Ignoring "decimal", error tests are not supported yet.

    // Ignoring "negative", error tests are not supported yet.

    // Ignoring "test_type", error tests are not supported yet.

    // Ignoring "zero", error tests are not supported yet.
}
#[test]
fn ignores_units() {
    assert_eq!(
        crate::rsass(
            "a {b: random(1px)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1;\
        \n}\
        \n"
    );
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "$value: random($limit: 10);\
            \na {b: $value > 0 and $value <= 10}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: true;\
        \n}\
        \n"
    );
}
#[test]
fn no_arg() {
    assert_eq!(
        crate::rsass(
            "$value: random();\
            \na {b: $value >= 0 and $value < 1}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: true;\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // unexepected error
fn null() {
    assert_eq!(
        crate::rsass(
            "@import \"../util\";\
            \n@function check($value) {@return $value >= 0 and $value < 1}\
            \n@include check-values(null, get-function(check));\
            \n"
        )
        .unwrap(),
        ""
    );
}
#[test]
#[ignore] // unexepected error
fn one() {
    assert_eq!(
        crate::rsass(
            "@import \"../util\";\
            \n@function check($value) {@return $value == 1}\
            \n@include check-values(1, get-function(check));\
            \n"
        )
        .unwrap(),
        ""
    );
}
#[test]
#[ignore] // unexepected error
fn one_hundred() {
    assert_eq!(
        crate::rsass(
            "@import \"../util\";\
            \n@function check($value) {@return $value == round($value) and $value > 0 and $value <= 100}\
            \n@include check-values(100, get-function(check));\
            \n"
        )
        .unwrap(),
        ""
    );
}
#[test]
#[ignore] // unexepected error
fn two() {
    assert_eq!(
        crate::rsass(
            "@import \"../util\";\
            \n@function check($value) {@return $value == 1 or $value == 2}\
            \n@include check-values(2, get-function(check));\
            \n"
        )
        .unwrap(),
        ""
    );
}
#[test]
fn within_precision() {
    assert_eq!(
        crate::rsass(
            "// This is within the precision limit to be considered identical to 1.\
            \na {b: random(1.0000000000001)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1;\
        \n}\
        \n"
    );
}
