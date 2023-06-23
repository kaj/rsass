//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2289.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2289")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".foo:baz:baz {\
             \n  float: left;\
             \n}\n\
             \n.bar {\
             \n  @extend .foo;\
             \n}\n"),
        ".foo:baz:baz, .bar:baz:baz {\
         \n  float: left;\
         \n}\n"
    );
}
