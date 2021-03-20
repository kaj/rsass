//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_690.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "test {\
            \n  left: expression(callSomeFunc());\
            \n  content: expression(\"Smile :-)\");\
            \n}\
            \n"
        )
        .unwrap(),
        "test {\
        \n  left: expression(callSomeFunc());\
        \n  content: expression(\"Smile :-)\");\
        \n}\
        \n"
    );
}
