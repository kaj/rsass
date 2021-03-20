//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1441/child.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".child {\
            \n    & > & {\
            \n        foo: bar;\
            \n    }\
            \n}\
            \n"
        )
        .unwrap(),
        ".child > .child {\
        \n  foo: bar;\
        \n}\
        \n"
    );
}
