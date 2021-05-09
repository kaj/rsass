//! Tests auto-converted from "sass-spec/spec/libsass-todo-issues/issue_2235/empty.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@media all and (min-width: 100px) {\
             \n  @import \"https://example.org\";\
             \n}\n"),
        "@media all and (min-width: 100px) {\
         \n  @import \"https://example.org\";\
         \n}\n"
    );
}
