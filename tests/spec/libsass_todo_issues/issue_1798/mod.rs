//! Tests auto-converted from "sass-spec/spec/libsass-todo-issues/issue_1798"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/libsass-todo-issues/issue_1798/3.hrx"
#[test]
#[ignore] // unexepected error
fn t3() {
    assert_eq!(
        rsass(
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
