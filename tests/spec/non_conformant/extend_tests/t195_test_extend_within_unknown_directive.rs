//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/195_test_extend_within_unknown_directive.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "@flooblehoof {\
            \n.foo {a: b}\
            \n.bar {@extend .foo}\
            \n}\
            \n"
        )
        .unwrap(),
        "@flooblehoof {\
        \n  .foo, .bar {\
        \n    a: b;\
        \n  }\
        \n}\
        \n"
    );
}
