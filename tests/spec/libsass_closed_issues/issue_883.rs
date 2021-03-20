//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_883.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "div {\
            \n  @foo {\
            \n    font: a;\
            \n  }\
            \n  @bar {\
            \n    color: b;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "@foo {\
        \n  div {\
        \n    font: a;\
        \n  }\
        \n}\
        \n@bar {\
        \n  div {\
        \n    color: b;\
        \n  }\
        \n}\
        \n"
    );
}
