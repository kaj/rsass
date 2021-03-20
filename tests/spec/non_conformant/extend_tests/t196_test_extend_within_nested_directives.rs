//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/196_test_extend_within_nested_directives.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "@media screen {\
            \n@flooblehoof {\
            \n  .foo {a: b}\
            \n  .bar {@extend .foo}\
            \n}\
            \n}\
            \n"
        )
        .unwrap(),
        "@media screen {\
        \n  @flooblehoof {\
        \n    .foo, .bar {\
        \n      a: b;\
        \n    }\
        \n  }\
        \n}\
        \n"
    );
}
