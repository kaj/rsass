//! Tests auto-converted from "sass-spec/spec/libsass-todo-issues/issue_1798/3.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("a  {\
             \n  content: \"#{ a /*#{\"}*/ }\";\
             \n}\n"),
        "a {\
         \n  content: \"a\";\
         \n}\n"
    );
}
