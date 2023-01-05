//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_506.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_506")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$list: foo bar baz;\
             \n$list--comma: foo, bar, baz;\
             \n$single: foo;\n\
             \ndiv {\
             \n  _list-space: list-separator($list);\
             \n  _list-comma: list-separator($list--comma);\
             \n  _single-item: list-separator($single);\
             \n}"),
        "div {\
         \n  _list-space: space;\
         \n  _list-comma: comma;\
         \n  _single-item: space;\
         \n}\n"
    );
}
