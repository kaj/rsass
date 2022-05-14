//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_131.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_131")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$foo: bar;\r\
             \n\r\
             \ndiv {\r\
             \n    content: \"foo #{$foo}\"\r\
             \n}"),
        "div {\
         \n  content: \"foo bar\";\
         \n}\n"
    );
}
