//! Tests auto-converted from "sass-spec/spec/libsass-todo-issues/issue_1798/3.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "a  {\
            \n  content: \"#{ a /*#{\"}*/ }\";\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  content: \"a\";\
        \n}\
        \n"
    );
}
