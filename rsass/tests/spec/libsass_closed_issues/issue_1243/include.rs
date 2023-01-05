//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1243/include.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("include")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@mixin foo() {\
             \n  a { b: c; }\
             \n}\
             \n@include foo\n"),
        "a {\
         \n  b: c;\
         \n}\n"
    );
}
