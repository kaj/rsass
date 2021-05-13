//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_890.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".foo {\
             \n  border: {\
             \n    right: 10px solid /*here is a comment*/;\
             \n  }\
             \n}\n"),
        ".foo {\
         \n  border-right: 10px solid;\
         \n}\n"
    );
}
