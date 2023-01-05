//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1632.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1632")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$foo: \\/ !global;\
             \n.foo#{$foo}bar { a: b; }\n"),
        ".foo\\/bar {\
         \n  a: b;\
         \n}\n"
    );
}
