//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_344.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_344")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$variable: 1;\n\
             \n$foo: #{$variable}px;\
             \n$bar: #{1}px;\
             \n$baz: \"1px\";\n\
             \ndiv {\
             \n  top: -$foo;\
             \n  top: -$bar;\
             \n  top: -$baz;\
             \n}\n"),
        "div {\
         \n  top: -1px;\
         \n  top: -1px;\
         \n  top: -\"1px\";\
         \n}\n"
    );
}
