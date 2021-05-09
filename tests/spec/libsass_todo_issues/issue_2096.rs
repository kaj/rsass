//! Tests auto-converted from "sass-spec/spec/libsass-todo-issues/issue_2096.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@mixin foo() {\
             \n  @import \"https://foo\";\
             \n}\
             \n@include foo;\n"),
        "@import \"https://foo\";\n"
    );
}
