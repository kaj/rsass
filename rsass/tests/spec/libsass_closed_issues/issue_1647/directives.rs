//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1647/directives.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("directives")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@foo #{\"directive\"} {\
             \n  .#{\"foo\"} { #{\"foo-prop\"}: #{\"foo-val\"}; }\
             \n}\n"),
        "@foo directive {\
         \n  .foo {\
         \n    foo-prop: foo-val;\
         \n  }\
         \n}\n"
    );
}
