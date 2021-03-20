//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/197_test_extend_within_disparate_media.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "@media screen {.foo {a: b}}\
            \n@media screen {.bar {@extend .foo}}\
            \n"
        )
        .unwrap(),
        "@media screen {\
        \n  .foo, .bar {\
        \n    a: b;\
        \n  }\
        \n}\
        \n"
    );
}
