//! Tests auto-converted from "sass-spec/spec/libsass-todo-issues/issue_2096.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2096")
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
