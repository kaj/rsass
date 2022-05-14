//! Tests auto-converted from "sass-spec/spec/libsass-todo-issues/issue_2235/not-empty.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("not-empty")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@media all and (min-width: 100px) {\
             \n  a { b: c }\
             \n  @import \"https://example.org\";\
             \n}\n"),
        "@media all and (min-width: 100px) {\
         \n  a {\
         \n    b: c;\
         \n  }\
         \n  @import \"https://example.org\";\
         \n}\n"
    );
}
