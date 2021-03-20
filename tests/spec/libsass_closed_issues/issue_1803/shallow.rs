//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1803/shallow.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "a {\
            \n  display: block\
            \n\
            \n  b {\
            \n    foo: bar;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  display: block b;\
        \n  display-foo: bar;\
        \n}\
        \n"
    );
}
