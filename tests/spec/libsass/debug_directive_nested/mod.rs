//! Tests auto-converted from "sass-spec/spec/libsass/debug-directive-nested"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/libsass/debug-directive-nested/function.hrx"
#[test]
fn function() {
    assert_eq!(
        rsass(
            "@function c() {\
            \n  @warn test;\
            \n  @return d;\
            \n}\
            \n\
            \na {\
            \n  b: {\
            \n    c: c();\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b-c: d;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass/debug-directive-nested/inline.hrx"
#[test]
#[ignore] // wrong result
fn inline() {
    assert_eq!(
        rsass(
            "a {\
            \n  b: {\
            \n    @debug test;\
            \n    c: d;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b-c: d;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass/debug-directive-nested/mixin.hrx"
#[test]
fn mixin() {
    assert_eq!(
        rsass(
            "@mixin c() {\
            \n  @warn test;\
            \n  c: d;\
            \n}\
            \n\
            \na {\
            \n  b: {\
            \n    @include c();\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b-c: d;\
        \n}\
        \n"
    );
}
