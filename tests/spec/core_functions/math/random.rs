//! Tests auto-converted from "sass-spec/spec/core_functions/math/random.hrx"

mod error {
    #[test]
    fn decimal() {
        assert_eq!(
            crate::rsass(
                "a {b: random(1.5)}\
             \n"
            )
            .unwrap_err(),
            "Error: $limit: 1.5 is not an int.\
         \n  ,\
         \n1 | a {b: random(1.5)}\
         \n  |       ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    fn negative() {
        assert_eq!(
            crate::rsass(
                "a {b: random(-1)}\
             \n"
            )
            .unwrap_err(),
            "Error: $limit: Must be greater than 0, was -1.\
         \n  ,\
         \n1 | a {b: random(-1)}\
         \n  |       ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            crate::rsass(
                "a {b: random(c)}\
             \n"
            )
            .unwrap_err(),
            "Error: $limit: c is not a number.\
         \n  ,\
         \n1 | a {b: random(c)}\
         \n  |       ^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    fn zero() {
        assert_eq!(
            crate::rsass(
                "a {b: random(0)}\
             \n"
            )
            .unwrap_err(),
            "Error: $limit: Must be greater than 0, was 0.\
         \n  ,\
         \n1 | a {b: random(0)}\
         \n  |       ^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
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
