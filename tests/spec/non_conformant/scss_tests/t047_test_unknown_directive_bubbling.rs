//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/047_test_unknown_directive_bubbling.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo {\
            \n  @fblthp {\
            \n    .bar {a: b}\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "@fblthp {\
        \n  .foo .bar {\
        \n    a: b;\
        \n  }\
        \n}\
        \n"
    );
}
