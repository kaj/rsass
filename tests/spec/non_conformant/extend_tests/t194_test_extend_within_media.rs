//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/194_test_extend_within_media.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "@media screen {\
            \n.foo {a: b}\
            \n.bar {@extend .foo}\
            \n}\
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
