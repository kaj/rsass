//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/198_test_extend_within_disparate_unknown_directive.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "@flooblehoof {.foo {a: b}}\
            \n@flooblehoof {.bar {@extend .foo}}\
            \n"
        )
        .unwrap(),
        "@flooblehoof {\
        \n  .foo, .bar {\
        \n    a: b;\
        \n  }\
        \n}\
        \n@flooblehoof {}\
        \n"
    );
}
