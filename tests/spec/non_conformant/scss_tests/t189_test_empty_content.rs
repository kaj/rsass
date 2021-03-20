//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/189_test_empty_content.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
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
