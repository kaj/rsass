//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1243/include.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin foo() {\
            \n  a { b: c; }\
            \n}\
            \n@include foo\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: c;\
        \n}\
        \n"
    );
}
